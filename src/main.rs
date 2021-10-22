use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};
use clap::{Arg, App};

#[derive(Serialize, Deserialize)]
struct Item {
    item_id: u32,
    editable_type: u64,
    item_category: u64,
    action_type: u64,
    hit_sound_type: u64,
    item_name: String,
    texture_file: String,
    texture_hash: u32,
    item_kind: u64,
    val1: u32,
    texture_x: u64,
    texture_y: u64,
    spread_type: u64,
    is_stripey_wallpaper: u64,
    collision_type: u64,
    break_hits: u64,
    drop_chance: u32,
    clothing_type: u64,
    rarity: u16,
    max_amount: u64,
    extra_file: String,
    extra_hash: u32,
    audio_volume: u32,
    pet_name: String,
    pet_prefix: String,
    pet_suffix: String,
    pet_ability: String,
    seed_base: u64,
    seed_overlay: u64,
    tree_base: u64,
    tree_leaves: u64,
    seed_color: u32,
    seed_overlay_color: u32,
    grow_time: u32,
    val2: u16,
    is_rayman: u16,
    extra_options: String,
    texture2: String,
    extra_options2: String,
    punch_options: String,
}

#[derive(Serialize, Deserialize)]
struct ItemDataFile {
    file_version: u16,
    item_count: u32,
    items: Vec<Item>,
}

fn decrypt_value(buf: &[u8], item_id: u32) -> String {
    let secret = "PBG892FXX982ABC*";
    let mut name = String::from("");
    for (i, byte) in buf.into_iter().enumerate() {
        let key_pos = (i + item_id as usize) % secret.len();
        let xor_key = secret.chars().nth(key_pos).unwrap();
        let dec_char = byte ^ xor_key as u8;
        name.push(dec_char as char);
    }
    name
}

fn bytes_to_str(vec: &[u8]) -> String {
    let s = match std::str::from_utf8(vec) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_string()
}

fn read_value(reader: &mut BufReader<File>, size: usize) -> Vec<u8> {
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf).unwrap();
    buf
}

fn read_string(reader: &mut BufReader<File>, decrypt: Option<bool>, item_id: Option<u32>) -> String {
    let mut buf = vec![0u8; 2];
    reader.read_exact(&mut buf).unwrap();
    let strlen = LittleEndian::read_u16(&buf);
    if strlen != 0 {
        buf = vec![0u8; strlen as usize];
        reader.read_exact(&mut buf).unwrap();
        if decrypt.unwrap_or(false) {
            decrypt_value(&buf, item_id.expect("invalid Item ID when decrypting string"))
        } else {
            return bytes_to_str(&buf);
        }
    } else {
        String::from("")
    }

}

