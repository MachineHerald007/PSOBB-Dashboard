use std::collections::HashMap;

use crate::item;
use crate::item_parser::types::Item;
use crate::item_parser::types::Inventory;
use crate::config::config::Config;

#[derive(Debug)]
pub struct SharedBank<'a> {
    account_type: &'static str,
    mode: u8,
    bank: Inventory<'a>,
    lang: &'a str
}

fn set_account_type(mode: u8) -> &'static str {
    Config::mode_name(mode)
}

pub fn create<'a>(char_data: &'a [u8], mode: u8, lang: &'a str, config: &'a Config<'a>) -> SharedBank<'a> {
    SharedBank {
        account_type: set_account_type(mode),
        mode: mode,
        bank: item::set_items(char_data, set_account_type(mode), lang, char_data.len(), config),
        lang: lang
    }
}