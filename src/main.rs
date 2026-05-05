use smtpd::{async_trait, start_server, SmtpConfig, AuthMach};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{

    dotenv().expect(".env file not found");

    let details = ServerDetails::new();
    let  config = SmtpConfig{
        bind_addr: "127.0.0.1:2525".to_string(),
        require_auth: true,
        auth_machs: vec![AuthMach::Plain, AuthMach::Login],
        ..Default::default()
    };
    let factory = MyHandlerFactory { details };

    start_server(config, factory).await?;
    Ok(())

}
#[derive(Clone)]
struct ServerDetails {
    server_username: String,
    server_password: String,
}

impl ServerDetails {
    fn new() -> Self{
        Self {
             server_username: env::var("SERVER_USERNAME").expect("USERNAME not set"),
             server_password: env::var("SERVER_PASSWORD").expect("PASSWORD NOT set"),
            }
    }
}

struct MyHandler {
    details: ServerDetails,
}

// struct User {
//     username: String,
//     password: String,
// }

#[async_trait]
impl smtpd::SmtpHandler for MyHandler{
    async fn handle_auth(
        &mut self,
        _session: &smtpd::Session,
        data: smtpd::AuthData,
    ) -> Result<smtpd::Response, smtpd::Error>{
        
        
        let (username, password, _) = data.data();

        if username == self.details.server_username && password == self.details.server_password{
            return Ok(smtpd::Response::Default);
        }

        Err(smtpd::Error::Abort)
    }

    async fn handle_rcpt(
        &mut self,
        _session: &smtpd::Session,
        _to: &str,
    ) -> Result<smtpd::Response, smtpd::Error>{
        
         return  Ok(smtpd::Response::Default);
        

       
    }
}

struct MyHandlerFactory{
    details: ServerDetails,
}

impl smtpd::SmtpHandlerFactory for MyHandlerFactory {

    type Handler = MyHandler;
    fn new_handler(&self, _session: &smtpd::Session) -> Self::Handler {
        MyHandler {
            details: self.details.clone(),
        }
    }
    
}
