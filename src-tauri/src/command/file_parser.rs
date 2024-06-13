use std::str;
use std::collections::HashMap;
use tauri::regex::Regex;
use serde::{ Serialize, Deserialize };

use crate::config::config::Config;
use crate::parser::{
    character,
    shared_bank,
    types::{ Character, SharedBank }
};

type Files = Vec<FileData>;

#[derive(Debug, Deserialize, Clone)]
pub struct FileData {
    pub filename: String,
    pub binary: Vec<u8>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Data {
    Character(Character),
    SharedBank(SharedBank),
}

#[derive(Debug, Serialize, Clone)]
pub struct ParsedFileData {
    pub filename: String,
    pub data: Data,
}

#[derive(Debug, Serialize)]
pub struct ParsedFiles {
    pub files: Vec<ParsedFileData>
}

fn parse(files_to_parse: Files, config: Config) -> Vec<ParsedFileData> {
    let mut parsed_files: Vec<ParsedFileData> = Vec::new();
    let re_psobank = Regex::new(r"psobank").unwrap();
    let re_psochar = Regex::new(r"psochar").unwrap();
    let re_psoclassicbank = Regex::new(r"psoclassicbank").unwrap();
    
    for file in files_to_parse.iter() {
        let binary = &file.binary;

        if re_psobank.is_match(&file.filename) {
            let normal = String::from("NORMAL");
            parsed_files.push(ParsedFileData {
                filename: String::from(&file.filename),
                data: Data::SharedBank(shared_bank::create(&binary[8..4808], Config::mode(normal), config.clone())),
            });
            continue;
        }

        if re_psoclassicbank.is_match(&file.filename) {
            let classic = String::from("CLASSIC");
            parsed_files.push(ParsedFileData {
                filename: String::from(&file.filename),
                data: Data::SharedBank(shared_bank::create(&binary[8..4808], Config::mode(classic), config.clone())),
            });
            continue;
        }

        if re_psochar.is_match(&file.filename) {
            let re_slot = Regex::new(r"Slot_ (\d+)").unwrap();

            let slot_match = re_slot
                .captures(&file.filename)
                .and_then(|caps| caps.get(1))
                .expect("Failed to match the slot");

            let slot: usize = slot_match
                .as_str()
                .parse()
                .expect("Failed to parse the slot to a number");
            
            parsed_files.push(ParsedFileData {
                filename: String::from(&file.filename.to_owned()),
                data: Data::Character(character::create(&binary, slot + 1, config.clone())),
            });
        }
    }
    println!("{:?}", parsed_files);
    parsed_files
}

#[tauri::command]
pub fn parse_files(files: Files, lang: String) -> Result<ParsedFiles, ()> {
    let reg_ex = Regex::new(r"psobank|psoclassicbank|psochar").unwrap();
    let mut files_to_parse: Files = Vec::new();
    let config = Config::init(lang.clone());

    for file in &files {
        if reg_ex.is_match(&file.filename) {
            files_to_parse.push(FileData {
                filename: file.filename.clone(),
                binary: file.binary.clone(),
            });
        }
    }

    if files_to_parse.is_empty() {
        return Err(());
    }

    Ok(ParsedFiles {
        files: parse(files_to_parse, config)
    })
}