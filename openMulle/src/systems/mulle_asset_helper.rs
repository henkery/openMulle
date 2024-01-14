use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom, Write},
    mem::size_of,
};

use bevy::render::render_resource::{Extent3d, TextureFormat};

use byteorder::ReadBytesExt;
use yore::code_pages::CP1252;

use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

use bevy::prelude::*;

use crate::parsers::database_language::{parse_dictish_structure, Value, get_hashmap_from_dblang, try_get_mulledb, MulleDB};

const PALETTE_MAC: &'static [u8] = &[
    0, 0, 0, 17, 17, 17, 34, 34, 34, 68, 68, 68, 85, 85, 85, 119, 119, 119, 136, 136, 136, 170,
    170, 170, 187, 187, 187, 221, 221, 221, 238, 238, 238, 0, 0, 17, 0, 0, 34, 0, 0, 68, 0, 0, 85,
    0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 17, 0, 0, 34, 0, 0, 68, 0,
    0, 85, 0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 17, 0, 0, 34, 0, 0,
    68, 0, 0, 85, 0, 0, 119, 0, 0, 136, 0, 0, 170, 0, 0, 187, 0, 0, 221, 0, 0, 238, 0, 0, 0, 0, 51,
    0, 0, 102, 0, 0, 153, 0, 0, 204, 0, 0, 255, 0, 51, 0, 0, 51, 51, 0, 51, 102, 0, 51, 153, 0, 51,
    204, 0, 51, 255, 0, 102, 0, 0, 102, 51, 0, 102, 102, 0, 102, 153, 0, 102, 204, 0, 102, 255, 0,
    153, 0, 0, 153, 51, 0, 153, 102, 0, 153, 153, 0, 153, 204, 0, 153, 255, 0, 204, 0, 0, 204, 51,
    0, 204, 102, 0, 204, 153, 0, 204, 204, 0, 204, 255, 0, 255, 0, 0, 255, 51, 0, 255, 102, 0, 255,
    153, 0, 255, 204, 0, 255, 255, 51, 0, 0, 51, 0, 51, 51, 0, 102, 51, 0, 153, 51, 0, 204, 51, 0,
    255, 51, 51, 0, 51, 51, 51, 51, 51, 102, 51, 51, 153, 51, 51, 204, 51, 51, 255, 51, 102, 0, 51,
    102, 51, 51, 102, 102, 51, 102, 153, 51, 102, 204, 51, 102, 255, 51, 153, 0, 51, 153, 51, 51,
    153, 102, 51, 153, 153, 51, 153, 204, 51, 153, 255, 51, 204, 0, 51, 204, 51, 51, 204, 102, 51,
    204, 153, 51, 204, 204, 51, 204, 255, 51, 255, 0, 51, 255, 51, 51, 255, 102, 51, 255, 153, 51,
    255, 204, 51, 255, 255, 102, 0, 0, 102, 0, 51, 102, 0, 102, 102, 0, 153, 102, 0, 204, 102, 0,
    255, 102, 51, 0, 102, 51, 51, 102, 51, 102, 102, 51, 153, 102, 51, 204, 102, 51, 255, 102, 102,
    0, 102, 102, 51, 102, 102, 102, 102, 102, 153, 102, 102, 204, 102, 102, 255, 102, 153, 0, 102,
    153, 51, 102, 153, 102, 102, 153, 153, 102, 153, 204, 102, 153, 255, 102, 204, 0, 102, 204, 51,
    102, 204, 102, 102, 204, 153, 102, 204, 204, 102, 204, 255, 102, 255, 0, 102, 255, 51, 102,
    255, 102, 102, 255, 153, 102, 255, 204, 102, 255, 255, 153, 0, 0, 153, 0, 51, 153, 0, 102, 153,
    0, 153, 153, 0, 204, 153, 0, 255, 153, 51, 0, 153, 51, 51, 153, 51, 102, 153, 51, 153, 153, 51,
    204, 153, 51, 255, 153, 102, 0, 153, 102, 51, 153, 102, 102, 153, 102, 153, 153, 102, 204, 153,
    102, 255, 153, 153, 0, 153, 153, 51, 153, 153, 102, 153, 153, 153, 153, 153, 204, 153, 153,
    255, 153, 204, 0, 153, 204, 51, 153, 204, 102, 153, 204, 153, 153, 204, 204, 153, 204, 255,
    153, 255, 0, 153, 255, 51, 153, 255, 102, 153, 255, 153, 153, 255, 204, 153, 255, 255, 204, 0,
    0, 204, 0, 51, 204, 0, 102, 204, 0, 153, 204, 0, 204, 204, 0, 255, 204, 51, 0, 204, 51, 51,
    204, 51, 102, 204, 51, 153, 204, 51, 204, 204, 51, 255, 204, 102, 0, 204, 102, 51, 204, 102,
    102, 204, 102, 153, 204, 102, 204, 204, 102, 255, 204, 153, 0, 204, 153, 51, 204, 153, 102,
    204, 153, 153, 204, 153, 204, 204, 153, 255, 204, 204, 0, 204, 204, 51, 204, 204, 102, 204,
    204, 153, 204, 204, 204, 204, 204, 255, 204, 255, 0, 204, 255, 51, 204, 255, 102, 204, 255,
    153, 204, 255, 204, 204, 255, 255, 255, 0, 0, 255, 0, 51, 255, 0, 102, 255, 0, 153, 255, 0,
    204, 255, 0, 255, 255, 51, 0, 255, 51, 51, 255, 51, 102, 255, 51, 153, 255, 51, 204, 255, 51,
    255, 255, 102, 0, 255, 102, 51, 255, 102, 102, 255, 102, 153, 255, 102, 204, 255, 102, 255,
    255, 153, 0, 255, 153, 51, 255, 153, 102, 255, 153, 153, 255, 153, 204, 255, 153, 255, 255,
    204, 0, 255, 204, 51, 255, 204, 102, 255, 204, 153, 255, 204, 204, 255, 204, 255, 255, 255, 0,
    255, 255, 51, 255, 255, 102, 255, 255, 153, 255, 255, 204, 255, 255, 255,
];

