use crate::api::Rpc;
use crate::mailer::Mailer;
use jsonrpc_core::{IoHandler, Result};
use jsonrpc_http_server::*;

struct RpcImpl {
    mailer: Mailer,
}

impl Rpc for RpcImpl {
    fn send_email(&self, recipient: String, subject: String, text: String) -> Result<()> {
        self.mailer.send_email(&recipient, &subject, &text);
        Ok(())
    }
}

impl RpcImpl {
    fn new(mailer: Mailer) -> Self {
        Self { mailer }
    }
}

pub struct Server {
    http_server: jsonrpc_http_server::Server,
}

pub struct ServerConf {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub rpc_http_addr: std::net::SocketAddr,
}

impl Server {
    pub fn wait(self) {
        self.http_server.wait()
    }

    pub fn new(conf: ServerConf) -> Self {
        env_logger::init();

        let ServerConf {
            smtp_server,
            smtp_username,
            smtp_password,
            rpc_http_addr,
            ..
        } = conf;

        let mailer = Mailer::new(smtp_server, smtp_username, smtp_password);

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
