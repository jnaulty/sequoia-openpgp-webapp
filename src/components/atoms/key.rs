use yew::prelude::*;
use crate::User;


#[function_component(Key)]
pub fn key() -> Html {
    let user_context: Option<User> = use_context::<User>();
    let key = user_context.clone().unwrap_or_default().key;
    let key_message = "User Private PGP Key";
    if user_context.clone().unwrap_or_default().key_submit {
    html! {
        <>
        <h2>{key_message}</h2>
        <p><pre>{key}</pre></p>
        </>
    } 
} else {
    html! {
        <h2>{"Key Not Created Yet"}</h2>
    }
}
}
