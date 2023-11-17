use std::time::Duration;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String, timeout:Option<u64>) -> Self {
        let mut builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();

        if let Some(timeout_secs) = timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs))
        }

        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host,
        }
    }
}