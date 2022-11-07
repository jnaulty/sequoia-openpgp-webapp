use crate::components::atoms::main_title::{Color, MainTitle};
use crate::CipherText;
use crate::User;
use gloo::console::log;
use yew::prelude::*;

#[function_component(EncryptedText)]
pub fn key() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    let ciphertext_context: Option<CipherText> = use_context::<CipherText>();
    let user_context: Option<User> = use_context::<User>();

    let encrypted_input = ciphertext_context
        .clone()
        .unwrap_or_default()
        .encrypted_input;
    let encrypted_input_message = "User Encrypted Message";
    if ciphertext_context
        .clone()
        .unwrap_or_default()
        .encrypted_submit
    {
        html! {
            <>
            <MainTitle title={encrypted_input_message} color={Color::Ok} on_load={&main_title_load}/>
            <p><pre>{encrypted_input}</pre></p>
            </>
        }
    } else if user_context.clone().unwrap_or_default().key_submit {
        html! {
            <MainTitle title="Encrypted Data Not Created Yet" color={Color::Error} on_load={&main_title_load}/>
        }
    } else {
        html! {
            <>
            <p>{"Create some keys to explore more content!"}</p>
            </>
        }
    }
}
