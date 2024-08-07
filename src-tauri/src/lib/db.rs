use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result as SqlResult, params};
use std::fs;
use std::io::Read;
use std::path::Path;
use crate::util::Util;
use crate::config::config::Config;
use crate::command::db::{SqlError, CharacterData};
use crate::parser::types::{Item, WrappedItem};
use crate::parser::item::{
    weapon,
    frame,
    barrier,
    unit,
    mag,
    tech,
    tool,
    s_rank_weapon,
    meseta,
    new_item,
    set_meseta
};

pub fn insert_item(conn: &Connection, item: &WrappedItem, account_id: i64, character_id: i64, storage_type: String, account_type: &String, lang: &String) -> Result<(), SqlError> {
    if let Some(item_type) = &item.item {
        match item_type {
            Item::Weapon { name, type_, item_data, special, special_code, grind, attribute, tekked, rare } => {
                conn.execute(
                    "INSERT INTO weapon
                    (
                        account_id, character_id, storage_type, name, type, item_data, special,
                        special_code, grind, native, a_beast, machine, dark, hit, tekked, rare, 
                        account_type, lang
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                    params![
                        account_id, character_id, storage_type, name, type_, item_data, special, special_code,
                        grind, attribute.native, attribute.a_beast, attribute.machine, attribute.dark,
                        attribute.hit, tekked, rare, account_type, lang
                    ]
                )?;
            }
            Item::Frame { name, type_, item_data, slot, addition, max_addition } => {
                conn.execute(
                    "INSERT INTO frame
                    (
                        account_id, character_id, storage_type, name, type, item_data, slot, dfp,
                        evp, max_dfp, max_evp, account_type, lang
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                    params![
                        account_id, character_id, storage_type, name, type_, item_data, slot, addition.dfp,
                        addition.evp, max_addition.dfp, max_addition.evp, account_type, lang
                    ]
                )?;
            }
            Item::Barrier { name, type_, item_data, addition, max_addition } => {
                conn.execute(
                    "INSERT INTO barrier
                    (
                        account_id, character_id, storage_type, name, type, dfp, evp, max_dfp, max_evp,
                        item_data, account_type, lang
                    )
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                     params![
                        account_id, character_id, storage_type, name, type_, addition.dfp, addition.evp,
                        max_addition.dfp, max_addition.evp, item_data, account_type, lang
                    ]
                )?;
            }
            Item::Unit { name, type_, item_data } => {
                conn.execute(
                    "INSERT INTO unit (account_id, character_id, storage_type, name, type, item_data, account_type, lang)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                     params![account_id, character_id, storage_type, name, type_, item_data, account_type, lang]
                )?;
            }
            Item::Mag { name, type_, item_data, level, sync, iq, color, rgb, stats, pbs } => {
                conn.execute(
                    "INSERT INTO mag
                    (
                        account_id, character_id, storage_type, name, type, level, sync, iq, color,
                        rgb, def, pow, dex, mind, pbs, item_data, account_type, lang
                    )
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                     params![
                        account_id, character_id, storage_type, name, type_, level, sync, iq, color, rgb,
                        stats.def, stats.pow, stats.dex, stats.mind, pbs.join(","), item_data, account_type, lang
                    ]
                )?;
            }
            Item::Tech { name, type_, item_data, level } => {
                conn.execute(
                    "INSERT INTO tech (account_id, character_id, storage_type, name, type, level, item_data, account_type, lang)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                     params![account_id, character_id, storage_type, name, type_, level, item_data, account_type, lang]
                )?;
            }
            Item::SRankWeapon { name, type_, item_data, grind, special, special_code } => {
                conn.execute(
                    "INSERT INTO srank_weapon (
                        account_id, character_id, storage_type, name, type, grind, special, special_code, item_data, account_type, lang
                     )
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                     params![account_id, character_id, storage_type, name, type_, grind, special, special_code, item_data, account_type, lang]
                )?;
            }
            Item::Tool { name, type_, item_data, number } => {
                conn.execute(
                    "INSERT INTO tool (account_id, character_id, storage_type, name, type, number, item_data, account_type, lang)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                     params![account_id, character_id, storage_type, name, type_, number, item_data, account_type, lang]
                )?;
            }
            Item::Meseta { name, type_, amount } => {
                conn.execute(
                    "INSERT INTO meseta (account_id, character_id, storage_type, name, type, amount, account_type, lang)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                     params![account_id, character_id, storage_type, name, type_, amount, account_type, lang]
                )?;
            }
            Item::Other { name, type_, item_data, number } => {
                conn.execute(
                    "INSERT INTO other (account_id, character_id, storage_type, name, type, number, item_data, account_type, lang)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                     params![account_id, character_id, storage_type, name, type_, number, item_data, account_type, lang]
                )?;
            }
        }
    } else {
        println!("Item is None");
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DBItem {
    Weapon {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        special: String,
        special_code: String,
        grind: u8,
        native: u8,
        a_beast: u8,
        machine: u8,
        dark: u8,
        hit: u8,
        tekked: bool,
        rare: bool,
        item_data: String,
        account_type: String,
        lang: String
    },
    SRankWeapon {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        grind: u8,
        special: String,
        special_code: String,
        item_data: String,
        account_type: String,
        lang: String
    },
    Frame {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        slot: u8,
        dfp: u8,
        evp: u8,
        max_dfp: u8,
        max_evp: u8,
        item_data: String,
        account_type: String,
        lang: String
    },
    Barrier {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        dfp: u8,
        evp: u8,
        max_dfp: u8,
        max_evp: u8,
        item_data: String,
        account_type: String,
        lang: String
    },
    Unit {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        item_data: String,
        account_type: String,
        lang: String
    },
    Mag {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        level: u8,
        sync: u8,
        iq: u8,
        color: String,
        rgb: String,
        def: u8,
        pow: u8,
        dex: u8,
        mind: u8,
        pbs: String,
        item_data: String,
        account_type: String,
        lang: String
    },
    Tech {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        level: u8,
        item_data: String,
        account_type: String,
        lang: String
    },
    Tool {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        number: u8,
        item_data: String,
        account_type: String,
        lang: String
    },
    Other {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        number: u32,
        item_data: String,
        account_type: String,
        lang: String
    },
    Meseta {
        id: i64,
        account_id: i64,
        character_id: i64,
        storage_type: String,
        type_: u8,
        name: String,
        amount: u32,
        account_type: String,
        lang: String
    }
}

pub fn get_items(conn: &Connection, account_id: i64, character_id: i64, lang: &String) -> Result<Vec<DBItem>, SqlError> {
    let mut items = Vec::new();

    let mut wep_stmt = conn.prepare("SELECT * FROM weapon WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let wep_itr = wep_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Weapon {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            special: row.get(6)?,
            special_code: row.get(7)?,
            grind: row.get(8)?,
            native: row.get(9)?,
            a_beast: row.get(10)?,
            machine: row.get(11)?,
            dark: row.get(12)?,
            hit: row.get(13)?,
            tekked: row.get(14)?,
            rare: row.get(15)?,
            item_data: row.get(16)?,
            account_type: row.get(17)?,
            lang: row.get(18)?
        })
    })?;

    for wep in wep_itr {
        items.push(wep?);
    }

    let mut srank_stmt = conn.prepare("SELECT * FROM srank_weapon WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let srank_itr = srank_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::SRankWeapon {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            grind: row.get(6)?,
            special: row.get(7)?,
            special_code: row.get(8)?,
            item_data: row.get(9)?,
            account_type: row.get(10)?,
            lang: row.get(11)?
        })
    })?;

    for srank in srank_itr {
        items.push(srank?);
    }

    let mut frame_stmt = conn.prepare("SELECT * FROM frame WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let frame_itr = frame_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Frame {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            slot: row.get(6)?,
            dfp: row.get(7)?,
            evp: row.get(8)?,
            max_dfp: row.get(9)?,
            max_evp: row.get(10)?,
            item_data: row.get(11)?,
            account_type: row.get(12)?,
            lang: row.get(13)?
        })
    })?;

    for frame in frame_itr {
        items.push(frame?);
    }

    let mut barrier_stmt = conn.prepare("SELECT * FROM barrier WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let barrier_itr = barrier_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Barrier {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            dfp: row.get(6)?,
            evp: row.get(7)?,
            max_dfp: row.get(8)?,
            max_evp: row.get(9)?,
            item_data: row.get(10)?,
            account_type: row.get(11)?,
            lang: row.get(12)?
        })
    })?;

    for barrier in barrier_itr {
        items.push(barrier?);
    }

    let mut unit_stmt = conn.prepare("SELECT * FROM unit WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let unit_itr = unit_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Unit {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            item_data: row.get(6)?,
            account_type: row.get(7)?,
            lang: row.get(8)?
        })
    })?;

    for unit in unit_itr {
        items.push(unit?);
    }

    let mut mag_stmt = conn.prepare("SELECT * FROM mag WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let mag_itr = mag_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Mag {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            level: row.get(6)?,
            sync: row.get(7)?,
            iq: row.get(8)?,
            color: row.get(9)?,
            rgb: row.get(10)?,
            def: row.get(11)?,
            pow: row.get(12)?,
            dex: row.get(13)?,
            mind: row.get(14)?,
            pbs: row.get(15)?,
            item_data: row.get(16)?,
            account_type: row.get(17)?,
            lang: row.get(18)?
        })
    })?;

    for mag in mag_itr {
        items.push(mag?);
    }

    let mut tech_stmt = conn.prepare("SELECT * FROM tech WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let tech_itr = tech_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Tech {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            level: row.get(6)?,
            item_data: row.get(7)?,
            account_type: row.get(8)?,
            lang: row.get(9)?
        })
    })?;

    for tech in tech_itr {
        items.push(tech?);
    }

    let mut tool_stmt = conn.prepare("SELECT * FROM tool WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let tool_itr = tool_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Tool {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            number: row.get(6)?,
            item_data: row.get(7)?,
            account_type: row.get(8)?,
            lang: row.get(9)?
        })
    })?;

    for tool in tool_itr {
        items.push(tool?);
    }
    
    let mut other_stmt = conn.prepare("SELECT * FROM other WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let other_itr = other_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Other {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            number: row.get(6)?,
            item_data: row.get(7)?,
            account_type: row.get(8)?,
            lang: row.get(9)?
        })
    })?;

    for other in other_itr {
        items.push(other?);
    }

    let mut meseta_stmt = conn.prepare("SELECT * FROM meseta WHERE account_id = ?1 AND character_id = ?2 AND lang = ?3")?;
    let meseta_itr = meseta_stmt.query_map(params![account_id, character_id, lang], |row| {
        Ok(DBItem::Meseta {
            id: row.get(0)?,
            account_id: row.get(1)?,
            character_id: row.get(2)?,
            storage_type: row.get(3)?,
            type_: row.get(4)?,
            name: row.get(5)?,
            amount: row.get(6)?,
            account_type: row.get(7)?,
            lang: row.get(8)?
        })
    })?;

    for meseta in meseta_itr {
        items.push(meseta?);
    }

    Ok(items)
}

