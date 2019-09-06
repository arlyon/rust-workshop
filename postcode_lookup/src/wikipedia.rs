use hyper::{Client};
use hyper_tls::HttpsConnector;
use serde::{Deserialize};
use futures::TryStreamExt;

use crate::api::postcode::Postcode;
use crate::util::Result;

#[derive(Deserialize, Debug)]
struct Response {
    query: ResponseQuery,
}

#[derive(Deserialize, Debug)]
struct ResponseQuery {
    geosearch: Vec<NearbyArticle>,
}

#[derive(Deserialize, Debug)]
pub struct NearbyArticle {
    #[serde(rename="pageid")]
    id: u64,

    pub title: String,

    #[serde(rename="lat")]
    pub latitude: f32,

    #[serde(rename="lon")]
    pub longitude: f32,
    
    #[serde(rename="dist")]
    pub distance: f32,
}

impl std::fmt::Display for NearbyArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} : ({:.5}, {:.5})", self.title, self.latitude, self.longitude)
    }
}

pub struct WikipediaClient {

}

impl WikipediaClient {
    pub async fn get_nearby(self, p: Postcode) -> Result<Vec<NearbyArticle>> {
        let https = HttpsConnector::new().unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let query_url = format!(
            "https://en.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord={lat}%7C{long}&gsradius=10000&gslimit={limit}&format=json", 
            lat=p.latitude, long=p.longitude, limit=10
        );

        let res = client.get(query_url.parse()?).await?;
        let body = res.into_body().try_concat().await?;
        let data: Result<Response> = serde_json::from_slice(&body).map_err(|e| e.into());
        data.map(|d| d.query.geosearch)
    }
}