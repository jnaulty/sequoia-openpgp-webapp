
use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::User;
use crate::CipherText;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct DecryptData {
    pub input: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<DecryptData>,
}

#[function_component(DecryptForm)]
pub fn encrypt_form(props: &Props) -> Html {

    let user_context: Option<User> = use_context::<User>();
    let state = use_state(|| DecryptData::default());
    let cloned_state = state.clone();

    let ciphertext_context = use_context::<CipherText>();


    let cloned_state = state.clone();
    let input_changed = Callback::from(move |input| {
        cloned_state.set(DecryptData {
            input,
            ..cloned_state.deref().clone()
        });
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        // do not submit form when submit is clicked
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    if ciphertext_context.clone().unwrap_or_default().encrypted_submit {
        html! {
            <form onsubmit={onsubmit}>
                <TextInput name="ASCII-Armored PGP Message" handle_onchange={input_changed} />
                <CustomButton label="Submit" />
            </form>

        }
    } else {
        html! {
            <p></p>
        }
    }
}
