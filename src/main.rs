mod bf;
mod b_unf;

use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::bf::{BFError, BFInterpreter};
use web_sys::{HtmlDivElement, HtmlInputElement};
use htmlescape::encode_minimal;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub enum BFMsg {
    ProgramUpdated,
    LabelProgram,   // to func?
    ExecuteOne,     // combine together?
    ExecuteAll,
    UpdateOutput(char),
    HandleError(BFError), // to func?
    Reset,
    GetInput,
}

#[derive(Default)]
struct RefList{
    edit_div: NodeRef,
    input: NodeRef,
}

struct BFDisplay {
    interp: BFInterpreter,
    output: String,
    input: Rc<RefCell<Option<char>>>,
    refs: RefList,
}

impl Component for BFDisplay{

    type Message = BFMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let input_val: Rc<RefCell<Option<char>>> = Rc::from(RefCell::from(None));

        let input_link = ctx.link().clone();
        let input_val2 = Rc::clone(&input_val);

        let input = move || -> Result<char, BFError> {

            input_link.send_message(BFMsg::GetInput);

            if let Some(input) = *input_val2.borrow(){
                Ok(input)
            } else {
                Err(BFError::InputFailed)
            }
        };

        let output_link = ctx.link().clone();

        let output = move |x| {
            // link.callback(|char| BFMsg::UpdateOutput(char)).emit(x);Ok(())
            output_link.send_message(BFMsg::UpdateOutput(x));
            Ok(())
        };

        Self {

            //+++++[->++++++<]>+++. --> !
            interp: BFInterpreter::new(String::new(), Box::new(input), Box::new(output)),

            output: "".to_string(),
            input: input_val,
            refs: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        let link = ctx.link();

        match msg{

            BFMsg::ProgramUpdated => {
                let program = &self.refs.edit_div;
                let x = program.cast::<HtmlDivElement>().expect("").inner_text();
                self.interp.program = x;
                // link.callback(move |_| BFMsg::Reset).emit(());
            }
            BFMsg::LabelProgram => {

                let program_input = &self.refs.edit_div.cast::<HtmlDivElement>().unwrap();

                program_input.set_inner_html(&*self.interp.program.chars().enumerate().map(|(i, x)|{

                    let escaped_str = encode_minimal(&x.to_string());

                    if i == self.interp.program_index{
                        format!(r#"<span style="color: red">{escaped_str}</span>"#)}

                    else { escaped_str }}).collect::<String>());
            }
            BFMsg::ExecuteOne => {

                if let Err(err) = self.interp.exec_one(){
                    link.send_message(BFMsg::HandleError(err));
                }

                link.send_message(BFMsg::LabelProgram);
            }
            BFMsg::ExecuteAll => {

                if let Err(err) = self.interp.run(){
                    link.send_message(BFMsg::HandleError(err));
                }

                link.send_message(BFMsg::LabelProgram);
            }

            BFMsg::UpdateOutput(char) => {self.output.push(char)}

            BFMsg::HandleError(err) => {

                match err {

                    _ => {log(&format!("{:?}", err))}
                }

            }
            BFMsg::Reset => {
                self.interp.program_index = 0;
                self.interp.array = vec![0_u32];
                self.interp.array_pointer = 0;
                self.output.clear();
                link.send_message(BFMsg::LabelProgram);
            }
            BFMsg::GetInput => {

                let input = (&self.refs.input).cast::<HtmlInputElement>().unwrap();

                input.value()
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        // link.callback(|_| BFMsg::LabelProgram).emit(()); ???
        // TODO: Get default to be annotated on load

        html!{
            <div class="BFInterpreter">

            <table class="BFArray">
                <tr> {self.interp.array.iter().enumerate().map(|(i,x)| html!{
                if i == self.interp.array_pointer{<td style="color: red;">{x.to_string()} </td>}
                else{<td>{x.to_string()} </td>}}).collect::<Html>()} </tr>
            </table>

            <lable>{"Program:"}</lable><br/>
            // <textarea rows="10" cols="30" placeholder="Your BF program" ref={&self.refs.textarea}
            // oninput={link.callback(|_| BFMsg::ProgramUpdated)}/><br/>

            <div contenteditable="true" style="border:1px solid black;" ref={&self.refs.edit_div}
            oninput={link.callback(|_| BFMsg::ProgramUpdated)}/>
            // style="height: 200px"

            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteOne)}>{">"}</button>
            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteAll)}>{">>"}</button>
            <button type="button" onclick={link.callback(|_| BFMsg::Reset)}>{"Reset"}</button><br/>

            <input placeholder="Input" ref={&self.refs.input}/><br/>

            // <p>{"Current program:"}</p>
            // <p> {self.interp.program.chars().take(self.interp.program_index).collect::<String>()}
            //
            //     if self.interp.program.len() > self.interp.program_index{
            //
            //         <span style="color: red;">{
            //         self.interp.program.chars().nth(self.interp.program_index).expect("Program index is invalid")}
            //         </span>}
            //
            // {self.interp.program.chars().skip(self.interp.program_index+1).collect::<String>()} </p>

            <p>{"Output:"}</p>
            <p>{self.output.clone()}</p>


            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html{
    html!{
        <BFDisplay/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}