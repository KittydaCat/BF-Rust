mod BF;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement};
use crate::BF::BF::{BFError, BFInterpreter};

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

    fn create(_ctx: &Context<Self>) -> Self {

        fn input() -> Result<String, BFError>{
            todo!()
        }

        fn output(str: String) -> Result<(), BFError>{
            todo!()
        }

        Self {

            interp: BFInterpreter{ array: vec!(0_u32), array_pointer: 0,
                program: "+++[->+<]".to_string(), program_index: 0, input, output
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
                    link.callback(move |_| BFMsg::HandleError(err.clone())).emit(1)
                }
            }
            BFMsg::ExecuteAll => {

                if let Err(err) = self.interp.run(){
                    link.callback(move |_| BFMsg::HandleError(err.clone())).emit(1)
                }
            }

            BFMsg::UpdateOutput(char) => {self.output.push(char)}

            BFMsg::HandleError(err) => {

                match err {

                    _ => {log(&err.to_string())}
                }

            }
        };
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        html!{
            <div class="BFInterpreter">

            <div class="BFArray"> {} </div>

            <lable>{"Program:"}</lable><br/>
            <textarea rows="10" cols="30" placeholder="Your BF program"
            oninput={link.callback(|_| BFMsg::ProgramUpdated)}/><br/>

            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteOne)}>{">"}</button>
            <button type="button" onclick={link.callback(|_| BFMsg::ExecuteAll)}>{">>"}</button><br/>

            <input placeholder="Input" id="input"/><br/>

            // prepare to replace with an indicator on the editable text

            <p>{"Current program:"}</p>
            <p>{ (self.interp.program).clone() }</p>

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