pub fn get_character_data(conn: &Connection, account_id: i64, lang: &String) -> Result<Vec<CharacterData>, SqlError> {
    let mut characters = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT
            id, account_id, slot, mode, guild_card, name, class,
            section_id, level, experience, ep1_progress, ep2_progress,
            image
        FROM
        character
        WHERE account_id = ?1;"
    )?;
    
    let character_itr = stmt.query_map(
        [account_id],
        |row| {
            let character_id: i64 = row.get(0)?;
            let items: Vec<DBItem> = get_items(conn, account_id, character_id, lang).expect("get_items error");
            let mut inventory = Vec::new();
            let mut bank = Vec::new();

            for item in items {
                let storage_type = match &item {
                      DBItem::Weapon { storage_type, .. }
                    | DBItem::SRankWeapon { storage_type, .. }
                    | DBItem::Frame { storage_type, .. }
                    | DBItem::Barrier { storage_type, .. }
                    | DBItem::Unit { storage_type, .. }
                    | DBItem::Mag { storage_type, .. }
                    | DBItem::Tech { storage_type, .. }
                    | DBItem::Tool { storage_type, .. }
                    | DBItem::Meseta { storage_type, .. }
                    | DBItem::Other { storage_type, .. } => storage_type.clone(),
                };

                match storage_type.as_str() {
                    "INVENTORY" => inventory.push(item),
                    "BANK" => bank.push(item),
                    _ => println!("Unknown storage type for item: {:?}", item),
                }
            }

            Ok(CharacterData {
                id: row.get(0)?,
                account_id: row.get(1)?,
                slot: row.get(2)?,
                mode: row.get(3)?,
                guild_card: row.get(4)?,
                name: row.get(5)?,
                class: row.get(6)?,
                section_id: row.get(7)?,
                level: row.get(8)?,
                experience: row.get(9)?,
                ep1_progress: row.get(10)?,
                ep2_progress: row.get(11)?,
                image: row.get::<_, Option<Vec<u8>>>(12)?,
                inventory: inventory,
                bank: bank
            })
        }
    )?;

    for character in character_itr {
        characters.push(character?);
    }

    Ok(characters)
}

