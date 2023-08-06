use crate::b64e::Base64;
use ed25519_dalek::{ed25519::SignatureBytes, VerifyingKey};
use hyper::{client::HttpConnector, Body, Client};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use strum::{Display, EnumString};
use url::Url;

// COMMON HTTP CLIENT

pub struct HttpClient(pub Client<HttpConnector, Body>);

impl HttpClient {
    pub fn new() -> Self {
        Self(hyper::Client::builder().build(HttpConnector::new()))
    }
}

impl Deref for HttpClient {
    type Target = Client<HttpConnector, Body>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ACCESSKEY

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccesskeyRequest {
    // "type" is a reserved keyword
    #[serde(rename = "type")]
    pub pof_type: String,
    pub quantity: u64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Accesskey {
    pub version: Version,
    pub contract: Contract,
    pub pofs: Vec<Pof>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pof {
    // "type" is a reserved keyword
    #[serde(rename = "type")]
    pub pof_type: String,
    pub nonce: String,
    pub expiration: i64,
    pub signature: Base64<SignatureBytes>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub endpoint: Url,
    pub public_key: Base64<VerifyingKey>,
}

// STATUS

#[derive(Debug, Serialize, Clone)]
pub struct Status {
    pub code: u16,
    #[serde(rename = "description")]
    pub desc: String,
}

// WITHDRAWAL

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithdrawalRequest {
    pub amount: i64,
    #[serde(rename = "type")]
    pub w_type: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Withdrawal {
    pub id: String,
    #[serde(flatten)]
    pub state_data: WithdrawalStateData,
    pub withdrawal_request: WithdrawalRequest,
    pub receipt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithdrawalStateData {
    pub state: WithdrawalState,
    pub state_changed: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Display, EnumString)]
pub enum WithdrawalState {
    Pending,
    Complete,
}
