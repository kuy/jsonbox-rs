use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::ResultExt;

use crate::error::{self, Error, Result};
use crate::query_builder::QueryBuilder;
use crate::url;

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
    base_url: String,
    box_id: String,
}

impl Client {
    pub fn new(box_id: &str) -> Client {
        Client {
            base_url: url::BASE_URL.to_string(),
            box_id: box_id.to_string(),
        }
    }

    pub fn with_base_url(box_id: &str, base_url: &str) -> Client {
        Client {
            base_url: base_url.to_string(),
            box_id: box_id.to_string(),
        }
    }

    pub fn create<T>(&self, data: &T) -> Result<(T, Meta)>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let mut res = client
            .post(&url::of_box(&self.base_url, &self.box_id))
            .json(&data)
            .send()
            .context(error::Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(error::Network {})?;
            let data: T = serde_json::from_str(&raw).context(error::Json { reason: "data" })?;
            let meta: Meta = serde_json::from_str(&raw).context(error::Json { reason: "meta" })?;
            Ok((data, meta))
        } else {
            let err: ErrorMessage = res.json().context(error::Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn read(&self) -> QueryBuilder {
        QueryBuilder::new(self)
    }

    pub(crate) fn read_by_id<T>(&self, id: &str) -> Result<(T, Meta)>
    where
        T: DeserializeOwned,
    {
        let url = url::of_record(&self.base_url, &self.box_id, id);
        let mut res = reqwest::get(&url).context(error::Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(error::Network {})?;
            let data: T = serde_json::from_str(&raw).context(error::Json { reason: "data" })?;
            let meta: Meta = serde_json::from_str(&raw).context(error::Json { reason: "meta" })?;
            Ok((data, meta))
        } else {
            let err: ErrorMessage = res.json().context(error::Network {})?;
            Err(Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub(crate) fn read_by_query<T>(&self, query: &QueryBuilder) -> Result<Vec<(T, Meta)>>
    where
        T: DeserializeOwned,
    {
        let url = &url::of_query(&self.base_url, &self.box_id, &query.to_string());
        let mut res = reqwest::get(url).context(error::Network {})?;
        if res.status().is_success() {
            let raw = res.text().context(error::Network {})?;
            let data: Vec<T> =
                serde_json::from_str(&raw).context(error::Json { reason: "data" })?;
            let meta: Vec<Meta> =
                serde_json::from_str(&raw).context(error::Json { reason: "meta" })?;
            Ok(data.into_iter().zip(meta.into_iter()).collect())
        } else {
            let err: ErrorMessage = res.json().context(error::Network {})?;
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
            .put(&url::of_record(&self.base_url, &self.box_id, id))
            .json(&data)
            .send()
            .context(error::Network {})?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err: ErrorMessage = res.json().context(error::Network {})?;
            Err(error::Error::General {
                code: res.status().as_u16(),
                message: err.message,
            })
        }
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let mut res = client
            .delete(&url::of_record(&self.base_url, &self.box_id, id))
            .send()
            .context(error::Network {})?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err: ErrorMessage = res.json().context(error::Network {})?;
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
    fn test_new() {
        let client = Client::new("01234012340123401234");
        assert_eq!(client.base_url, "https://jsonbox.io");
        assert_eq!(client.box_id, "01234012340123401234");
    }

    #[test]
    fn test_with_base_url() {
        let client = Client::with_base_url("01234012340123401234", "https://blog.endflow.net");
        assert_eq!(client.base_url, "https://blog.endflow.net");
        assert_eq!(client.box_id, "01234012340123401234");
    }
}
