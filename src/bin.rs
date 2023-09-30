use crate::{
    b64e::Base64,
    cfg::{Config, ConfigType},
};
use ed25519_dalek::SigningKey;
use log::*;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs};

pub fn common_setup<'de, T>(_phantom: ConfigType<T>) -> Result<Option<Config<T>>, Box<dyn Error>>
where
    T: Serialize + Deserialize<'de> + Default,
{
    env_logger::init();

    let mut p = env::current_exe()?;
    p.pop();

    let main = p.join("config.json5");
    if !main.exists() {
        fs::write(
            main,
            serde_json::to_string::<Config<T>>(&Config::default())?,
        )?
    }

    let local = p.join("config.local.json5");
    if !local.exists() {
        // to write just the correct config subset
        #[derive(Serialize)]
        struct K<'a> {
            keypair: &'a Base64<SigningKey>,
        }

        let kp: SigningKey = SigningKey::generate(&mut OsRng {});

        fs::write(
            p.join("key.pub"),
            serde_json::to_string(&Base64(kp.verifying_key()))?,
        )?;
        fs::write(
            local,
            serde_json::to_string(&K {
                keypair: &Base64(kp),
            })?,
        )?
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "init" {
        // our job here is done
        return Ok(None);
    }

    let mut cfg: Config<T> = config::Config::builder()
        .add_source(config::File::from(p.join("config")))
        .add_source(config::File::from(p.join("config.local")))
        .add_source(config::Environment::with_prefix("WIRESKIP"))
        .build()?
        .try_deserialize()?;

    cfg.root = p;
    info!("Listening on {}", cfg.address);
    Ok(Some(cfg))
}
