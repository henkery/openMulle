use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf}, io::{Read, SeekFrom, Seek, Write}, mem::size_of,
};

use bincode::{config::LittleEndian, Options};
use byteorder::{BigEndian, ReadBytesExt, ByteOrder};
use yore::code_pages::CP1252;

use serde::{Deserialize, Serialize};
use serde_json;

use bevy::prelude::*;

const PALETTE_MAC: &'static [u8] = &[0, 0, 0, 17, 17, 17, 34, 34, 34, 68, 68, 68, 85, 85, 85, 119, 119, 119, 136, 136, 136, 170, 170, 170, 187, 187, 187, 221, 221, 221, 238, 238, 238, 0, 0, 17, 0, 0, 34, 0, 0, 68, 0, 0, 85, 0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 17, 0, 0, 34, 0, 0, 68, 0, 0, 85, 0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 17, 0, 0, 34, 0, 0, 68, 0, 0, 85, 0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 0, 0, 0, 51, 0, 0, 102, 0, 0, 153, 0, 0, 204, 0, 0, 255, 0, 51, 0, 0, 51, 51, 0, 51, 102, 0, 51, 153, 0, 51, 204, 0, 51, 255, 0, 102, 0, 0, 102, 51, 0, 102, 102, 0, 102, 153, 0, 102, 204, 0, 102, 255, 0, 153, 0, 0, 153, 51, 0, 153, 102, 0, 153, 153, 0, 153, 204, 0, 153, 255, 0, 204, 0, 0, 204, 51, 0, 204, 102, 0, 204, 153, 0, 204, 204, 0, 204, 255, 0, 255, 0, 0, 255, 51, 0, 255, 102, 0, 255, 153, 0, 255, 204, 0, 255, 255, 51, 0, 0, 51, 0, 51, 51, 0, 102, 51, 0, 153, 51, 0, 204, 51, 0, 255, 51, 51, 0, 51, 51, 51, 51, 51, 102, 51, 51, 153, 51, 51, 204, 51, 51, 255, 51, 102, 0, 51, 102, 51, 51, 102, 102, 51, 102, 153, 51, 102, 204, 51, 102, 255, 51, 153, 0, 51, 153, 51, 51, 153, 102, 51, 153, 153, 51, 153, 204, 51, 153, 255, 51, 204, 0, 51, 204, 51, 51, 204, 102, 51, 204, 153, 51, 204, 204, 51, 204, 255, 51, 255, 0, 51, 255, 51, 51, 255, 102, 51, 255, 153, 51, 255, 204, 51, 255, 255, 102, 0, 0, 102, 0, 51, 102, 0, 102, 102, 0, 153, 102, 0, 204, 102, 0, 255, 102, 51, 0, 102, 51, 51, 102, 51, 102, 102, 51, 153, 102, 51, 204, 102, 51, 255, 102, 102, 0, 102, 102, 51, 102, 102, 102, 102, 102, 153, 102, 102, 204, 102, 102, 255, 102, 153, 0, 102, 153, 51, 102, 153, 102, 102, 153, 153, 102, 153, 204, 102, 153, 255, 102, 204, 0, 102, 204, 51, 102, 204, 102, 102, 204, 153, 102, 204, 204, 102, 204, 255, 102, 255, 0, 102, 255, 51, 102, 255, 102, 102, 255, 153, 102, 255, 204, 102, 255, 255, 153, 0, 0, 153, 0, 51, 153, 0, 102, 153, 0, 153, 153, 0, 204, 153, 0, 255, 153, 51, 0, 153, 51, 51, 153, 51, 102, 153, 51, 153, 153, 51, 204, 153, 51, 255, 153, 102, 0, 153, 102, 51, 153, 102, 102, 153, 102, 153, 153, 102, 204, 153, 102, 255, 153, 153, 0, 153, 153, 51, 153, 153, 102, 153, 153, 153, 153, 153, 204, 153, 153, 255, 153, 204, 0, 153, 204, 51, 153, 204, 102, 153, 204, 153, 153, 204, 204, 153, 204, 255, 153, 255, 0, 153, 255, 51, 153, 255, 102, 153, 255, 153, 153, 255, 204, 153, 255, 255, 204, 0, 0, 204, 0, 51, 204, 0, 102, 204, 0, 153, 204, 0, 204, 204, 0, 255, 204, 51, 0, 204, 51, 51, 204, 51, 102, 204, 51, 153, 204, 51, 204, 204, 51, 255, 204, 102, 0, 204, 102, 51, 204, 102, 102, 204, 102, 153, 204, 102, 204, 204, 102, 255, 204, 153, 0, 204, 153, 51, 204, 153, 102, 204, 153, 153, 204, 153, 204, 204, 153, 255, 204, 204, 0, 204, 204, 51, 204, 204, 102, 204, 204, 153, 204, 204, 204, 204, 204, 255, 204, 255, 0, 204, 255, 51, 204, 255, 102, 204, 255, 153, 204, 255, 204, 204, 255, 255, 255, 0, 0, 255, 0, 51, 255, 0, 102, 255, 0, 153, 255, 0, 204, 255, 0, 255, 255, 51, 0, 255, 51, 51, 255, 51, 102, 255, 51, 153, 255, 51, 204, 255, 51, 255, 255, 102, 0, 255, 102, 51, 255, 102, 102, 255, 102, 153, 255, 102, 204, 255, 102, 255, 255, 153, 0, 255, 153, 51, 255, 153, 102, 255, 153, 153, 255, 153, 204, 255, 153, 255, 255, 204, 0, 255, 204, 51, 255, 204, 102, 255, 204, 153, 255, 204, 204, 255, 204, 255, 255, 255, 0, 255, 255, 51, 255, 255, 102, 255, 255, 153, 255, 255, 204, 255, 255, 255];

