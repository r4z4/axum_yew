use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use yewdux::storage::Area;
use yewdux::store::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub is_authenticated: bool,
}

pub fn set_username(username: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.username = Some(username);
    })
}

pub fn set_password(password: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.password = Some(password);
    })
}

pub fn set_token(token: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.token = Some(token);
    })
}
