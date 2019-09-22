use reqwest;
use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Record {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_createdOn")]
    pub created_on: String,
}

pub type ListOp = Vec<Record>;
pub type CreateOp = Record;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("Network: {}", "source")]
    Network { source: reqwest::Error },

    #[snafu_display("JSON deserialize: {}", "source")]
    Json { source: reqwest::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Client {
    box_id: String,
    endpoint: String,
}

impl Client {
    pub fn new(box_id: &str) -> Client {
        Client {
            box_id: box_id.to_string(),
            endpoint: format!("https://jsonbox.io/{}", box_id),
        }
    }

    pub fn list(&self) -> Result<ListOp> {
        let mut res = reqwest::get(&self.endpoint).context(Network {})?;
        let res: ListOp = res.json().context(Json {})?;
        Ok(res)
    }

    pub fn create(&self) -> Result<CreateOp> {
        let mut data = HashMap::new();
        data.insert("greeting", "hello");

        let client = reqwest::Client::new();
        let mut res = client
            .post(&self.endpoint)
            .json(&data)
            .send()
            .context(Network {})?;
        let res: CreateOp = res.json().context(Json {})?;
        Ok(res)
    }
}