const MULLE_CARS_DIRS: &'static [&'static str] = &[
    "00.cxt",
    "02.dxr",
    "03.dxr",
    "04.dxr",
    "05.dxr",
    "06.dxr",
    "08.dxr",
    "10.dxr",
    "12.dxr",
    "13.dxr",
    "18.dxr",
    "82.dxr",
    "83.dxr",
    "84.dxr",
    "85.dxr",
    "86.dxr",
    "87.dxr",
    "88.dxr",
    "89.dxr",
    "90.dxr",
    "91.dxr",
    "92.dxr",
    "93.dxr",
    "94.dxr",
    "cddata.cxt",
    "tempplug.cxt",
    "unload.dxr",
]; //Note: this is not case sensative because different localised versions will have different casing

pub struct MulleAssetHelperPlugin;

impl Plugin for MulleAssetHelperPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MulleAssetHelp>()
            .add_systems(Startup, parse_meta);
    }
}

pub trait MulleAssetHelper {
    fn find_member(&self, dir: &str, name: &str) -> Option<&Member>;
    fn find_member_path(&self, dir: &str, name: &str, file_ext: &str) -> Option<PathBuf>;
    fn find_member_path_from_actor_name(
        &self,
        dir: &str,
        name: &str,
        file_ext: &str,
    ) -> Option<PathBuf>;
    fn find_member_path_with_asset(&self, dir: &str, name: &str, file_ext: &str)
        -> Option<PathBuf>;
    fn find_member_path_with_asset_from_actor_name(
        &self,
        dir: &str,
        name: &str,
        file_ext: &str,
    ) -> Option<PathBuf>;
}

impl MulleAssetHelper for MulleAssetHelp {
    fn find_member(&self, dir: &str, name: &str) -> Option<&Member> {
        // All "dir" here is lowercase!
        match self.metadatafiles.get(dir) {
            Some(metakey) => {
                for library in &metakey.libraries {
                    return library.members.get(name);
                }
                None
            }
            None => None,
        }
    }

