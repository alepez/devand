use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::transports::http;
use tokio::runtime::Runtime;

use crate::api::GenClient;

pub struct Client {
    url: String,
}

pub struct ClientConf {
    pub url: String,
}

impl Client {
    pub fn new(conf: ClientConf) -> Self {
        Self { url: conf.url }
    }

    pub fn send_email(&self, recipient: String, subject: String, text: String) {
        let mut rt = Runtime::new().unwrap();

        let client_url = &self.url;
        let client = rt
            .block_on(http::connect::<GenClient>(&client_url))
            .unwrap();

        client
            .clone()
            .send_email(recipient, subject, text)
            .map(|_| println!("OK"))
            .wait()
            .unwrap();

        rt.shutdown_now().wait().unwrap();
    }
}
