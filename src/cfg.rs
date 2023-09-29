use crate::b64e::Base64;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use std::{env, marker::PhantomData, path::PathBuf};

// phantom type specifier for deserialization
pub struct ConfigType<T>(PhantomData<T>);

impl<T> ConfigType<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config<T> {
    pub address: String,
    pub keypair: Option<Base64<SigningKey>>,
    #[serde(skip)]
    pub root: PathBuf,
    #[serde(flatten)]
    pub etc: T,
}

impl<T> Default for Config<T>
where
    T: Default,
{
    fn default() -> Self {
        let mut p = env::current_exe().unwrap();
        p.pop();

        Self {
            address: "127.0.0.1:8080".to_string(),
            root: p,
            keypair: None,
            etc: Default::default(),
        }
    }
}
