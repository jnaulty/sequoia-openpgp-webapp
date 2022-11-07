# Sequoia OpenPGP Demo on WASM using Yew

`sequoia-openpgp-webapp` is a playground project I created to explore pgp on the web, in rust.

My objective was to create a web app written completely in rust that could show off the `sequoia-openpgp` project.
I've had exposure to Javascript web frameworks like Angular, React, and Vue, but wanted to stay completely within the rust ecosystem.
I'd like to get more comfortable with the `sequoia-openpgp` library. Eventually, I'd like to use it on the [betrusted.io](https://betrusted.io) `precursor` device.

Here's the [Demo.](https://jnaulty.github.io/sequoia-openpgp-webapp/) (rendered via `gh-pages`).

## Underlying Tech

This web app is built with two primary technologies, [yew.rs](https://yew.rs) and [sequoia-openpgp](https://sequoia-pgp.org/)

### Why Yew?

I wanted to work on a project that was using Rust + WebAssembly--[Mozilla Developer Docs](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#rust_and_webassembly_use_cases) pointed me to Yew:

```
There are two main use cases for Rust and WebAssembly:

- Build an entire application — an entire web app based in Rust.
- Build a part of an application — using Rust in an existing JavaScript frontend.

For now, the Rust team is focusing on the latter case, and so that's what we cover here. 
For the former case, check out projects like yew.
```

[Yew](https://github.com/yewstack/yew) 
> Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.

My pet peeve with web development is having to deal with javascript. I'm so happy to see frameworks like `Yew` developing in the rust ecosystem!

### Sequoia-Openpgp

The Sequoia Project ([gitlab](https://gitlab.com/sequoia-pgp/sequoia)) is a 
> cool new OpenPGP implementation.  It consists of several crates, providing both a low-level and a high-level API for dealing with OpenPGP data.

Some cool features about this project is that it's a very complete implementation of OpenPGP standard in rust. This project is much more friendly for development than building something on top of the `gpg` ecosystem.

#### OpenPGP Web App Integration Features

- [] Go through [Sequoia OpenPGP Examples](https://gitlab.com/sequoia-pgp/sequoia/-/tree/main/openpgp/examples)
    - [] [decrypt.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/decrypt-with.rs)
    - [] [encrypt-for.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/encrypt-for.rs)
    - [] [generate-encrypt-decrypt.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/generate-encrypt-decrypt.rs)
    - [] [generate-group-key.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/generate-group-key.rs)
    - [] [generate-sign-verify.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/generate-sign-verify.rs)
    - [] [notarize.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/notarize.rs)
    - [] [pad.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/pad.rs)
    - [] [replly-encrypted.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/reply-encrypted.rs)
    - [] [sign-detached.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/sign-detached.rs)
    - [] [sign.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/sign.rs)
    - [] [statistics.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/statistics.rs)
    - [] [supported-algorithms.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/supported-algorithms.rs)
    - [] [web-of-trust.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/web-of-trust.rs)
    - [] [wrap-literal.rs](https://gitlab.com/sequoia-pgp/sequoia/-/blob/main/openpgp/examples/wrap-literal.rs)


## Miscellaneous

### Deps.Dev

#### Yew

- [deps.dev - yew v0.19.3](https://deps.dev/cargo/yew/0.19.3)

#### sequoia-openpgp

- [deps.dev - sequoia-openpgp v1.10.0](https://deps.dev/cargo/sequoia_openpgp)


### Thanks

- to [antonleviathan/pgp-encrypt-files-static-site](https://github.com/antonleviathan/pgp-encrypt-files-static-site). I was inspired to try out a similar project in rust after seeing this work!
- to the [hashbang](https://hashbang.sh) group--it's cool learning from you all!
- to github for free hosting!
