extern crate imap;
extern crate native_tls;

pub fn fetch_inbox_top() -> imap::error::Result<Options<String>>{
    let domain = "aizenstore.shop";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let messages = imap_sessions.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next(){
        m
    } else {
        return Ok(None);
    };

    let body = message.body().expect("Message did not have a body");
    let body = std::str::from_utf8(body)
    .expect("Message was not valid utf-8")
    .to_string();

    imap_session.logout()?;

    Ok(Some(body))

}