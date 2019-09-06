use std::default::Default;

use hyper::{Client};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use futures::{TryStreamExt};
use async_trait::async_trait;
use url::Url;

use super::{PostcodeClient, Postcode};
use crate::util;

#[derive(Serialize, Deserialize)]
pub struct TargetLockClient {
    url: Url,

    #[serde(default)] 
    api_key: Option<String>,
}

impl Default for TargetLockClient {
    fn default() -> Self {
        TargetLockClient {
            url: Url::parse("https://api.targetlock.io/v1/post-code").expect("This should always be valid."),
            api_key: None,
        }
    }
}

#[async_trait]
impl PostcodeClient for TargetLockClient {
    async fn get_postcode(&self, pc: String) -> util::Result<Postcode> {
        let https = HttpsConnector::new().unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let res = client.get(format!("{}/{}", self.url, pc).parse()?).await?;
        println!("{}/{}", self.url, pc);
        let body = res.into_body().try_concat().await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }

    async fn get_postcodes(&self, postcodes: Vec<String>) -> util::Result<Vec<Postcode>> {
        // todo(arlyon) use futures::future::join_all to run concurrently
        let mut out: Vec<Postcode> = vec!();
        for pc in postcodes {
            out.push(self.get_postcode(pc).await?);
        }
        Ok(out)
    }
}
