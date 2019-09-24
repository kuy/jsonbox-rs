use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Network: {}", "source"))]
    Network { source: reqwest::Error },

    #[snafu(display("JSON: {}", "source"))]
    Json {
        reason: String,
        source: serde_json::Error,
    },

    #[snafu(display("General: [{}] {}", "code", "message"))]
    General { code: u16, message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
