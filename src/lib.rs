use std::ops::Deref;

use openpgp::cert::prelude::*;
use openpgp::serialize::SerializeInto;
use sequoia_openpgp as openpgp;

use gloo::console::log;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew::ContextProvider;

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::atoms::key::Key;
use components::molecules::custom_form::CustomForm;

use crate::components::molecules::custom_form::Data;

fn generate(userid: String) -> openpgp::Result<openpgp::Cert> {
    let (cert, _revocation) = CertBuilder::new()
        .add_userid(userid)
        .add_transport_encryption_subkey()
        .generate()?;

    // Save the revocation certificate somewhere.

    Ok(cert)
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub userid: String,
    pub key: String,
    pub key_submit: bool
}

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let userid = format!("{} <{}>", data.username, data.email);
            let cert = generate(userid).unwrap();
            let key = String::from_utf8(cert.armored().to_vec().unwrap()).unwrap();

            let mut user = user_state.deref().clone();
            user.userid = format!("{} <{}>", data.username, data.email);
            user.key = key;
            user.key_submit = true;
            user_state.set(user);
            //log!(key);
        })
    };

    // start of html template
    html! {
        <>
            // Global Styles can be applied with <Global /> component.
            // from: https://github.com/futursolo/stylist-rs/blob/master/examples/yew-theme-context/src/main.rs
           <Global css={css!(
               r#"
                   html, body {
                       font-family: sans-serif;
                       padding: 0;
                       margin: 0;
                       display: flex;
                       justify-content: center;
                       align-items: center;
                       min-height: 100vh;
                       flex-direction: column;
                       background-color: ${bg};
                       color: ${ft_color};
                   }
               "#,
               bg = "black",
               ft_color = "white",
           )} />
        <MainTitle title="Sequoia OpenPGP Explorer" color={Color::Normal} on_load={&main_title_load}/> 
        <MainTitle title="hello there, create a userid to generate some keys" color={Color::Ok} on_load={&main_title_load}/>
        <ContextProvider<User> context={user_state.deref().clone()}>
            <CustomForm onsubmit={custom_form_submit}/>
            <Key/>           
        </ContextProvider<User>>

        </>
    }
}
