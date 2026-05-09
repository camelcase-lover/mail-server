mod pop3;
mod storage;

// use pop3::server::start_pop3_server;
use pop3::server::start_pop3_server;

#[tokio::main]
async  fn main(){
    start_pop3_server("127.0.0.1:1100").await.unwrap();
}