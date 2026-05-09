use std::fs;
use std::path::PathBuf;

fn inbox_path(username: &str) -> PathBuf{
    PathBuf::from(format!(
        "maildata/mailboxes/{}/inbox", username
    ))
}

pub fn list_messages(username: &str) -> std::io::Result<Vec<u64>>{

    let path = inbox_path(username);
    let mut sizes = Vec::new();

    for entry in fs::read_dir(path)?{
        let entry = entry?;

        let metadata = entry.metadata()?;

        if metadata.is_file(){
            sizes.push(metadata.len());
        }
    }
    Ok(sizes)
}

pub fn read_message(username: &str, index: usize) -> std::io::Result<String>{
    let path = inbox_path(username);

    let files: Vec<_> = fs::read_dir(path)?.filter_map(Result::Ok).collect();
    
    let entry = files.get(index - 1).ok_or(std::io::ErrorKind::NotFound)?;

    fs::read_to_string(entry.path())
}

pub fn delete_message(username: &str, index: usize) -> std::io::Result<()>{
    let path = inbox_path(username);

    let files: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();

    let entry = files.get(index - 1).ok_or(std::io::ErrorKind::NotFound)?;

    fs::remove_file(entry.path())
}