use reqwest;
use clap::Parser;
use std::error::Error;

mod structs;
mod data; 

#[derive(Parser)]
#[command(name = "http-ra")]
#[command(author = "Skijetler")]
#[command(version = "0.1.0")]
#[command(about = "Automating http requests", long_about = None)]
struct Cli {
    #[arg(short, long)]
    file:  String,
    #[arg(short, long)]
    count: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut results = Vec::new();
    let mut count = if cli.count.is_some() { cli.count.unwrap() } else { 1 };
    let client = reqwest::Client::new();

    let req: structs::Request = data::parse_file(cli.file).await?;

    while count > 0 {
        count -= 1;
        match req {
            structs::Request::Get { ref url } => {
                results.push(client.get(url)
                                   .send())
            }
            structs::Request::Post { ref url, ref body_type, ref body} => {
                match body_type {
                    structs::RequestBodyType::Form => {
                        results.push(client.post(url)
                                           .form(body)
                                           .send())
                    }
                    structs::RequestBodyType::Json  => {
                        results.push(client.post(url)
                                           .json(body)
                                           .send())
                    }
                }
            }
        }
    }

    for res in results {
        let resp : String = res.await?.text().await?;
        println!("{:?} \n", resp);
    }

    Ok(())
}
