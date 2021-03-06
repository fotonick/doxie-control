use std::fmt;
use std::fs::File;
use std::path::Path;

use chrono::{DateTime, Local};
use chrono::offset::TimeZone;
use failure::{bail, Error, Fail};
use json::{JsonError, JsonValue, parse};
use log::info;
use reqwest::Client;
use url::Url;

const TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Fail)]
pub enum DoxieError {
    #[fail(display = "the download wrote zero bytes")]
    EmptyRead,
}

#[derive(Debug)]
pub struct Doxie {
    pub base_url: Url,
    client: Client,
}

#[derive(Debug)]
pub struct ScanEntry {
    pub name: String,
    pub size: usize,
    pub modified: DateTime<Local>,
}

impl fmt::Display for ScanEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanEntry {{\n    name: \"{}\",\n    size: {}\n    modified: \"{}\"\n}}", self.name, self.size, self.modified.format(TIME_FORMAT))
    }
}

impl Doxie {
    pub fn from_base_url_string(url_string: &str) -> Result<Doxie, Error> {
        Ok(Doxie {
            base_url: Url::parse(url_string)?,
            client: Client::new(),
        })
    }

    pub fn list_scans(&mut self) -> Result<Vec<ScanEntry>, Error> {
        let url = self.base_url.join("/scans.json")?;
        let mut response = self.client.get(url).send()?;
        let json_text = response.text()?;
        info!("received json_text = {}", json_text.to_owned());
        let json = parse(&json_text)?;
        let mut result : Vec<ScanEntry> = vec![];
        match json {
            JsonValue::Array(arr) => {  // top level is specced to be array
                for entry_json in arr {
                    result.push(ScanEntry {
                        name: entry_json["name"].to_string(),
                        size: entry_json["size"].to_string().parse::<usize>()?,
                        modified: Local.datetime_from_str(&entry_json["modified"].to_string(), TIME_FORMAT)?,
                    })
                }
                Ok(result)
            },
            _ => Err(JsonError::WrongType("Expected top-level struct to be array".to_string()).into()),
        }
    }

    pub fn download_scan_by_name(&mut self, name: &str, dest: Option<&str>) -> Result<String, Error> {
        let dest: &str = match dest {
            Some(s) => s,
            None => Path::new(name).file_name().unwrap().to_str().unwrap(),  // default same name into current directory
        };
        let url = self.base_url.join(&format!("scans{}", name))?;
        info!("url = {}", url);
        let mut response = self.client.get(url.as_str()).send()?;
        if response.status().is_success() {
            if response.url() == &url {
                let mut file = File::create(dest)?;
                let num_bytes = response.copy_to(&mut file)?;
                if num_bytes > 0 {
                    Ok(dest.to_string())
                }
                else {
                    Err(DoxieError::EmptyRead.into())
                }
            }
            else {
                bail!("File not found");  // We saw a redirect to the main page, which means file doesn't exist
            }
        }
        else {
            bail!("Request returned status: {}", response.status());
        }
    }

    pub fn delete_scan_by_name(&mut self, name: &str) -> Result<(), Error> {
        let url = self.base_url.join(&format!("scans{}", name))?;
        info!("url = {}", url);
        let response = self.client.delete(url.as_str()).send()?;
        if response.status().is_success() {
            Ok(())
        }
        else {
            bail!("Request returned status: {}", response.status());
        }
    }

    pub fn delete_scans_by_names(&mut self, names: &Vec<&str>) -> Result<(), Error> {
        let url = self.base_url.join("scans/delete.json")?;
        info!("url = {}", url);
        let json = json::stringify(names.to_owned());
        info!("json = {}", json);
        let request = self.client.post(url.as_str()).body(json.to_string());  // weirdly, must use body(), not json()
        info!("request = {:?}", request);
        let response = request.send()?;
        if response.status().is_success() {
            Ok(())
        }
        else {
            bail!("Request returned status: {:?}", response);
        }
    }
}