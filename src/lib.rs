#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate maplit;
extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tungstenite;
extern crate url;

pub mod errors;
mod transport;
mod util;

pub mod model;

pub mod account;
pub mod client;
pub mod general;
pub mod market;
pub mod userstream;
pub mod websockets;

pub use client::{Account, Binance, General, Market, UserStream};
