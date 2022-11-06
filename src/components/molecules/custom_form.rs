
use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::User;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub email: String,
    pub key: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());
    let cloned_state = state.clone();

    let user_context = use_context::<User>();

    let username_changed = Callback::from(move |username| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let email_changed = Callback::from(move |email| {
        cloned_state.set(Data {
            email,
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

    html! {
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_changed} />
            <TextInput name="email" handle_onchange={email_changed} />
            <CustomButton label="Submit" />
            <p>{"userid:"}{user_context.clone().unwrap_or_default().userid}</p>
        </form>

    }
}
