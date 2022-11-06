use gloo::console::log;
use yew::prelude::*;
use crate::User;
use crate::components::atoms::main_title::{Color, MainTitle};


#[function_component(Key)]
pub fn key() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));
    let user_context: Option<User> = use_context::<User>();
    let key = user_context.clone().unwrap_or_default().key;
    let key_message = "User Private PGP Key";
    if user_context.clone().unwrap_or_default().key_submit {
    html! {
        <>
        <MainTitle title={key_message} color={Color::Ok} on_load={&main_title_load}/>
        <p><pre>{key}</pre></p>
        </>
    } 
} else {
    html! {
        <MainTitle title="Key Not Created Yet" color={Color::Error} on_load={&main_title_load}/>
    }
}
}
