#![feature(const_trait_impl)]

use reqwest;
use std::{
    io,
    fs
};
use std::error::Error;
use std::path::Path;


const API_KEY: &str = "API_KEY" // https://dash.html2pdf.app/register;


#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    eprint!("Hello, enter the url you want to convert to pdf : ");
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    url.pop();
    if url.contains("\r") {
        url.pop();
    };

    let client = reqwest::blocking::Client::new();
    let req = client.post("https://api.html2pdf.app/v1/generate")
        .query(&[("html", &url), ("apiKey", &API_KEY.to_string()), ("format",&format!("{}", form()))])
        .send();

    let file = format!("{}.pdf", file().trim());

    if !Path::new("./pdf").exists() {
        fs::create_dir("./pdf").expect("Can't create directory");
        fs::write(String::from("./pdf/") + file.trim(), req?.bytes()?)
    } else {
        fs::write(String::from("./pdf/") + file.trim(), req?.bytes()?)
    }.expect("Can't create file");
    println!("{} have been created", file);
    Ok(())
}

fn form() -> String {
    let mut format = String::new();
    eprint!("Format (Letter, Legal, Tabloid, Ledger, A0, A1, A2, A3, A4, A5 or A6) : ");
    io::stdin().read_line(&mut format).unwrap();
    format.pop();
    if format.contains("\r") {
        format.pop();
    };
    let formatarray= ["Letter", "Legal", "Tabloid", "Ledger", "A0", "A1", "A2", "A3", "A4", "A5", "A6"];
    return if !formatarray.contains(&format.trim()){
        println!("You didn't enter any format");
        form();
        String::new()
    } else {
        format
    };
}

fn file() -> String {
    let mut filename = String::new();
    eprint!("File name u want : ");
    io::stdin().read_line(&mut filename).unwrap();
    filename.pop();
    if filename.contains("\r") {
        filename.pop();
    };
    return filename
}
