use crate::pop3::mailbox::{
    delete_message,
    list_messages,
    read_messages
};

pub struct Pop3Session{
    authenticated: bool,
    username: Option<String>,
}

impl Pop3Session{
    pub fn new() -> Self{
        Self{
            authenticated: false,
            username: None,
        }
    }

    pub async fn handle_command(&mut self, input: &str) -> String{
        let parts: Vec<&str> = input.split_whitespaces().collect();

        if parts.is_empty(){
            return "-ERR Empty command\r\n".to_string();
        }

        match parts[0].to_uppercase().as_str(){
            "USER" => {
                if parts.len() < 2 {
                    return "-ERR Missing username\r\n".to_string();
                }
                self.username = Some(parts[1].to_string());
                "+OK User accepted\r\n".to_string()
            }

            "PASS" => {
                self.authenticated = true;
                "+OK Mailbox locked\r\n".to_string()
            }

            "LIST" => {
                if !self.authenticated {
                    return "-ERR Authenticate first\r\n".to_string();
                }

                let username = self.username.clone().unwrap();

                match list_messages(&username){ 
                    Ok(messages) => {
                        let mut response = String::from("+Ok messages follow\r\n");
                        for (index, size) in messages.iter().enumerate(){
                            response.push_str(&format!("{} {}\r\n", index + 1, size));
                        }

                        response.push_str(".\r\n");

                        response
                    }

                    Err(_) => {
                        "-ERR Failed to read mailbox\r\n".to_string()
                    }
                }
            }

            "RETR" => {
                if !self.authenticated {
                    return "-ERR Authenticate first\r\n".to_string();
                }

                if parts.len < 2 {
                    return "-ERR Missing Message number\r\n".to_string();
                }

                let msg_num: usize = match parts[1].parse(){
                    Ok(n) => n,
                    Err(_) => {
                        return "-ERR Invalid Message number\r\n".to_string();
                    }
                };

                let username = self.username.clone().unwrap();

                match read_message(&username, msg_num){
                    Ok(content)=>{
                        format!("+Ok message follows\r\n{}\r\n.\r\n", content)
                    }

                    Err(_) => {
                    "-ERR Failed to read message\r\n".to_string()
                }
                }
            }

            "DELE" => {
                if !self.authenticated{
                    return "-ERR Authenticate first\r\n".to_string();
                }

                if parts.len() < 2{
                    return "-ERR Missing message number\r\n".to_string();
                }

                let msg_num: usize = match parts[1].parse(){
                    Ok(n) => n, 
                    Err(_) => {
                        return "-ERR Invalid message number\r\n".to_string();
                    }
                };

                let username = self.username.clone().unwrap();

                match delete_message(&message, msg_num,){
                    Ok(_) => {
                        "+Ok Message deleted\r\n".to_string()
                    }
                    Err(_) => {
                        "-ERR Delete Failed\r\n".to_string()
                    }
                }
            }

            "QUIT" => {
                "+Ok Goodbye\r\n".to_string()
            }

            "HELP" => {
                "USER for username example camel \r\n PASS for password 
                \r\n LIST returns all mailbox messages \r\n
                RETR retrive full mail example RETR 1 
                \r\n DELE delete email example DELE 1".to_string()
            }
            _ => {
                "-ERR Uknown command \r\n".to_string
            }
        }
    }
}