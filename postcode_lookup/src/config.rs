use serde::{Deserialize, Serialize};
use crate::api::postcode::{PostcodesIOClient, TargetLockClient};
use std::default::Default;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub postcodes_io: PostcodesIOClient,
    pub target_lock: TargetLockClient,
}