    fn find_member_path(&self, dir: &str, name: &str, file_ext: &str) -> Option<PathBuf> {
        // All "dir" here is lowercase!

        //TODO make file_ext automatically resolve
        match self.metadatafiles.get(dir) {
            Some(metakey) => {
                for library in &metakey.libraries {
                    let path = format!(
                        "cst_out_new/{}/{}/{}{}",
                        metakey.dir, library.name, name, file_ext
                    );
                    return Some(PathBuf::from(path));
                }
                None
            }
            None => None,
        }
    }
    fn find_member_path_with_asset(
        &self,
        dir: &str,
        name: &str,
        file_ext: &str,
    ) -> Option<PathBuf> {
        // All "dir" here is lowercase!

        //TODO make file_ext automatically resolve
        match self.metadatafiles.get(dir) {
            Some(metakey) => {
                for library in &metakey.libraries {
                    let path = format!(
                        "assets/cst_out_new/{}/{}/{}{}",
                        metakey.dir, library.name, name, file_ext
                    );
                    return Some(PathBuf::from(path));
                }
                None
            }
            None => None,
        }
    }
    fn find_member_path_with_asset_from_actor_name(
        &self,
        dir: &str,
        name: &str,
        file_ext: &str,
    ) -> Option<PathBuf> {
        // All "dir" here is lowercase!

        //TODO make file_ext automatically resolve
        if let Some(metakey) = self.metadatafiles.get(dir) {
            for library in &metakey.libraries {
                for (member_name, member) in &library.members {
                    // member.name and member_name ARE NOT THE SAME THING
                    //TODO make this sane
                    if member.name == name {
                        let path = format!(
                            "assets/cst_out_new/{}/{}/{}{}",
                            metakey.dir, library.name, member_name, file_ext
                        );
                        return Some(PathBuf::from(path));
                    }
                }
            }
        }
        None
    }
    fn find_member_path_from_actor_name(
        &self,
        dir: &str,
        name: &str,
        file_ext: &str,
    ) -> Option<PathBuf> {
        // All "dir" here is lowercase!

        //TODO make file_ext automatically resolve
        if let Some(metakey) = self.metadatafiles.get(dir) {
            for library in &metakey.libraries {
                for (member_name, member) in &library.members {
                    // member.name and member_name ARE NOT THE SAME THING
                    //TODO make this sane
                    if member.name == name {
                        let path = format!(
                            "cst_out_new/{}/{}/{}{}",
                            metakey.dir, library.name, member_name, file_ext
                        );
                        return Some(PathBuf::from(path));
                    }
                }
            }
        }
        None
    }
}

fn parse_meta(mut all_metadata: ResMut<MulleAssetHelp>) {

    parse_macromedia_file();

    for dir in MULLE_CARS_DIRS {
        let meta_file_path: PathBuf = {
            let p1 = format!("assets/cst_out_new/{}/metadata.json", dir);
            let p2 = format!("assets/cst_out_new/{}/metadata.json", dir.to_uppercase());
            let path = Path::new(&p1);
            let path2 = Path::new(&p2);
            if path.exists() {
                PathBuf::from(path)
            } else if path2.exists() {
                PathBuf::from(path2)
            } else {
                panic!("failed to find expected dir: {} in assets!", dir);
            }
        };
        match File::open(meta_file_path.to_owned()) {
            Ok(meta_file_handler) => match serde_json::from_reader(meta_file_handler) {
                Ok(meta) => {
                    all_metadata
                        .metadatafiles
                        .insert(dir.to_lowercase().to_string(), meta);
                }
                Err(error) => {
                    panic!(
                        "encountered error in file{}: {}",
                        meta_file_path.clone().to_string_lossy(),
                        error
                    )
                }
            },
            Err(_) => {
                panic!("failed to open expected dir: {} in assets!", dir);
            }
        }
    }
}

// struct ShockwaveFile {
//     header: [u8; 4],//CP1252
//     file_size: u32, // not sure if signed or unsigned
//     file_sign: [u8; 4], //CP1252 string
//     imap: [u8; 4], //CP1252 string
//     imap_length: u32, //CP1252 string
//     imap_unknown: u32, //UNKNOWN DATA
//     mmap_offset: u32,
//     unknown_data: [u8; 28-self.mmap_offset]

// }

enum Endianness {
    Little,
    Big
}