lazy_static! {
    static ref OPAQUE: HashMap<String, Vec<u32>> = HashMap::from([
        (
            "00.cxt".to_string(),
            Vec::from([64, 65, 66, 67, 68, 69, 70, 71, 72, 75, 76, 81, 83, 84, 86])
        ),
        ("02.dxr".to_string(), Vec::from([66, 68, 69, 70, 71, 72])),
        ("03.dxr".to_string(), Vec::from([33, 100, 101])),
        (
            "04.dxr".to_string(),
            Vec::from([16, 17, 27, 30, 37, 116, 117, 118, 145, 146, 228, 229, 230])
        ),
        ("05.dxr".to_string(), Vec::from([25, 26, 53, 54, 57])),
        (
            "10.dxr".to_string(),
            Vec::from([1, 2, 5, 12, 13, 92, 93, 94, 95, 96, 173, 174, 188])
        ),
        ("18.dxr".to_string(), Vec::from([8, 12, 13])),
        ("84.dxr".to_string(), Vec::from([25])),
        ("85.dxr".to_string(), Vec::from([25])),
        ("86.dxr".to_string(), Vec::from([1])),
        ("87.dxr".to_string(), Vec::from([15, 16, 17, 18, 208])),
        (
            "88.dxr".to_string(),
            Vec::from([
                32, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 92, 93, 96, 97, 100, 101
            ])
        ),
        ("92.dxr".to_string(), Vec::from([1])),
        ("94.dxr".to_string(), Vec::from([200])),
        (
            "cddata.cxt".to_string(),
            Vec::from([
                629, 630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 640, 641, 642, 643, 644,
                645, 646, 647, 648, 649, 650, 651, 652, 653, 654, 656, 657, 658, 661, 662, 663,
                664, 665, 666, 667, 668, 669, 670, 671, 672, 673, 674, 675, 676, 677, 678, 679,
                680, 681, 682, 683, 684, 685, 686, 687, 688
            ])
        ),
        ("Plugin.cst".to_string(), Vec::from([18]))
    ]);
}

const MULLE_CARS_FILES: &'static [&'static str] = &[
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
            .add_systems(PreStartup, parse_meta);
    }
}

pub trait MulleAssetHelper {
    fn get_image_by_asset_number(&self, dir: String, name: u32) -> Option<&Handle<Image>>;
    fn get_image_by_name(&self, dir: String, name: String) -> Option<&Handle<Image>>;
    fn get_mulle_file_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleFile>;
    fn get_mulle_file_by_name(&self, dir: String, name: String) -> Option<&MulleFile>;
    fn get_mulle_image_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleImage>;
    fn get_mulle_text_by_name(&self, dir: String, name: String) -> Option<&MulleText>;
    fn get_mulle_text_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleText>;
    fn get_mulle_db_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleDBHolder>;
}

impl MulleAssetHelper for MulleAssetHelp {
    fn get_image_by_asset_number(&self, dir: String, name: u32) -> Option<&Handle<Image>> {
        if let Some(mulle_file) = self.get_mulle_file_by_asset_number(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleImage(image) => return Some(&image.image),
                _ => return None,
            });
        }
        None
    }
    fn get_mulle_image_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleImage> {
        if let Some(mulle_file) = self.get_mulle_file_by_asset_number(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleImage(image) => return Some(image),
                _ => return None,
            });
        }
        None
    }
    fn get_mulle_text_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleText> {
        if let Some(mulle_file) = self.get_mulle_file_by_asset_number(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleText(text) => return Some(text),
                _ => return None,
            });
        }
        None
    }
    fn get_mulle_text_by_name(&self, dir: String, name: String) -> Option<&MulleText> {
        if let Some(mulle_file) = self.get_mulle_file_by_name(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleText(text) => return Some(text),
                _ => return None,
            });
        }
        None
    }
    fn get_image_by_name(&self, dir: String, name: String) -> Option<&Handle<Image>> {
        if let Some(mulle_file) = self.get_mulle_file_by_name(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleImage(image) => return Some(&image.image),
                _ => return None,
            });
        }
        None
    }
    fn get_mulle_file_by_name(&self, dir: String, name: String) -> Option<&MulleFile> {
        if let Some(mulle_library) = self.metadatafiles.get(&dir) {
            for (_num, mulle_file) in &mulle_library.files {
                if mulle_file.name() == name {
                    // is this expensive?
                    return Some(&mulle_file);
                }
            }
        }
        None
    }
    fn get_mulle_file_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleFile> {
        if let Some(mulle_library) = self.metadatafiles.get(&dir) {
            if let Some(mulle_file) = mulle_library.files.get(&name) {
                return Some(&mulle_file);
            }
        }
        None
    }
    fn get_mulle_db_by_asset_number(&self, dir: String, name: u32) -> Option<&MulleDBHolder> {
        if let Some(mulle_file) = self.get_mulle_file_by_asset_number(dir, name) {
            return Some(match mulle_file {
                MulleFile::MulleDB(db) => return Some(db),
                _ => return None,
            });
        }
        None
    }
}

