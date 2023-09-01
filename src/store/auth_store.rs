use yewdux::prelude::*;
use serde::{Serialize, Deserialize};
use yewdux::storage::Area;
use yewdux::store::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool
}

impl Persistent for AuthStore {
    fn key() -> &'static str {
        std::any::type_name::<Self>();
    }

    fn area() -> Area {
        Area::Local
    }
}