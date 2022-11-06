use std::ops::Deref;
use std::io::{self, Write};



use openpgp::serialize::SerializeInto;
use sequoia_openpgp as openpgp;
use crate::openpgp::cert::prelude::*;
use crate::openpgp::crypto::SessionKey;
use crate::openpgp::types::SymmetricAlgorithm;
use openpgp::types::DataFormat;
use crate::openpgp::serialize::stream::*;
use crate::openpgp::parse::{Parse, stream::*};
use crate::openpgp::policy::Policy;
use crate::openpgp::policy::StandardPolicy as P;


use gloo::console::log;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew::ContextProvider;

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::atoms::key::Key;
use components::atoms::encrypted_text::EncryptedText;
use components::molecules::custom_form::CustomForm;
use components::molecules::encrypt_form::EncryptForm;

use crate::components::molecules::custom_form::CustomData;
use crate::components::molecules::encrypt_form::EncryptData;

fn generate(userid: String) -> openpgp::Result<openpgp::Cert> {
    let (cert, _revocation) = CertBuilder::new()
        .add_userid(userid)
        .add_transport_encryption_subkey()
        .generate()?;

    // Save the revocation certificate somewhere.

    Ok(cert)
}

/// Encrypts the given message.
fn encrypt(p: &dyn Policy, sink: &mut (dyn Write + Send + Sync),
           plaintext: &str, recipient: &openpgp::Cert)
    -> openpgp::Result<()>
{
    let recipients =
        recipient.keys().with_policy(p, None).supported().alive().revoked(false)
        .for_transport_encryption();

    // Start streaming an OpenPGP message.
    let message = Message::new(sink);
    let message = Armorer::new(message).build()?;

    // We want to encrypt a literal data packet.
    let message = Encryptor::for_recipients(message, recipients)
        .build()?;

    // Emit a literal data packet.
    let mut message = LiteralWriter::new(message).format(DataFormat::Text).build()?;

    // Encrypt the data.
    message.write_all(plaintext.as_bytes())?;


    // Finalize the OpenPGP message to make sure that all data is
    // written.
    message.finalize()?;

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub userid: String,
    pub key: String,
    pub priv_key: String,
    pub key_submit: bool
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct CipherText {
    pub input: String,
    pub encrypted_input: String,
    pub encrypted_submit: bool,
}

#[derive(Clone, PartialEq)]
pub struct UserCert {
    pub user_cert: Cert,
}

#[styled_component(App)]
pub fn app() -> Html {
    // create user state context
    let user_state = use_state(User::default);
    // create ciphertext state context
    let ciphertext_state = use_state(CipherText::default);

    // initialize cert (workaround for no Default implemented for Cert)
    let (initial_cert, _revocation_crt) = CertBuilder::new().generate().unwrap();
    // create user cert state context
    let user_cert_state = use_state(|| {UserCert { user_cert: initial_cert}});

    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = {
        let user_state = user_state.clone();
        let user_cert_state = user_cert_state.clone();

        Callback::from(move |data: CustomData| {
            let userid = format!("{} <{}>", data.username, data.email);
            let user_cert = generate(userid).unwrap();
            let key = String::from_utf8(user_cert.armored().to_vec().unwrap()).unwrap();
            let priv_key = String::from_utf8(user_cert.as_tsk().armored().to_vec().unwrap()).unwrap();

            let mut user = user_state.deref().clone();
            user.userid = format!("{} <{}>", data.username, data.email);
            user.key = key;
            user.priv_key = priv_key;
            user.key_submit = true;
            user_state.set(user);

            let mut user_cert_x = user_cert_state.deref().clone();
            user_cert_x.user_cert = user_cert;
            user_cert_state.set(user_cert_x);

            //log!(key);
        })
    };

    let encrypt_form_submit = {
        let ciphertext_state = ciphertext_state.clone();
        let user_cert_state = user_cert_state.clone();
        Callback::from(move |data: EncryptData| {
            log!("in encrypt_from_submit callback");
            let p = &P::new();
            let input = data.input;
            let key = user_cert_state.deref().clone().user_cert;
            let mut ciphertext = Vec::new();
            let message = encrypt(p, &mut ciphertext, &input, &key).unwrap();

            let mut ciphertext_struct = ciphertext_state.deref().clone();
            ciphertext_struct.input = input;
            ciphertext_struct.encrypted_input = String::from_utf8(ciphertext).unwrap();
            ciphertext_struct.encrypted_submit = true;
            ciphertext_state.set(ciphertext_struct);
            
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
            <ContextProvider<CipherText> context={ciphertext_state.deref().clone()}>
            <EncryptForm onsubmit={encrypt_form_submit}/>
            <EncryptedText/>
            // <DecryptForm/>
            </ContextProvider<CipherText>>
        </ContextProvider<User>>

        </>
    }
}
