use std::ops::Deref;
use std::io::{self, Write, Read};
use std::str;



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
use components::atoms::decrypted_text::DecryptedText;

use components::molecules::custom_form::CustomForm;
use components::molecules::encrypt_form::EncryptForm;
use components::molecules::decrypt_form::DecryptForm;

use crate::components::molecules::custom_form::CustomData;
use crate::components::molecules::encrypt_form::EncryptData;
use crate::components::molecules::decrypt_form::DecryptData;

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
struct Helper<'a> {
    secret: &'a openpgp::Cert,
    policy: &'a dyn Policy,
}


impl<'a> DecryptionHelper for Helper<'a> {
    fn decrypt<D>(&mut self,
                  pkesks: &[openpgp::packet::PKESK],
                  _skesks: &[openpgp::packet::SKESK],
                  sym_algo: Option<SymmetricAlgorithm>,
                  mut decrypt: D)
                  -> openpgp::Result<Option<openpgp::Fingerprint>>
        where D: FnMut(SymmetricAlgorithm, &SessionKey) -> bool
    {
        let key = self.secret.keys().unencrypted_secret()
            .with_policy(self.policy, None)
            .for_transport_encryption().next().unwrap().key().clone();

        // The secret key is not encrypted.
        let mut pair = key.into_keypair()?;

        pkesks[0].decrypt(&mut pair, sym_algo)
            .map(|(algo, session_key)| decrypt(algo, &session_key));

        // XXX: In production code, return the Fingerprint of the
        // recipient's Cert here
        Ok(None)
    }
}
impl<'a> VerificationHelper for Helper<'a> {
    fn get_certs(&mut self, _ids: &[openpgp::KeyHandle])
                       -> openpgp::Result<Vec<openpgp::Cert>> {
        // Return public keys for signature verification here.
        Ok(Vec::new())
    }

    fn check(&mut self, _structure: MessageStructure)
             -> openpgp::Result<()> {
        // Implement your signature verification policy here.
        Ok(())
    }
}




/// Decrypts the given message.
fn decrypt(p: &dyn Policy,
           sink: &mut dyn Write, ciphertext: &[u8], recipient: &openpgp::Cert)
           -> openpgp::Result<()> {
    // Make a helper that that feeds the recipient's secret key to the
    // decryptor.
    let helper = Helper {
        secret: recipient,
        policy: p,
    };

    // Now, create a decryptor with a helper using the given Certs.
    let mut decryptor = DecryptorBuilder::from_bytes(ciphertext)?
        .with_policy(p, None, helper)?;

    // Decrypt the data.
    io::copy(&mut decryptor, sink)?;

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
    pub input: String, // string of what was inputted by user
    pub encrypted_input: String, // string encrypted form input
    pub encrypted_input_submitted: String, // string of what was inputted in form of encrypted ciphertext from user
    pub decrypted_output: String, // decrypted output
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

    let decrypt_form_submit = {
        let ciphertext_state = ciphertext_state.clone();
        let user_cert_state = user_cert_state.clone();
        Callback::from(move |data: DecryptData| {
            log!("in decrypt_from_submit callback");
            // convert from ASCII-armor to vec
            let key = user_cert_state.deref().clone().user_cert;
            let p = &P::new();


            // there is a bug right now with encrypted_input_submitted not being parsed correctly into an OpenPGP message.
            let ciphertext = data.input.as_bytes();
            let mut plaintext = Vec::new();
            decrypt(p, &mut plaintext, &ciphertext, &key).unwrap();

            let decrypted_output = str::from_utf8(&plaintext).unwrap();            

            let mut ciphertext_struct = ciphertext_state.deref().clone();
            ciphertext_struct.decrypted_output = decrypted_output.to_string();
            ciphertext_struct.encrypted_input_submitted = data.input;
            ciphertext_state.set(ciphertext_struct);

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
            <DecryptForm onsubmit={decrypt_form_submit}/>
            <DecryptedText/>
            </ContextProvider<CipherText>>
        </ContextProvider<User>>

        </>
    }
}
