use crate::CcnEmail;

pub struct Client {
    url: String,
}

pub struct ClientConf {
    pub url: String,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
}

// TODO DRY
// TODO Keep runtime

impl Client {
    pub fn new(conf: ClientConf) -> Self {
        Self { url: conf.url }
    }

    #[cfg(not(feature = "mock"))]
    pub fn send_email(&self, email: CcnEmail) -> Result<(), Error> {
        use crate::api::client::GenClient;
        use jsonrpc_core::futures::future::Future;
        use jsonrpc_core_client::transports::http;
        use tokio::runtime::Runtime;

        let mut rt = Runtime::new().map_err(|_| Error::Unknown)?;

        let client_url = &self.url;
        let client = rt
            .block_on(http::connect::<GenClient>(client_url))
            .map_err(|_| Error::Unknown)?;

        client
            .send_email(email)
            .wait()
            .map_err(|_| Error::Unknown)?;

        rt.shutdown_now().wait().map_err(|_| Error::Unknown)?;

        Ok(())
    }

    #[cfg(not(feature = "mock"))]
    pub fn verify_address(&self, address: String) -> Result<(), Error> {
        use crate::api::client::GenClient;
        use jsonrpc_core::futures::future::Future;
        use jsonrpc_core_client::transports::http;
        use tokio::runtime::Runtime;

        let mut rt = Runtime::new().map_err(|_| Error::Unknown)?;

        let client_url = &self.url;
        let client = rt
            .block_on(http::connect::<GenClient>(client_url))
            .map_err(|_| Error::Unknown)?;

        client
            .verify_address(address)
            .wait()
            .map_err(|_| Error::Unknown)?;

        rt.shutdown_now().wait().map_err(|_| Error::Unknown)?;

        Ok(())
    }

    #[cfg(feature = "mock")]
    pub fn send_email(&self, email: CcnEmail) -> Result<(), Error> {
        dbg!(email);
        Ok(())
    }

    #[cfg(feature = "mock")]
    pub fn verify_address(&self, address: String) -> Result<(), Error> {
        dbg!(address);
        Ok(())
    }
}
