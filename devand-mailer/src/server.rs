use crate::api::Rpc;
use crate::mailer::Mailer;
use jsonrpc_core::{IoHandler, Result};
use jsonrpc_http_server::*;
use std::sync::{Arc, Mutex};

struct RpcImpl {
    mailer: Arc<Mutex<Mailer>>,
}

impl Rpc for RpcImpl {
    fn send_email(&self, recipients: Vec<String>, subject: String, text: String) -> Result<()> {
        for recipient in recipients {
            self.mailer.lock().unwrap().send_email(
                recipient.clone(),
                subject.clone(),
                text.clone(),
            );
        }
        Ok(())
    }

    fn verify_address(&self, address: String) -> Result<()> {
        self.mailer.lock().unwrap().verify_address(address);
        Ok(())
    }
}

impl RpcImpl {
    fn new(mailer: Mailer) -> Self {
        Self {
            mailer: Arc::new(Mutex::new(mailer)),
        }
    }
}

pub struct Server {
    http_server: jsonrpc_http_server::Server,
}

pub struct ServerConf {
    pub secret: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub rpc_http_addr: std::net::SocketAddr,
    pub from_name: String,
}

impl Server {
    pub fn wait(self) {
        self.http_server.wait()
    }

    pub fn new(conf: ServerConf) -> Self {
        let ServerConf {
            secret,
            from_name,
            smtp_server,
            smtp_username,
            smtp_password,
            rpc_http_addr,
        } = conf;

        let encoder = devand_crypto::Encoder::new_from_secret(&secret.as_bytes());

        let mailer = Mailer::new(
            smtp_server,
            smtp_username,
            smtp_password,
            from_name,
            encoder,
        );

        let mut io = IoHandler::default();
        let rpc = RpcImpl::new(mailer);
        io.extend_with(rpc.to_delegate());

        let http_server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Null,
            ]))
            .start_http(&rpc_http_addr)
            .expect("Unable to start RPC server");

        Self { http_server }
    }
}
