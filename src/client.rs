use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

use crate::url;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("Network: {}", "source")]
    Network { source: reqwest::Error },

    #[snafu_display("JSON: {}", "source")]
    Json {
        reason: String,
        source: serde_json::Error,
    },

    #[snafu_display("General: [{}] {}", "code", "message")]
    General { code: u16, message: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_createdOn")]
    pub created_on: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

pub struct Client {
    box_id: String,
}

impl Client {
    pub fn new(box_id: &str) -> Client {
        Client {
            box_id: box_id.to_string(),
        }
    }

    pub fn create<T>(&self, data: &T) -> Result<(T, Meta)>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let mut res = client
            .post(&url::of_box(&self.box_id))
            .json(&data)
            .send()
            .context(Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(Network {})?;
            let data: T = serde_json::from_str(&raw).context(Json { reason: "data" })?;
            let meta: Meta = serde_json::from_str(&raw).context(Json { reason: "meta" })?;
            Ok((data, meta))
        } else {
            let err: ErrorMessage = res.json().context(Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn read_all<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut res = reqwest::get(&url::of_box(&self.box_id)).context(Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(Network {})?;
            let data: Vec<T> = serde_json::from_str(&raw).context(Json { reason: "data" })?;
            Ok(data)
        } else {
            let err: ErrorMessage = res.json().context(Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn read<T>(&self, id: &str) -> Result<(T, Meta)>
    where
        T: DeserializeOwned,
    {
        let mut res = reqwest::get(&url::of_record(&self.box_id, id)).context(Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(Network {})?;
            let data: T = serde_json::from_str(&raw).context(Json { reason: "data" })?;
            let meta: Meta = serde_json::from_str(&raw).context(Json { reason: "meta" })?;
            Ok((data, meta))
        } else {
            let err: ErrorMessage = res.json().context(Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn update<T>(&self, id: &str, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let client = reqwest::Client::new();
        let mut res = client
            .put(&url::of_record(&self.box_id, id))
            .json(&data)
            .send()
            .context(Network {})?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err: ErrorMessage = res.json().context(Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let mut res = client
            .delete(&url::of_record(&self.box_id, id))
            .send()
            .context(Network {})?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err: ErrorMessage = res.json().context(Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("01234012340123401234");
        assert_eq!(client.box_id, "01234012340123401234");
    }
}
