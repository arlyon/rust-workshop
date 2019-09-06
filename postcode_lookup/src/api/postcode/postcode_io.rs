use hyper::{Client, Request, Body};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use async_trait::async_trait;
use url::Url;

use crate::util::Result;
use super::types::{Postcode, PostcodeClient};
use types::{
    PostcodeResponse, GetPostcodesResponseElement, 
    GetPostcodesResponse, GetPostcodesRequest
};

#[derive(Serialize, Deserialize)]
pub struct PostcodesIOClient {
    url: Url,
}

impl Default for PostcodesIOClient {
    fn default() -> Self {
        PostcodesIOClient {
            url: Url::parse("http://api.postcodes.io/postcodes").expect("This should never fail.")
        }
    }
}

#[async_trait]
impl PostcodeClient for PostcodesIOClient {
    async fn get_postcode(&self, pc: String) -> Result<Postcode> {
        let client = Client::new();
        let res = client.get(format!("{}/{}", self.url, pc).parse()?).await?;
        let body = res.into_body().try_concat().await?;
        let data: Result<PostcodeResponse<_>> = serde_json::from_slice(&body).map_err(|e| e.into());
        data.map(|d| d.result)
    }

    async fn get_postcodes(&self, postcodes: Vec<String>) -> Result<Vec<Postcode>> {
        let client = Client::new();
        
        match postcodes.as_slice() {
            [] => return Ok(vec!()),
            [p] => return self.get_postcode(p.to_owned()).await.map(|p| vec!(p)),
            _ => (),
        };

        let req = Request::builder()
            .uri(self.url.to_string())
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&GetPostcodesRequest {
                    postcodes,
                })?
            ));

        let res = client.request(req?).await?;
        let body = res.into_body().try_concat().await?;
        let data: PostcodeResponse<GetPostcodesResponse> = serde_json::from_slice(&body)?;

        Ok(
            data.result.iter()
                .filter_map(|GetPostcodesResponseElement {result, ..}| result.to_owned())
                .collect()
        )
    }
}

mod types {
    use serde::{Deserialize, Serialize};
    use super::super::Postcode;

    #[derive(Deserialize, Debug)]
    pub struct PostcodeResponse<T> {
        pub status: u16,
        pub result: T,
    }

    #[derive(Deserialize, Debug)]
    pub struct GetPostcodesResponseElement {
        pub query: String,
        pub result: Option<Postcode>,
    }

    pub type GetPostcodesResponse = Vec<GetPostcodesResponseElement>;

    #[derive(Serialize, Debug)]
    pub struct GetPostcodesRequest {
        pub postcodes: Vec<String>
    }
}