#[derive(Clone, Deserialize)]
struct MacromediaSubFile {
    entry_type: [u8; 4], // CP1252 encoded string
    entry_length: u32,
    entry_offset: u32,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Clone, Deserialize)]
struct MacromediaFileHeader {
    file_size: u32,
    file_sign: [u8; 4], // CP1252 string
    imap: [u8; 4], // CP1252 string
    imap_length: u32,
    imap_unknown: u32,
    mmap_offset: u32,
}

#[derive(Clone, Deserialize)]
struct MacromediaFileHeaderMmap {
    mmap: [u8; 4], // CP1252 string
    mmap_length: u32,
    version: u32, // requires +0xf00
    unknown1: u32,
    amount_of_files: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
}

#[derive(Clone, Deserialize)]
struct MacromediaCastEntryHeader {
    entry_type: [u8; 4],
    entry_length: u32,
}

struct MacromediaCastLibrary {
    lib_slot: u32,
    linked_entries: Vec<u32>
}

struct MacromediaCastMember {
    entry_type: String,
    entry_length: u32,
    data: Vec<u8>
}


fn parse_macromedia_file() {
    let mut file = File::open("assets/00.CXT").unwrap();

    let mut buffer = [0u8; 4];


    file.read_exact(&mut buffer);

    let header_string = CP1252.decode(&buffer);

    let mut endian: Endianness = Endianness::Big;


    if header_string == "RIFX" {
        let endian = Endianness::Little;
        eprint!("little endianness found");
    } else if header_string == "XFIR" {
        // eprint!("big endianness found");

        // TODO replace temp buffers with cursors

        let mut headerbuffer = [0u8; size_of::<MacromediaFileHeader>()];
        file.read_exact(&mut headerbuffer);

        let macromedia_file_header: MacromediaFileHeader = bincode::deserialize(&headerbuffer).unwrap();

        file.seek(SeekFrom::Start(macromedia_file_header.mmap_offset as u64));

        let mut headerbuffermmap = [0u8; size_of::<MacromediaFileHeaderMmap>()];
        file.read_exact(&mut headerbuffermmap);

        let macromedia_file_header_mmap: MacromediaFileHeaderMmap = bincode::deserialize(&headerbuffermmap).unwrap();

        let mut files = Vec::<MacromediaSubFile>::new();

        for i in 0..macromedia_file_header_mmap.amount_of_files {

            let mut filebuffer = [0u8; size_of::<MacromediaSubFile>()];

            file.read_exact(&mut filebuffer);

            let macromedia_sub_file: MacromediaSubFile = bincode::deserialize(&filebuffer).unwrap();

            if macromedia_sub_file.entry_length > 0 { // ignoring 0 byte entries...
                eprint!("found subfile entry of type {} and {} bytes long!", reversed_cp1252_array_to_string(&macromedia_sub_file.entry_type), macromedia_sub_file.entry_length);
            }
            files.push(macromedia_sub_file); // is this expensive?

            
        }

        let mut cast_libraries_map = HashMap::<u32, MacromediaCastLibrary>::new();

        let mut linked_entries = HashMap::<u32, Vec<u32>>::new();

        for subfile_entry in &files {
            if reversed_cp1252_array_to_string(&subfile_entry.entry_type) == "KEY*" { // KEY* entry

                file.seek(SeekFrom::Start(subfile_entry.entry_offset.into()));

                let rel_post = file.stream_position().unwrap();

                let castar_entry_type_raw = file.read_u32::<byteorder::BigEndian>().unwrap().to_le_bytes();
                let castar_entry_length = file.read_u32::<byteorder::LittleEndian>().unwrap();

                _ = file.read_u64::<byteorder::LittleEndian>(); // discarding this data since I don't know what it does
                let amount_of_entries = file.read_u32::<byteorder::LittleEndian>().unwrap();

                for i in 0..amount_of_entries {
                    let cast_file_slot = file.read_u32::<byteorder::LittleEndian>().unwrap();
                    let cast_slot = file.read_u32::<byteorder::LittleEndian>().unwrap();
                    let cast_type_raw = file.read_u32::<byteorder::BigEndian>().unwrap().to_le_bytes();
                    let cast_type = CP1252.decode(&cast_type_raw);

                    if cast_slot >= 1024 {
                        let cast_num = cast_slot - 1024;
                        if ["Lctx", "FXmp", "Cinf", "MCsL", "Sord", "VWCF", "VWFI", "VWLB", "VWSC", "Fmap", "SCRF", "DRCF", "VWFM", "VWtk"].contains(&cast_type.to_string().as_str()) {
                            // ignore this case
                        }
                        else if cast_type == "CAS*" {
                            cast_libraries_map.insert(cast_num, MacromediaCastLibrary { lib_slot: cast_file_slot, linked_entries: Vec::new() });
                        } else {
                            if linked_entries.contains_key(&cast_slot) {
                                linked_entries.get_mut(&cast_slot).unwrap().push(cast_file_slot);
                            } else {
                                linked_entries.insert(cast_slot, Vec::from([cast_file_slot]));
                            }
                        }
                    } else {
                        if linked_entries.contains_key(&cast_slot) {
                            linked_entries.get_mut(&cast_slot).unwrap().push(cast_file_slot);
                        } else {
                            linked_entries.insert(cast_slot, Vec::from([cast_file_slot]));
                        }
                    }
                }
            }
        }

        
        let mut header_buffer = [0u8; size_of::<MacromediaCastEntryHeader>()];

        let mut cast_members = Vec::<(u32, u32)>::new(); // These should be only one member list per library?

        for (index, cast_library) in &cast_libraries_map {
            let subfile = &files[cast_library.lib_slot as usize];
            file.seek(SeekFrom::Start(subfile.entry_offset.into()));
            file.read_exact(&mut header_buffer);
            let cas_star_header: MacromediaCastEntryHeader = bincode::deserialize(&header_buffer).unwrap();


            for i in 0..(cas_star_header.entry_length/4) {

                let cast_slot = file.read_u32::<byteorder::BigEndian>().unwrap(); // WHY IS THIS ONE BE
                let cast_num = i+1;
                if cast_slot != 0 { // no need to store 0 reference
                    cast_members.push((cast_num, cast_slot));
                }
            }
        }

        let mut bitmap_meta = HashMap::<u32, MacromediaCastBitmap>::new();

        for (num, slot) in &cast_members {
            let subfile = &files[slot.clone() as usize];
            file.seek(SeekFrom::Start(subfile.entry_offset.into()));
            file.read_exact(&mut header_buffer);
            let cast_member_preheader: MacromediaCastEntryHeader = bincode::deserialize(&header_buffer).unwrap();
            // let mut cast_member_header_buffer = [0u8; size_of::<MacromediaCastMemberHeader>()];
            // file.read_exact(&mut cast_member_header_buffer);
            // let cast_member_header: MacromediaCastMemberHeader = bincode::deserialize(&cast_member_header_buffer).unwrap(); // WATCH OUT THESE VALUES ARE BE
            let cast_member_cast_type = file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE
            let cast_member_cast_data_length= file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE
            let cast_memer_cast_end_data_length = file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE

            file.seek(SeekFrom::Current(cast_member_cast_data_length.into()));

            if cast_member_cast_type == 1 {

                let unknown1= file.read_u16::<byteorder::BigEndian>().unwrap();
                let image_pos_y= file.read_i16::<byteorder::BigEndian>().unwrap();
                let image_pos_x= file.read_i16::<byteorder::BigEndian>().unwrap();

                bitmap_meta.insert(slot.clone(), MacromediaCastBitmap { 
                    unknown1: unknown1,
                    image_pos_y: image_pos_y,
                    image_pos_x: image_pos_x,
                    image_height: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_y,
                    image_width: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                    _garbage: file.read_u64::<byteorder::BigEndian>().unwrap(),
                    image_reg_y: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_y,
                    image_reg_x: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                    image_bit_is_opaque: file.read_u8().unwrap(), // it remains unclear 
                    image_bit_depth: file.read_u8().unwrap(), // possibly just u8 but only the second nibble?
                    image_bit_alpha: file.read_u8().unwrap(),
                    unknown2: file.read_u8().unwrap(),
                    image_palette: file.read_u16::<byteorder::BigEndian>().unwrap()
                });
            }                
        }

        for (num, slot) in &cast_members {
            
            // appearently you're supposed to do this per "library"
            // appearently you're supposed to validate the libraries by name, I do not know names yet
            // TODO parse the MSCl to get the library name or set a default
            for (linked_num, linked_items) in &linked_entries {

                if linked_num == slot {
                    let subfile = &files[slot.clone() as usize];
                    file.seek(SeekFrom::Start(subfile.entry_offset.into()));
                    file.read_exact(&mut header_buffer);
                    let cast_member_preheader: MacromediaCastEntryHeader = bincode::deserialize(&header_buffer).unwrap();
                    // let mut cast_member_header_buffer = [0u8; size_of::<MacromediaCastMemberHeader>()];
                    // file.read_exact(&mut cast_member_header_buffer);
                    // let cast_member_header: MacromediaCastMemberHeader = bincode::deserialize(&cast_member_header_buffer).unwrap(); // WATCH OUT THESE VALUES ARE BE
                    let cast_member_cast_type = file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE
                    let cast_member_cast_data_length= file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE
                    let cast_memer_cast_end_data_length = file.read_u32::<byteorder::BigEndian>().unwrap(); // ENTERING THE BigEndian ZONE

                    for linked_item in linked_items {
                        // TODO clean this up a lot
                        if cast_member_cast_type == 1 { //TODO use enum?
                            let linked_file = &files[linked_item.clone() as usize];

                            if reversed_cp1252_array_to_string(&linked_file.entry_type) == "BITD" {
                                file.seek(SeekFrom::Start(linked_file.entry_offset.into()));

                                let unknown1 = file.read_u64::<byteorder::BigEndian>().unwrap();

                                let bitmap_meta = bitmap_meta.get(slot).unwrap();

                                let vds = file.stream_position().unwrap();

                                let mut pad = 0;
                                if bitmap_meta.image_width % 2 != 0 { // if image width is divisible by 2 pad equals image height?
                                    pad = bitmap_meta.image_height;
                                }

                                if bitmap_meta.image_bit_depth > 32 {
                                    // bit field mode
                                } else if ((bitmap_meta.image_width + bitmap_meta.image_height) + pad) as u32 == linked_file.entry_length {
                                    // direct palette mode?

                                } else {
                                    // other mode??
                                    let mut rgba_data = Vec::<u8>::with_capacity(((bitmap_meta.image_height as i32*bitmap_meta.image_width as i32) * 4) as usize);

                                    let mut pixel_written = 0;

                                    while pixel_written <= (bitmap_meta.image_height as i32*bitmap_meta.image_width as i32) {
                                        let byte: u16 = file.read_u8().unwrap() as u16;

                                        // we want Rgba8Uint data
                                        // looks like this per pixel: 0x00 0xFF 0XFF 0xFF

                                        if bitmap_meta.image_bit_depth == 32 {
                                            // do something
                                        } else {
                                            if 0x100 - byte > 127 {

                                                if bitmap_meta.image_bit_alpha != 255 {
                                                    eprint!("sd");
                                                }
                                                // lle mode
                                                for j in 0..(byte+1) {
                                                    let val = 0xFF - file.read_u8().unwrap() as u32;

                                                    // convert to RGBA
                                                    let (r,g,b) = (PALETTE_MAC[(val*3) as usize], PALETTE_MAC[((val*3)+1) as usize], PALETTE_MAC[((val*3)+2) as usize]);
                                                    let mut alpha: u8 = 0xff;
                                                    if bitmap_meta.image_bit_is_opaque == 0 && bitmap_meta.image_bit_alpha as u32 == val {
                                                        alpha = 0x00;
                                                    }

                                                    rgba_data.push(r);
                                                    rgba_data.push(g);
                                                    rgba_data.push(b);
                                                    rgba_data.push(alpha);
                                                    pixel_written += 1;
                                                }
        
                                            } else {
                                                // rle mode
                                                let val = 0xFF - file.read_u8().unwrap() as u32;
                                                for j in 0..(0x101-byte) {
                                                    let (r,g,b) = (PALETTE_MAC[(val*3) as usize], PALETTE_MAC[((val*3)+1) as usize], PALETTE_MAC[((val*3)+2) as usize]);
                                                    let mut alpha: u8 = 0xff;
                                                    if bitmap_meta.image_bit_is_opaque == 0 && bitmap_meta.image_bit_alpha as u32 == val {
                                                        alpha = 0x00;
                                                    }

                                                    rgba_data.push(r);
                                                    rgba_data.push(g);
                                                    rgba_data.push(b);
                                                    rgba_data.push(alpha);
                                                    pixel_written += 1;
                                                }
                                            }
                                        }
                                    }
                                    // eprintln!("{} was {}x{}", num, bitmap_meta.image_height, bitmap_meta.image_width);
                                    // let mut dump_file = File::create(format!("{}.bin", num)).unwrap();
                                    // dump_file.write_all(&rgba_data);
                                }
                            }
                        }
                    }
                }
            }
        }
    
    } else {
        eprint!("Not a shockwave file!");
        return;
    }
}

