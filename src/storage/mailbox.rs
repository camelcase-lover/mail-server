use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
//use std::path::Uuid;

use super::message::MailMessage;
pub fn save_mail(username: &str, message: &MailMessage) -> std::io::Result<String>{
    let base_path = format!("maildata/mailboxes/{}/inbox", username);

    fs::create_dir_all(&base_path)?;

    let id = Uuid::new_v4().to_string();

    let file_path = format!("{}/{}.eml", base_path, id);

    let mut file = File::create(&file_path)?;

    let content = format!(
        "From: {} \n To: {} \n Subject: {} \n Date: {}\n\n{}",
        message.from,
        message.to,
        message.subject,
        message.date,
        message.body, 
    );
    file.write_all(content.as_bytes())?;

    Ok(file_path)
}