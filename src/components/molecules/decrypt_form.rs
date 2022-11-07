use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_area_input::TextAreaInput;
use crate::CipherText;
use crate::User;
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
    let _user_context: Option<User> = use_context::<User>();
    let _state = use_state(|| DecryptData::default());
    let _cloned_state = _state.clone();

    let ciphertext_context = use_context::<CipherText>();

    let _cloned_state = _state.clone();
    let input_changed = Callback::from(move |input| {
        _cloned_state.set(DecryptData {
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

    if ciphertext_context
        .clone()
        .unwrap_or_default()
        .encrypted_submit
    {
        html! {
            <form onsubmit={onsubmit}>
                <TextAreaInput name="-----BEGIN PGP MESSAGE-----" handle_onchange={input_changed} />
                <CustomButton label="Submit" />
                <p>{"ascii--armored input:"}
                    <pre>{ciphertext_context.clone().unwrap_or_default().encrypted_input_submitted}</pre>
                </p>
            </form>

        }
    } else {
        html! {
            <p></p>
        }
    }
}