#[derive(Clone, Deserialize)]
struct MacromediaCastBitmap {
    unknown1: u16,
    image_pos_y: i16,
    image_pos_x: i16,
    image_height: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    image_width: i16,// appearently you need to subtract the pos elements of these to get the correct value?
    _garbage: u64,
    image_reg_y: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    image_reg_x: i16,// appearently you need to subtract the pos elements of these to get the correct value?
    // possibly the data ends here if it's 1-but, but is it padded to fit?
    image_bit_depth: u8,
    image_bit_is_opaque: u8,
    image_bit_alpha: u8,
    unknown2: u8,
    image_palette: u16,
}

fn decode_cast_bitmap(data: Vec<u8>) {
    
}

fn reversed_cp1252_array_to_string(array: &[u8; 4]) -> String {
    let mut reversed = [0u8; 4];
    reversed.copy_from_slice(array);
    reversed.reverse();
    CP1252.decode(&reversed).to_string()
}

#[derive(Resource, Default)]
pub struct MulleAssetHelp {
    metadatafiles: HashMap<String, Metadata>,
}

#[derive(Deserialize)]
struct Metadata {
    libraries: Vec<Library>,
    dir: String,
}

#[derive(Deserialize)]
struct Library {
    name: String,
    members: HashMap<String, Member>,
}

