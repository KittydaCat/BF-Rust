mod bf;
mod b_unf;


use std::any::Any;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement};
use crate::bf::{BFError, BFInterpreter};


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub enum BFMsg {
    ProgramUpdated,
    ExecuteOne,
    ExecuteAll,
    UpdateOutput(char),
    HandleError(BFError),
    Reset,
}

struct BFDisplay{

    interp: BFInterpreter,
    output: String,
    error: bool,
    // error_display: String,
}

impl Component for BFDisplay{

    type Message = BFMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let link = ctx.link().clone();

        fn input() -> Result<char, BFError>{
            todo!()
        }

        let output = move |x| {link.callback(|char| BFMsg::UpdateOutput(char)).emit(x); Ok(())};

        Self {

            interp: BFInterpreter{ array: vec!(0_u32), array_pointer: 0,
                program: "+++++[->++++++<]>+++.".to_string(), program_index: 0,
                input: Box::new(input), output: Box::new(output)
            },

            error: false,

            output: String::new(),

            // error_display: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        let link = ctx.link();

        match msg{

            BFMsg::ProgramUpdated => { todo!() }
            BFMsg::ExecuteOne => {

                if let Err(err) = self.interp.exec_one(){
                    link.callback(move |_| BFMsg::HandleError(err.clone())).emit(());
                    log("ExecuteOneError")
                }
                log("ExecuteOne")
            }
            BFMsg::ExecuteAll => {

                if self.interp.program.len() == self.interp.program_index{self.interp.reset()}

                if let Err(err) = self.interp.run(){
                    link.callback(move |_| BFMsg::HandleError(err.clone())).emit(())
                }
            }

            BFMsg::UpdateOutput(char) => {self.output.push(char)}

            BFMsg::HandleError(err) => {

                match err {

                    _ => {log(&format!("{:?}", err))}
                }

            }
            BFMsg::Reset => {
                self.interp.reset();
                self.output.clear();}
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        html!{
            <div class="BFInterpreter">

            <table class="BFArray">
                <tr> {self.interp.array.iter().enumerate().map(|(i,x)| html!{
                if i == self.interp.array_pointer{<td style="color: red;">{x.to_string()} </td>}
                else{<td>{x.to_string()} </td>}}).collect::<Html>()} </tr>
            </table>

            <lable>{"Program:"}</lable><br/>
            <textarea rows="10" cols="30" placeholder="Your BF program"
            oninput={link.callback(|_| BFMsg::ProgramUpdated)}/><br/>

            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteOne)}>{">"}</button>
            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteAll)}>{">>"}</button>
            <button type="button" onclick={link.callback(|_| BFMsg::Reset)}>{"Reset"}</button><br/>

            <input placeholder="Input" id="input"/><br/>

            // prepare to replace with an indicator on the editable text

            <p>{"Current program:"}</p>
            <p> {self.interp.program.chars().take(self.interp.program_index).collect::<String>()}

                if self.interp.program.len() > self.interp.program_index{
                    <span style="color: red;">{
                        self.interp.program.chars().nth(self.interp.program_index).expect("Program index is invalid")}
                    </span>}

            {self.interp.program.chars().skip(self.interp.program_index+1).collect::<String>()} </p>

            <p>{"Output:"}</p>
            <p>{ self.output.clone()}</p>


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