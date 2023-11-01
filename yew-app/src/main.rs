use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| String::new());

    let set_to_joe = {
        let state = state.clone();
        Callback::from(move |_| state.set(String::from("Joe")))
    };

    let set_to_contents = {
        let state = state.clone();
        Callback::from(move |e: yew::events::MouseEvent|{
            let mut str = String::from("Click");
            println!("Click started");
            str = e.range_parent().unwrap().first_child().unwrap().node_name();
            println!("Successful click");
            state.set(String::from(str))
        })
    };

    html! {
        <>
        <h1>{ "BF" }</h1>

        <form>

            <lable>{"Program:"}</lable><br/>

            <textarea rows="10" cols="30" placeholder = "Your BF program">{ "+++[->+<]" }</textarea><br/>

            <input type="button" onclick={set_to_contents} value="Submit"/>
            <input type="reset" value="Reset"/>

        </form><br/>

        <button type="button" onclick={set_to_joe}>{">"}</button>
        <button type="button">{">>"}</button><br/>
        <input id="input"/><br/>

        <p>{"Test output:"}</p>
        <p>{ (*state).clone() }</p>
        </>
    }
}

fn main() {
    let _output = String::new();
    yew::Renderer::<App>::new().render();
}