#[derive(Deserialize)]
pub struct Member {
    #[serde(alias = "type")]
    pub type_: String,
    pub length: u32,
    #[serde(rename = "castType")]
    pub cast_type: u32,
    pub name: String,
    #[serde(rename = "imagePosY")]
    pub image_pos_y: Option<i32>,
    #[serde(rename = "imagePosX")]
    pub image_pos_x: Option<i32>,
    #[serde(rename = "imageHeight")]
    pub image_height: Option<u32>,
    #[serde(rename = "imageWidth")]
    pub image_width: Option<u32>,
    #[serde(rename = "imageRegY")]
    pub image_reg_y: Option<i32>,
    #[serde(rename = "imageRegX")]
    pub image_reg_x: Option<i32>,
    #[serde(rename = "imageBitAlpha")]
    pub image_bit_alpha: Option<u32>,
    #[serde(rename = "imageBitDepth")]
    pub image_bit_depth: Option<u32>,
    #[serde(rename = "imagePalette")]
    pub image_palette: Option<u32>,
    #[serde(rename = "imageHash")]
    pub image_hash: Option<i128>,
    #[serde(skip)]
    pub sound_cue_points: Option<Vec<Vec<(u32, String)>>>,
    #[serde(rename = "soundLooped")]
    pub sound_looped: Option<bool>,
    #[serde(rename = "soundLength")]
    pub sound_length: Option<u32>,
    #[serde(rename = "soundSampleRate")]
    pub sound_sample_rate: Option<u32>,
}
