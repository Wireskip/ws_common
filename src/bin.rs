use crate::{
    b64e::Base64,
    cfg::{Config, ConfigType},
};
use ed25519_dalek::SigningKey;
use log::*;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug)]
pub struct E {
    pub exit_code: u8,
    pub desc: &'static str,
}

pub fn common_setup<'de, T>(_phantom: ConfigType<T>) -> Result<Config<T>, E>
where
    T: Serialize + Deserialize<'de> + Default,
{
    env_logger::init();

    let mut p = env::current_exe().unwrap();
    p.pop();

    let main = p.join("config.json5");
    if !main.exists() {
        fs::write(
            main,
            serde_json::to_string::<Config<T>>(&Config::default()).unwrap(),
        )
        .expect("Unable to write config file");
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
            serde_json::to_string(&Base64(kp.verifying_key())).unwrap(),
        )
        .expect("Unable to write pubkey compat file");
        fs::write(
            local,
            serde_json::to_string(&K {
                keypair: &Base64(kp),
            })
            .unwrap(),
        )
        .expect("Unable to write local config file");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "init" {
        // our job here is done
        return Err(E {
            exit_code: 0,
            desc: "init done",
        });
    }

    let mut cfg: Config<T> = config::Config::builder()
        .add_source(config::File::from(p.join("config")))
        .add_source(config::File::from(p.join("config.local")))
        .add_source(config::Environment::with_prefix("WIRESKIP"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    cfg.root = p;
    info!("Listening on {}", cfg.address);
    Ok(cfg)
}