fn translate_item(item_data: &String, config: &Config) -> WrappedItem {
    let _item_data = Util::hex_string_to_array(&item_data);
    let item_code = Util::binary_array_to_int(&_item_data[0..3]);
    let item_hex_code = Util::binary_array_to_hex(&_item_data[0..3]);
    let item = new_item(_item_data.to_vec(), item_code, config.clone());
    
    item
}

fn translate_meseta(amount: u32, config: &Config) -> WrappedItem {
    let item = set_meseta(amount, config.clone());
    println!("meseta: {:?}", item);

    WrappedItem { item }
}

pub fn translate_items(conn: &Connection, account_id: i64, character_id: i64, items: &Vec<DBItem>, storage_type: String,  config: Config) -> Result<(), SqlError> {
    for _item in items {
        let (item_data, account_type) = match _item {
              DBItem::Weapon { item_data, account_type, .. }
            | DBItem::SRankWeapon { item_data, account_type, .. }
            | DBItem::Frame { item_data, account_type, .. }
            | DBItem::Barrier { item_data, account_type, .. }
            | DBItem::Unit { item_data, account_type, .. }
            | DBItem::Mag { item_data, account_type, .. }
            | DBItem::Tech { item_data, account_type, .. }
            | DBItem::Tool { item_data, account_type, .. }
            | DBItem::Other { item_data, account_type, .. } => (translate_item(item_data, &config), account_type),
            DBItem::Meseta { amount, account_type, .. } => (translate_meseta(*amount, &config), account_type),
        };
        insert_item(conn, &item_data, account_id, character_id, storage_type.clone(), &account_type, &config.lang.clone().unwrap());
    }

    Ok(())
}