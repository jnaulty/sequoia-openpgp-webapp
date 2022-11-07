use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::CipherText;
use crate::User;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct EncryptData {
    pub input: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<EncryptData>,
}

#[function_component(EncryptForm)]
pub fn encrypt_form(props: &Props) -> Html {
    let user_context: Option<User> = use_context::<User>();
    let _state = use_state(|| EncryptData::default());
    let _cloned_state = _state.clone();

    let ciphertext_context = use_context::<CipherText>();

    let _cloned_state = _state.clone();
    let input_changed = Callback::from(move |input| {
        _cloned_state.set(EncryptData {
            input,
            .._cloned_state.deref().clone()
        });
    });

    let form_onsubmit = props.onsubmit.clone();
    let _cloned_state = _state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        // do not submit form when submit is clicked
        event.prevent_default();
        let data = _cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    if user_context.clone().unwrap_or_default().key_submit {
        html! {
            <form onsubmit={onsubmit}>
                <TextInput name="input" handle_onchange={input_changed} />
                <CustomButton label="Submit" />
                <p>{"input:"}{ciphertext_context.clone().unwrap_or_default().input}</p>
            </form>

        }
    } else {
        html! {
            <p></p>
        }
    }
}
