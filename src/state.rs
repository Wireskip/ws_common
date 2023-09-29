use axum::extract::{FromRef, State};
use ed25519_dalek::SigningKey;
use std::ops::Deref;

#[derive(Clone)]
pub struct BaseState<T>
where
    T: Send + Sync,
{
    pub crypto: CryptoState,
    pub inner: T,
}

#[derive(Clone)]
pub struct CryptoState {
    pub key: SigningKey,
}

impl<T> FromRef<BaseState<T>> for CryptoState
where
    T: Send + Sync,
{
    fn from_ref(st: &BaseState<T>) -> CryptoState {
        st.crypto.clone()
    }
}

impl<T> Deref for BaseState<T>
where
    T: Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn new<T>(sk: SigningKey, t: T) -> BaseState<T>
where
    T: Send + Sync,
{
    BaseState {
        crypto: CryptoState { key: sk },
        inner: t,
    }
}

pub type Safe<T> = State<BaseState<T>>;
