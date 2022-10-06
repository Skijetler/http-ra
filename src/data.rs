use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn parse_file(file_name: String) -> Result<super::structs::Request, Box<dyn Error>> {
    let mut file = File::open(file_name).await?;
    let mut data = String::new();
    file.read_to_string(&mut data).await?;

    let req: super::structs::Request = serde_json::from_str(&data)?;
    
    Ok(req)
}