use gloo::console::log;
use yew::prelude::*;
use crate::CipherText;
use crate::components::atoms::main_title::{Color, MainTitle};


#[function_component(DecryptedText)]
pub fn key() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));
    let ciphertext_context : Option<CipherText> = use_context::<CipherText>();


    //let encrypted_input = ciphertext_context.clone().unwrap_or_default().encrypted_input_submitted;
    //let encrypted_input_message = "Ciphertext Input for Decryption";

    let decrypted_output = ciphertext_context.clone().unwrap_or_default().decrypted_output;
    let decrypted_output_message = "Decrypted Message From ciphertext";

    if ciphertext_context.clone().unwrap_or_default().encrypted_submit {
    html! {
        <>
            <MainTitle title={decrypted_output_message} color={Color::Ok} on_load={&main_title_load}/>
            <p><pre>{decrypted_output}</pre></p>
        </>
    } 
} else {
    html! {
        <MainTitle title="Encrypted Data Not Created Yet (need to create key)" color={Color::Error} on_load={&main_title_load}/>
    }
}
}
