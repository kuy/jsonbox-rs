use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("Network: {}", "source")]
    Network { source: reqwest::Error },

    #[snafu_display("JSON: {}", "source")]
    Json {
        reason: String,
        source: serde_json::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize)]
pub struct Meta {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_createdOn")]
    pub created_on: String,
}

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

    pub fn list<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut res = reqwest::get(&self.endpoint).context(Network {})?;
        let raw = res.text().context(Network {})?;
        // println!("{}", raw);

        let data: Vec<T> = serde_json::from_str(&raw).context(Json { reason: "data" })?;
        Ok(data)
    }

    pub fn create<T>(&self, data: &T) -> Result<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let mut res = client
            .post(&self.endpoint)
            .json(&data)
            .send()
            .context(Network {})?;
        let raw = res.text().context(Network {})?;
        // println!("{}", raw);

        let data: T = serde_json::from_str(&raw).context(Json { reason: "data" })?;
        Ok(data)
    }
}