fn parse_meta(mut all_metadata: ResMut<MulleAssetHelp>, mut images: ResMut<Assets<Image>>) {
    for dir in MULLE_CARS_FILES {
        let mut mulle_library = MulleLibrary {
            name: "".to_string(),
            files: HashMap::new(),
        };
        let mut file: File;
        if let Ok(file_handle) = File::open(format!("assets/{}", dir.to_string())) {
            // all_metadata.metadatafiles.insert(dir.to_string(), mulle_library);
            file = file_handle;
        } else if let Ok(file_handle) = File::open(format!("assets/{}", dir.to_uppercase())) {
            // all_metadata.metadatafiles.insert(dir.to_string(), mulle_library);
            // file = File::open(filename).unwrap();
            file = file_handle;
        } else {
            return;
        }

        let mut buffer = [0u8; 4];

        _ = file.read_exact(&mut buffer);

        let header_string = CP1252.decode(&buffer);

        let mut endian = Endianness::Little;

        if header_string == "RIFX" {
            endian = Endianness::Little;
            // eprint!("little endianness found");
        } else if header_string == "XFIR" {
            // eprint!("big endianness found");

            endian = Endianness::Big;
            // TODO replace temp buffers with cursors
        } else {
            eprint!("Not a shockwave file!");
        }

        //TODO use a sane method

        let macromedia_file_header = MacromediaFileHeader {
            file_size: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            file_sign: match &endian {
                //surely this can be done better
                Endianness::Big => file
                    .read_u32::<byteorder::LittleEndian>()
                    .unwrap()
                    .to_le_bytes(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file
                    .read_u32::<byteorder::BigEndian>()
                    .unwrap()
                    .to_le_bytes(),
            },
            imap: match &endian {
                //surely this can be done better
                Endianness::Big => file
                    .read_u32::<byteorder::LittleEndian>()
                    .unwrap()
                    .to_le_bytes(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file
                    .read_u32::<byteorder::BigEndian>()
                    .unwrap()
                    .to_le_bytes(),
            },
            imap_length: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            imap_unknown: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            mmap_offset: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
        };

        _ = file.seek(SeekFrom::Start(macromedia_file_header.mmap_offset as u64));

        let macromedia_file_header_mmap: MacromediaFileHeaderMmap = MacromediaFileHeaderMmap {
            mmap: match &endian {
                //surely this can be done better
                Endianness::Big => file
                    .read_u32::<byteorder::LittleEndian>()
                    .unwrap()
                    .to_le_bytes(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file
                    .read_u32::<byteorder::BigEndian>()
                    .unwrap()
                    .to_le_bytes(),
            },
            mmap_length: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            version: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            unknown1: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            amount_of_files: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            unknown2: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            unknown3: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
            unknown4: match &endian {
                //surely this can be done better
                Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
            },
        };

        let mut files = Vec::<MacromediaSubFile>::new();

        for _ in 0..macromedia_file_header_mmap.amount_of_files {
            let macromedia_sub_file: MacromediaSubFile = MacromediaSubFile {
                entry_type: match &endian {
                    //surely this can be done better
                    Endianness::Big => file
                        .read_u32::<byteorder::LittleEndian>()
                        .unwrap()
                        .to_le_bytes(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file
                        .read_u32::<byteorder::BigEndian>()
                        .unwrap()
                        .to_le_bytes(),
                },
                entry_length: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
                entry_offset: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
                unknown1: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
                index: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
            };

            files.push(macromedia_sub_file); // is this expensive?
                                             // TODO figure out if this is expensive
        }

        let mut cast_libraries_map = HashMap::<u32, MacromediaCastLibrary>::new();

        let mut linked_entries = HashMap::<u32, Vec<u32>>::new();

        for subfile_entry in &files {
            if reversed_cp1252_array_to_string(&subfile_entry.entry_type) == "KEY*" {
                // KEY* entry

                _ = file.seek(SeekFrom::Start(subfile_entry.entry_offset.into()));

                let _castar_entry_type_raw = match &endian {
                    //surely this can be done better
                    Endianness::Big => file
                        .read_u32::<byteorder::BigEndian>()
                        .unwrap()
                        .to_le_bytes(),
                    Endianness::Little => file
                        .read_u32::<byteorder::LittleEndian>()
                        .unwrap()
                        .to_le_bytes(),
                };

                let _castar_entry_length = match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                };

                _ = file.read_u64::<byteorder::LittleEndian>(); // discarding this data since I don't know what it does
                let amount_of_entries = match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                };

                for _ in 0..amount_of_entries {
                    let cast_file_slot = match &endian {
                        //surely this can be done better
                        Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                        Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                    };
                    let cast_slot = match &endian {
                        //surely this can be done better
                        Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                        Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                    };
                    let cast_type_raw = match &endian {
                        //surely this can be done better
                        Endianness::Big => file
                            .read_u32::<byteorder::BigEndian>()
                            .unwrap()
                            .to_le_bytes(),
                        Endianness::Little => file
                            .read_u32::<byteorder::LittleEndian>()
                            .unwrap()
                            .to_le_bytes(),
                    };
                    let cast_type = CP1252.decode(&cast_type_raw);

                    if cast_slot >= 1024 {
                        let cast_num = cast_slot - 1024;
                        if [
                            "FXmp", "Cinf", "MCsL", "Sord", "VWCF", "VWFI", "VWLB", "VWSC", "Fmap",
                            "SCRF", "DRCF", "VWFM", "VWtk",
                        ]
                        .contains(&cast_type.to_string().as_str())
                        {
                            // ignore this case
                        } else if cast_type == "CAS*" {
                            cast_libraries_map.insert(
                                cast_num,
                                MacromediaCastLibrary {
                                    lib_slot: cast_file_slot,
                                    linked_entries: Vec::new(),
                                },
                            );
                        } else {
                            if linked_entries.contains_key(&cast_slot) {
                                linked_entries
                                    .get_mut(&cast_slot)
                                    .unwrap()
                                    .push(cast_file_slot);
                            } else {
                                linked_entries.insert(cast_slot, Vec::from([cast_file_slot]));
                            }
                        }
                    } else {
                        if linked_entries.contains_key(&cast_slot) {
                            linked_entries
                                .get_mut(&cast_slot)
                                .unwrap()
                                .push(cast_file_slot);
                        } else {
                            linked_entries.insert(cast_slot, Vec::from([cast_file_slot]));
                        }
                    }
                }
            }
        }

        let _header_buffer = [0u8; size_of::<MacromediaCastEntryHeader>()];

        let mut cast_members = Vec::<(u32, u32)>::new(); // These should be only one member list per library?

        for (_index, cast_library) in &cast_libraries_map {
            let subfile = &files[cast_library.lib_slot as usize];
            file.seek(SeekFrom::Start(subfile.entry_offset.into()));
            let cas_star_header: MacromediaCastEntryHeader = MacromediaCastEntryHeader {
                entry_type: match &endian {
                    //surely this can be done better
                    Endianness::Big => file
                        .read_u32::<byteorder::BigEndian>()
                        .unwrap()
                        .to_le_bytes(),
                    Endianness::Little => file
                        .read_u32::<byteorder::LittleEndian>()
                        .unwrap()
                        .to_le_bytes(),
                },
                entry_length: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
            };

            for i in 0..(cas_star_header.entry_length / 4) {
                let cast_slot = file.read_u32::<byteorder::BigEndian>().unwrap(); // this one is always bigendian???

                let cast_num = i + 1;
                if cast_slot != 0 {
                    // no need to store 0 reference
                    cast_members.push((cast_num, cast_slot));
                }
            }
        }

        let mut bitmap_meta = HashMap::<u32, MacromediaCastBitmapMetadata>::new();
        let mut castmember_name = HashMap::<u32, String>::new();

        for (num, slot) in &cast_members {
            let subfile = &files[slot.clone() as usize];
            file.seek(SeekFrom::Start(subfile.entry_offset.into()));
            let _cast_member_preheader: MacromediaCastEntryHeader = MacromediaCastEntryHeader {
                entry_type: match &endian {
                    //surely this can be done better
                    Endianness::Big => file
                        .read_u32::<byteorder::BigEndian>()
                        .unwrap()
                        .to_le_bytes(),
                    Endianness::Little => file
                        .read_u32::<byteorder::LittleEndian>()
                        .unwrap()
                        .to_le_bytes(),
                },
                entry_length: match &endian {
                    //surely this can be done better
                    Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(), // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                    Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                },
            };
            let cast_member_cast_type = file.read_u32::<byteorder::BigEndian>().unwrap();
            let cast_member_cast_data_length = file.read_u32::<byteorder::BigEndian>().unwrap();
            let _cast_member_cast_end_data_length =
                file.read_u32::<byteorder::BigEndian>().unwrap();

            let pre_meta_pos = file.stream_position().unwrap();

            // Metadata of the cast_member is here
            if cast_member_cast_data_length > 0 {
                let _cast_member_unknown = file.read_u128::<byteorder::LittleEndian>().unwrap(); // gap of unknown data
                let _cast_member_unknown2 = file.read_u128::<byteorder::LittleEndian>().unwrap(); // gap of unknown data

                let cast_member_num = file.read_u16::<byteorder::BigEndian>().unwrap();
                let mut cast_member_field_offsets = Vec::<u32>::new();

                for _ in 0..cast_member_num {
                    cast_member_field_offsets
                        .push(file.read_u32::<byteorder::BigEndian>().unwrap());
                }

                let cast_member_field_data_length =
                    file.read_u32::<byteorder::BigEndian>().unwrap();

                let pre_member_field_pos = file.stream_position().unwrap();

                let mut member_fields = Vec::<String>::new();

                for offset in cast_member_field_offsets {
                    file.seek(SeekFrom::Start(pre_member_field_pos + offset as u64));
                    let string_length = file.read_u8().unwrap();
                    if string_length == 0 || string_length as u32 > cast_member_field_data_length {
                        continue;
                    }
                    let mut member_string = vec![0u8; string_length as usize];
                    file.read_exact(&mut member_string);
                    // member_string.reverse();
                    member_fields.push(CP1252.decode(&member_string).to_string());
                }

                if let Some(name) = member_fields.get(0) {
                    castmember_name.insert(num.clone(), name.clone());
                }
            }

            file.seek(SeekFrom::Start(
                pre_meta_pos + cast_member_cast_data_length as u64,
            ));

            if cast_member_cast_type == 1 {
                let unknown1 = file.read_u16::<byteorder::BigEndian>().unwrap(); //ignoring endianness for unknowns... //V27??

                let image_pos_y = file.read_i16::<byteorder::BigEndian>().unwrap(); // these are always BE for some reason
                let image_pos_x = file.read_i16::<byteorder::BigEndian>().unwrap();

                bitmap_meta.insert(
                    slot.clone(),
                    MacromediaCastBitmapMetadata {
                        //image struct is always BE!
                        v27: unknown1,
                        image_pos_y: image_pos_y,
                        image_pos_x: image_pos_x,
                        image_height: file.read_i16::<byteorder::BigEndian>().unwrap()
                            - image_pos_y,
                        image_width: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                        alpha_treshold: file.read_u16::<byteorder::BigEndian>().unwrap(),
                        _ole1: file.read_u32::<byteorder::BigEndian>().unwrap(),
                        _ole2: file.read_u16::<byteorder::BigEndian>().unwrap(),
                        image_reg_y: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_y,
                        image_reg_x: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                        flags: file.read_u8().unwrap(), // it remains unclear
                        image_bit_depth: file.read_u8().unwrap(), // this part may not exist
                        _image_palette: file.read_u32::<byteorder::BigEndian>().unwrap(),
                    },
                );
            }
            // Known types and details
            // 1: bitmap_metadata
            // 2: filmloop?
            // 3: field?
            // 4: Palette
            // 5: Picture?
            // 6: Audio (file? metadata?), Member field may contain hints towards audio format
            // 7: button
            // 8: shape
            // 9: movie
            // 10: digitalvideo
            // 11: scripts?
            // 12: Text?
            // 13: OLE?
            // 14: Transition
        }

        for (num, slot) in &cast_members {
            // appearently you're supposed to do this per "library"
            // appearently you're supposed to validate the libraries by name, I do not know names yet
            // TODO parse the MSCl to get the library name or set a default
            for (linked_num, linked_items) in &linked_entries {
                if linked_num == slot {
                    let subfile = &files[slot.clone() as usize];
                    file.seek(SeekFrom::Start(subfile.entry_offset.into()));
                    let _cast_member_preheader: MacromediaCastEntryHeader =
                        MacromediaCastEntryHeader {
                            entry_type: match &endian {
                                //surely this can be done better
                                Endianness::Big => file
                                    .read_u32::<byteorder::BigEndian>()
                                    .unwrap()
                                    .to_le_bytes(),
                                Endianness::Little => file
                                    .read_u32::<byteorder::LittleEndian>()
                                    .unwrap()
                                    .to_le_bytes(),
                            },
                            entry_length: match &endian {
                                //surely this can be done better
                                Endianness::Big => {
                                    file.read_u32::<byteorder::LittleEndian>().unwrap()
                                } // yes those are reversed, yes that is the point, no I do not know why macromedia is like this
                                Endianness::Little => {
                                    file.read_u32::<byteorder::BigEndian>().unwrap()
                                }
                            },
                        };
                    // let mut cast_member_header_buffer = [0u8; size_of::<MacromediaCastMemberHeader>()];
                    // let cast_member_header: MacromediaCastMemberHeader = bincode::deserialize(&cast_member_header_buffer).unwrap(); // WATCH OUT THESE VALUES ARE BE
                    let cast_member_cast_type = file.read_u32::<byteorder::BigEndian>().unwrap(); // this one is always BE
                    let _cast_member_cast_data_length =
                        file.read_u32::<byteorder::BigEndian>().unwrap(); // this one is always BE?
                    let _cast_memer_cast_end_data_length = match &endian {
                        //surely this can be done better
                        Endianness::Little => file.read_u32::<byteorder::BigEndian>().unwrap(),
                        Endianness::Big => file.read_u32::<byteorder::LittleEndian>().unwrap(),
                    };

                    for linked_item in linked_items {
                        // TODO clean this up a lot
                        if cast_member_cast_type == 1 {
                            //TODO use enum?
                            let linked_file = &files[linked_item.clone() as usize];

                            if reversed_cp1252_array_to_string(&linked_file.entry_type) == "BITD" {
                                file.seek(SeekFrom::Start(linked_file.entry_offset.into()));

                                let _unknown1 = file.read_u64::<byteorder::BigEndian>().unwrap(); //ignoring endianness of unknown values

                                let bitmap_meta = bitmap_meta.get(slot).unwrap();

                                let mut img_buffer = vec![0u8; linked_file.entry_length as usize];
                                file.read_exact(&mut img_buffer);
                                let mut img_cursor = Cursor::new(img_buffer);

                                let mut pad = 0;
                                if bitmap_meta.image_width % 2 != 0 {
                                    // if image width is divisible by 2 pad equals image height?
                                    pad = bitmap_meta.image_height;
                                }

                                let is_opaque = if let Some(numvec) = OPAQUE.get(*dir) {
                                    numvec.contains(num)
                                } else {
                                    false
                                }; // is this expensive?

                                if bitmap_meta.image_bit_depth > 32 {
                                    // bit field mode
                                } else if ((bitmap_meta.image_width as i32
                                    * bitmap_meta.image_height as i32)
                                    + pad as i32) as u32
                                    == linked_file.entry_length
                                {
                                    let rgba_data = decode_direct_palette_image(
                                        bitmap_meta,
                                        is_opaque,
                                        &mut img_cursor,
                                    );
                                    mulle_library.files.insert(
                                        num.clone(),
                                        MulleFile::MulleImage(MulleImage {
                                            name: match castmember_name.get(num) {
                                                None => "default".to_string(),
                                                Some(name) => name.clone(),
                                            },
                                            bitmap_metadata: bitmap_meta.clone(),
                                            image: images.add(Image::new(
                                                Extent3d {
                                                    width: bitmap_meta.image_width as u32,
                                                    height: bitmap_meta.image_height as u32,
                                                    depth_or_array_layers: 1,
                                                },
                                                bevy::render::render_resource::TextureDimension::D2,
                                                rgba_data,
                                                TextureFormat::Rgba8UnormSrgb,
                                            )),
                                        }),
                                    );
                                    // direct palette mode?
                                } else {
                                    // other mode??
                                    let rgba_data =
                                        decode_8bit_image(bitmap_meta, is_opaque, &mut img_cursor);

                                    mulle_library.files.insert(
                                        num.clone(),
                                        MulleFile::MulleImage(MulleImage {
                                            name: match castmember_name.get(num) {
                                                None => "default".to_string(),
                                                Some(name) => name.clone(),
                                            },
                                            bitmap_metadata: bitmap_meta.clone(),
                                            image: images.add(Image::new(
                                                Extent3d {
                                                    width: bitmap_meta.image_width as u32,
                                                    height: bitmap_meta.image_height as u32,
                                                    depth_or_array_layers: 1,
                                                },
                                                bevy::render::render_resource::TextureDimension::D2,
                                                rgba_data,
                                                TextureFormat::Rgba8UnormSrgb,
                                            )),
                                        }),
                                    );
                                    // eprintln!("{} was {}x{}", num, bitmap_meta.image_height, bitmap_meta.image_width);
                                    // let mut dump_file = File::create(format!("{}.bin", num)).unwrap();
                                    // dump_file.write_all(&rgba_data);
                                }
                            }
                        } else if cast_member_cast_type == 6 {
                            // Sound
                            let linked_file = &files[linked_item.clone() as usize];

                            if reversed_cp1252_array_to_string(&linked_file.entry_type) == "BITD" {}
                        } else if cast_member_cast_type == 3 {
                            // styled text
                            let linked_file = &files[linked_item.clone() as usize];

                            if reversed_cp1252_array_to_string(&linked_file.entry_type) == "STXT" {
                                file.seek(SeekFrom::Start(linked_file.entry_offset as u64 + 8)); // +8 to skip the fourcc

                                let mut stxt_buffer = vec![0u8; linked_file.entry_length as usize];
                                file.read_exact(&mut stxt_buffer);
                                let mut stxt_cursor = Cursor::new(stxt_buffer);

                                let _unknown =
                                    stxt_cursor.read_u32::<byteorder::BigEndian>().unwrap();
                                let text_length =
                                    stxt_cursor.read_u32::<byteorder::BigEndian>().unwrap();
                                let _text_padding =
                                    stxt_cursor.read_u32::<byteorder::BigEndian>().unwrap();
                                let mut text_content = vec![0u8; text_length as usize];
                                stxt_cursor.read_exact(&mut text_content);
                                
                                if let Some(name) = castmember_name.get(num) {
                                    if name.contains("DB") {
                                        match try_get_mulledb(CP1252.decode(&text_content).to_string()) {
                                            Some(db) => {
                                                mulle_library.files.insert(
                                                    num.clone(),
                                                    MulleFile::MulleDB(MulleDBHolder {
                                                        name: name.clone(),
                                                        db: db,
                                                    }),
                                                );
                                            },
                                            None => { eprint!("attempted but failed to parse {}, {}", name, num)},
                                        }
                                        continue
                                    }
                                }
                                mulle_library.files.insert(
                                    num.clone(),
                                    MulleFile::MulleText(MulleText {
                                        name: match castmember_name.get(num) {
                                            None => "default".to_string(),
                                            Some(name) => name.clone(),
                                        },
                                        text: CP1252.decode(&text_content).to_string(),
                                    }),
                                );
                            }
                        } else if cast_member_cast_type == 11 {
                            continue;
                            let linked_file = &files[linked_item.clone() as usize];
                            file.seek(SeekFrom::Start(linked_file.entry_offset as u64 + 8)); // +8 to skip the fourcc

                            let mut stxt_buffer = vec![0u8; linked_file.entry_length as usize];
                            file.read_exact(&mut stxt_buffer);
                            let _stxt_cursor = Cursor::new(stxt_buffer);
                        } else if cast_member_cast_type == 12 {
                            // rich text?
                            let linked_file = &files[linked_item.clone() as usize];

                            if reversed_cp1252_array_to_string(&linked_file.entry_type)
                                .contains("RTE")
                            {
                                continue; // Not supported
                                file.seek(SeekFrom::Start(linked_file.entry_offset.into()));

                                let mut img_buffer = vec![0u8; linked_file.entry_length as usize];
                                file.read_exact(&mut img_buffer);
                                let mut img_cursor = Cursor::new(img_buffer);

                                let _RTE0_len =
                                    img_cursor.read_u32::<byteorder::LittleEndian>().unwrap();
                                // Contains names of fonts?
                            } else {
                                file.seek(SeekFrom::Start(linked_file.entry_offset.into()));

                                let mut img_buffer = vec![0u8; linked_file.entry_length as usize];
                                file.read_exact(&mut img_buffer);
                                let _img_cursor = Cursor::new(img_buffer);
                            }
                        } else {
                            // some kind of script files??
                            continue;
                            let linked_file = &files[linked_item.clone() as usize];
                            file.seek(SeekFrom::Start(linked_file.entry_offset as u64 + 8)); // +8 to skip the fourcc

                            let mut stxt_buffer = vec![0u8; linked_file.entry_length as usize];
                            file.read_exact(&mut stxt_buffer);
                            let _stxt_cursor = Cursor::new(stxt_buffer);
                        }
                    }
                }
            }
        }
        all_metadata
            .metadatafiles
            .insert(dir.to_string(), mulle_library);
        // mulle_library
    } // None
}

fn decode_direct_palette_image(
    bitmap_meta: &MacromediaCastBitmapMetadata,
    is_opaque: bool,
    img_cursor: &mut Cursor<Vec<u8>>,
) -> Vec<u8> {
    let mut rgba_data = Vec::<u8>::with_capacity(
        ((bitmap_meta.image_height as i32 * bitmap_meta.image_width as i32) * 4) as usize,
    );

    let mut pixel_written = 0;

    let stride = ((bitmap_meta.image_width * 8 + 7) / 8) as i32; // possibly pointless

    let mut x_pix: i32 = 0;

    while pixel_written < (bitmap_meta.image_height as i32 * bitmap_meta.image_width as i32) {
        //TODO unify this with 8bit_decode and split off linescan
        let val = 0xFF - img_cursor.read_u8().unwrap() as u32;

        // convert to RGBA
        let (r, g, b) = (
            PALETTE_MAC[(val * 3) as usize],
            PALETTE_MAC[((val * 3) + 1) as usize],
            PALETTE_MAC[((val * 3) + 2) as usize],
        );
        let mut alpha: u8 = 0xff;
        if !is_opaque && val == 255 as u32 {
            alpha = 0x00;
        }
        if x_pix >= 0 {
            rgba_data.push(r);
            rgba_data.push(g);
            rgba_data.push(b);
            rgba_data.push(alpha);
            pixel_written += 1;
        }
    }
    rgba_data
}

pub fn decode_8bit_image(
    bitmap_meta: &MacromediaCastBitmapMetadata,
    is_opaque: bool,
    img_cursor: &mut Cursor<Vec<u8>>,
) -> Vec<u8> {
    let mut rgba_data = Vec::<u8>::with_capacity(
        ((bitmap_meta.image_height as i32 * bitmap_meta.image_width as i32) * 4) as usize,
    );

    let mut pixel_written = 0;

    let stride = ((bitmap_meta.image_width * 8 + 7) / 8) as i32; // possibly pointless

    let mut x_pix: i32 = 0;

    while pixel_written < (bitmap_meta.image_height as i32 * bitmap_meta.image_width as i32) {
        let byte = match img_cursor.read_u8() {
            Err(_) => {
                eprint!("sdsd");
                break;
                0
            }
            Ok(byte) => byte,
        } as i16;

        // we want Rgba8Uint data
        // looks like this per pixel: 0x00 0xFF 0XFF 0xFF

        if bitmap_meta.image_bit_depth == 32 {
            // do something
        } else {
            if 0x100 - byte > 127 {
                // lle mode
                for _j in 0..(byte + 1) {
                    let val = 0xFF
                        - match img_cursor.read_u8() {
                            Err(_) => {
                                break;
                                0
                            }
                            Ok(byte) => byte,
                        } as u32;

                    // convert to RGBA
                    let (r, g, b) = (
                        PALETTE_MAC[(val * 3) as usize],
                        PALETTE_MAC[((val * 3) + 1) as usize],
                        PALETTE_MAC[((val * 3) + 2) as usize],
                    );
                    let mut alpha: u8 = 0xff;
                    if !is_opaque && val == 255 as u32 {
                        alpha = 0x00;
                    }
                    if x_pix >= 0 {
                        rgba_data.push(r);
                        rgba_data.push(g);
                        rgba_data.push(b);
                        rgba_data.push(alpha);
                        pixel_written += 1;
                    }

                    x_pix += 1;

                    if x_pix >= stride {
                        x_pix = 0;
                        if bitmap_meta.image_width % 2 != 0 {
                            // destroy a single byte after each column for widths not-divisible-by-2
                            x_pix = -1;
                        }
                    }
                }
            } else {
                // rle mode
                let val = 0xFF
                    - match img_cursor.read_u8() {
                        Err(_) => {
                            break;
                            0
                        }
                        Ok(byte) => byte,
                    } as u32;
                for _j in 0..(0x101 - byte) {
                    let (r, g, b) = (
                        PALETTE_MAC[(val * 3) as usize],
                        PALETTE_MAC[((val * 3) + 1) as usize],
                        PALETTE_MAC[((val * 3) + 2) as usize],
                    );
                    let mut alpha: u8 = 0xff;
                    if !is_opaque && val == 255 as u32 {
                        alpha = 0x00;
                    }

                    if x_pix >= 0 {
                        rgba_data.push(r);
                        rgba_data.push(g);
                        rgba_data.push(b);
                        rgba_data.push(alpha);
                        pixel_written += 1;
                    }
                    x_pix += 1;
                    if x_pix >= stride {
                        x_pix = 0;
                        if bitmap_meta.image_width % 2 != 0 {
                            // destroy a single byte after each column for widths not-divisible-by-2
                            x_pix = -1;
                            break;
                        }
                    }
                }
            }
        }
    }
    rgba_data
}

enum Endianness {
    Little,
    Big,
}

#[derive(Clone, Deserialize)]
struct MacromediaSubFile {
    entry_type: [u8; 4], // CP1252 encoded string
    entry_length: u32,
    entry_offset: u32,
    unknown1: u32,
    // 3072 for 0 length, 0 offset
    // 1024 for 0 length, any offset
    // 0 for any length any offset
    index: u32, // index is only populated for 0 length
}

#[derive(Clone, Deserialize, Serialize)]
struct MacromediaFileHeader {
    file_size: u32,
    file_sign: [u8; 4], // CP1252 string
    imap: [u8; 4],      // CP1252 string
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
    linked_entries: Vec<u32>,
}

#[derive(Clone, Deserialize)]
pub struct MacromediaCastBitmapMetadata {
    v27: u16,
    pub image_pos_y: i16,
    pub image_pos_x: i16,
    pub image_height: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    pub image_width: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    alpha_treshold: u16,
    _ole1: u32,
    _ole2: u16,
    pub image_reg_y: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    pub image_reg_x: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    // possibly the data ends here if it's 1-but, but is it padded to fit?
    image_bit_depth: u8,
    flags: u8,
    _image_palette: u32,
}

fn reversed_cp1252_array_to_string(array: &[u8; 4]) -> String {
    let mut reversed = [0u8; 4];
    reversed.copy_from_slice(array);
    reversed.reverse();
    CP1252.decode(&reversed).to_string()
}

#[derive(Resource, Default)]
pub struct MulleAssetHelp {
    metadatafiles: HashMap<String, MulleLibrary>,
}

struct MulleLibrary {
    name: String, //TODO fix name
    files: HashMap<u32, MulleFile>,
}

// struct MulleFile {
//     name: String,
//     mulle_type: MulleFileType,
//     mulle_image: MulleImage,
// }

pub trait Named {
    fn name(&self) -> String;
}

#[derive(Clone)]
pub enum MulleFile {
    MulleImage(MulleImage),
    MulleText(MulleText),
    MulleDB(MulleDBHolder),
}
#[derive(Clone)]
pub struct MulleImage {
    name: String,
    pub bitmap_metadata: MacromediaCastBitmapMetadata,
    pub image: Handle<Image>,
}

#[derive(Clone)]
pub struct MulleDBHolder {
    name: String,
    pub db: MulleDB
}

impl Named for MulleFile {
    fn name(&self) -> String {
        match self {
            MulleFile::MulleImage(image) => image.name.clone(),
            MulleFile::MulleText(text) => text.name.clone(),
            MulleFile::MulleDB(db) => db.name.clone(),
        }
    }
}


#[derive(Clone)]
pub struct MulleText {
    name: String,
    pub text: String,
}

#[derive(PartialEq)]
enum MulleFileType {
    Bitmap,
    Sound,
    Text,
}
