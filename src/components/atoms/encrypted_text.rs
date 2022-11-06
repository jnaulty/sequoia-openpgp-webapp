use gloo::console::log;
use yew::prelude::*;
use crate::CipherText;
use crate::components::atoms::main_title::{Color, MainTitle};


#[function_component(EncryptedText)]
pub fn key() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));
    let ciphertext_context : Option<CipherText> = use_context::<CipherText>();


    let encrypted_input = ciphertext_context.clone().unwrap_or_default().encrypted_input;
    let encrypted_input_message = "User Encrypted Message";
    if ciphertext_context.clone().unwrap_or_default().encrypted_submit {
    html! {
        <>
        <MainTitle title={encrypted_input_message} color={Color::Ok} on_load={&main_title_load}/>
        <p><pre>{encrypted_input}</pre></p>
        </>
    } 
} else {
    html! {
        <MainTitle title="Key Not Created Yet" color={Color::Error} on_load={&main_title_load}/>
    }
}
}