fn main() {
    let matches = App::new("Growtopia items.dat decoder")
        .version("1.0")
        .about("This program decodes the items.dat file found in growtopia game files. Outputs the information in JSON format")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("The items.dat file path (default: items.dat)"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("The file name to write the data to (default: items.json)"))
        .get_matches();

    let file = OpenOptions::new().read(true).open(
        matches.value_of("file").unwrap_or("items.dat"))
        .expect("File not found");
    let mut buf_reader = BufReader::new(file);
    let file_version = LittleEndian::read_u16(&read_value(&mut buf_reader, 2));
    println!("Items Dat version: {}", file_version);
    let item_count = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
    println!("Number of items: {}", item_count);
    let mut items: Vec<Item> = vec![];
    // Start loop, reading item data
    for i in 0..item_count {
        let item_id = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        let editable_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        let item_category = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        let action_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read hit_sound_type
        let hit_sound_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read item name and decrypt
        let item_name = read_string(&mut buf_reader, Option::from(true), Option::from(item_id));
        // read texture file name
        let texture_file = read_string(&mut buf_reader, None, None);
        // read textureHash value
        let texture_hash = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read itemKind value
        let item_kind = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read val1 (unknown) value type
        let val1 = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read textureX value
        let texture_x = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read textureY value
        let texture_y = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        let spread_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read isStripeyWallpaper value
        let is_stripey_wallpaper = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read collisionType value
        let collision_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read breakHits value
        let break_hits = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1) / 6;
        // read dropChance value
        let drop_chance = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read clothingType value
        let clothing_type = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read rarity value
        let rarity = LittleEndian::read_u16(&read_value(&mut buf_reader, 2));
        // read maxAmount value
        let max_amount = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read extra file name
        let extra_file = read_string(&mut buf_reader, None, None);
        // read extraFileHash value
        let extra_hash = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read audioVolume value
        let audio_volume = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read pet name
        let pet_name = read_string(&mut buf_reader, None, None);
        // read pet prefix
        let pet_prefix = read_string(&mut buf_reader, None, None);
        // read pet suffix
        let pet_suffix = read_string(&mut buf_reader, None, None);
        // read pet ability
        let pet_ability = read_string(&mut buf_reader, None, None);
        // read seedBase value
        let seed_base = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read seedOverlay value
        let seed_overlay = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read treeBase value
        let tree_base = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read treeLeaves value
        let tree_leaves = LittleEndian::read_uint(&read_value(&mut buf_reader, 1), 1);
        // read seedColor value
        let seed_color = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read seedOverlayColor value
        let seed_overlay_color = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        buf_reader.seek_relative(4).unwrap(); // move 4 forward bytes, reason: deleted data (ingredients)
        // read growTime value
        let grow_time = LittleEndian::read_u32(&read_value(&mut buf_reader, 4));
        // read val2 (unknown) value type
        let val2 = LittleEndian::read_u16(&read_value(&mut buf_reader, 2));
        // read isRayman value
        let is_rayman = LittleEndian::read_u16(&read_value(&mut buf_reader, 2));
        // read extraOptions
        let extra_options= read_string(&mut buf_reader, None, None);
        // read texture2
        let texture2= read_string(&mut buf_reader, None, None);
        // read extraOptions2
        let extra_options2= read_string(&mut buf_reader, None, None);
        buf_reader.seek_relative(80).unwrap(); // move 80 forward bytes, reason: file format
        // Doing stuff now based on file version, ready punchOptions var
        let mut punch_options = String::from("");
        if file_version >= 11 {
            // read punchOptions
            punch_options.push_str(&*read_string(&mut buf_reader, None, None));
        }
        // if file version >= 12, move forward 13 bytes
        if file_version >= 12 {buf_reader.seek_relative(13).unwrap();};
        // if file version >= 13, move forward 4 bytes
        if file_version >= 13 {buf_reader.seek_relative(4).unwrap();};
        // if file version >= 14, move forward 4 bytes
        if file_version >= 14 {buf_reader.seek_relative(4).unwrap();};
        // When out of alignment with new file version, add more bytes offset till ok.
        if i != item_id {panic!("Items unordered, check offsets")};
        // If all is good create Item obj
        let item: Item = Item {
            item_id,
            editable_type,
            item_category,
            action_type,
            hit_sound_type,
            item_name,
            texture_file,
            texture_hash,
            item_kind,
            val1,
            texture_x,
            texture_y,
            spread_type,
            is_stripey_wallpaper,
            collision_type,
            break_hits,
            drop_chance,
            clothing_type,
            rarity,
            max_amount,
            extra_file,
            extra_hash,
            audio_volume,
            pet_name,
            pet_prefix,
            pet_suffix,
            pet_ability,
            seed_base,
            seed_overlay,
            tree_base,
            tree_leaves,
            seed_color,
            seed_overlay_color,
            grow_time,
            val2,
            is_rayman,
            extra_options,
            texture2,
            extra_options2,
            punch_options
        };
        items.push(item);
    }
    let data_file: ItemDataFile = ItemDataFile {
        file_version,
        item_count,
        items,
    };
    serde_json::to_writer_pretty(&File::create(matches.value_of("output").unwrap_or("items.json")).unwrap(), &data_file)
        .expect("Unable to write to file, please check if you have permissions to write in this folder");
}

