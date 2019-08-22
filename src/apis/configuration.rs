use std::fmt::{Display, Formatter, Result};

use hyper;

pub struct Configuration<C: hyper::client::connect::Connect> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: hyper::client::Client<C>,
    pub token: Option<ApiToken>,
}

#[derive(Clone)]
pub struct ApiToken {
    pub token: String,
}

impl Display for ApiToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Bearer {}", self.token)
    }
}

impl<C: hyper::client::connect::Connect> Configuration<C> {
    pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
        Configuration {
            base_path: "https://api.thetvdb.com".to_string(),
            user_agent: Some("Swagger-Codegen/2.2.0/rust".to_string()),
            client,
            token: None,
        }
    }
}
