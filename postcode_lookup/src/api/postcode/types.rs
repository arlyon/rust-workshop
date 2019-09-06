use serde::{Deserialize};
use async_trait::async_trait;

use crate::util::Result;
use super::serde::deserialize_number_from_string;

#[derive(Deserialize, Debug, Clone)]
pub struct Postcode {
    #[serde(alias="post_code")]
    pub postcode: String,
    pub country: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub longitude: f32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub latitude: f32,
}

impl std::fmt::Display for Postcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{: <8} : ({:.5}, {:.5})", self.postcode, self.latitude, self.longitude)
    }
}

#[async_trait]
pub trait PostcodeClient {
    async fn get_postcode(&self, pc: String) -> Result<Postcode>;
    async fn get_postcodes(&self, postcodes: Vec<String>) -> Result<Vec<Postcode>>;
}