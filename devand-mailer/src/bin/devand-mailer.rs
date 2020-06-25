use devand_mailer::{Server, ServerConf};

fn main() {
    env_logger::init();

    dotenv::dotenv().ok();

    let conf = ServerConf {
        smtp_server: std::env::var("DEVAND_MAILER_SMTP_SERVER").unwrap(),
        smtp_username: std::env::var("DEVAND_MAILER_SMTP_USERNAME").unwrap(),
        smtp_password: std::env::var("DEVAND_MAILER_SMTP_PASSWORD").unwrap(),
        rpc_http_addr: std::env::var("DEVAND_MAILER_RPC_HTTP_ADDR")
            .unwrap()
            .parse()
            .unwrap(),
    };
    let server = Server::new(conf);
    server.wait();
}
