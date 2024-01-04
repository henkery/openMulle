use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    mem::size_of,
};

use bevy::render::render_resource::{Extent3d, TextureFormat};

use byteorder::ReadBytesExt;
use yore::code_pages::CP1252;

use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

use bevy::prelude::*;

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
            "CDDATA.cxt".to_string(),
            Vec::from([
                629, 630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 640, 641, 642, 643, 644,
                645, 646, 647, 648, 649, 650, 651, 652, 653, 654, 656, 657, 658
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
            .add_systems(Startup, parse_meta);
    }
}

pub trait MulleAssetHelper {
    fn get_image_by_name(&self, dir: String, name: u32) -> Option<&Handle<Image>>;
    fn get_mulle_image_by_name(&self, dir: String, name: u32) -> Option<&MulleImage>;
    // fn find_member(&self, dir: &str, name: &str) -> Option<&Member>;
    // fn find_member_path(&self, dir: &str, name: &str, file_ext: &str) -> Option<PathBuf>;
    // fn find_member_path_from_actor_name(
    //     &self,
    //     dir: &str,
    //     name: &str,
    //     file_ext: &str,
    // ) -> Option<PathBuf>;
    // fn find_member_path_with_asset(&self, dir: &str, name: &str, file_ext: &str)
    //     -> Option<PathBuf>;
    // fn find_member_path_with_asset_from_actor_name(
    //     &self,
    //     dir: &str,
    //     name: &str,
    //     file_ext: &str,
    // ) -> Option<PathBuf>;
}

impl MulleAssetHelper for MulleAssetHelp {
    fn get_image_by_name(&self, dir: String, name: u32) -> Option<&Handle<Image>> {
        if let Some(mulle_image) = self.get_mulle_image_by_name(dir, name) {
            return Some(&mulle_image.image);
        }
        None
    }
    fn get_mulle_image_by_name(&self, dir: String, name: u32) -> Option<&MulleImage> {
        if let Some(mulle_library) = self.metadatafiles.get(&dir) {
            if let Some(mulle_file) = mulle_library.files.get(&name) {
                if mulle_file.mulle_type == MulleFileType::Bitmap {
                    return Some(&mulle_file.mulle_image);
                }
            }
        }
        None
    }
    // fn find_member(&self, dir: &str, name: &str) -> Option<&Member> {
    //     // All "dir" here is lowercase!
    //     match self.metadatafiles.get(dir) {
    //         Some(metakey) => {
    //             for library in &metakey.libraries {
    //                 return library.members.get(name);
    //             }
    //             None
    //         }
    //         None => None,
    //     }
    // }

    // fn find_member_path(&self, dir: &str, name: &str, file_ext: &str) -> Option<PathBuf> {
    //     // All "dir" here is lowercase!

    //     //TODO make file_ext automatically resolve
    //     match self.metadatafiles.get(dir) {
    //         Some(metakey) => {
    //             for library in &metakey.libraries {
    //                 let path = format!(
    //                     "cst_out_new/{}/{}/{}{}",
    //                     metakey.dir, library.name, name, file_ext
    //                 );
    //                 return Some(PathBuf::from(path));
    //             }
    //             None
    //         }
    //         None => None,
    //     }
    // }
    // fn find_member_path_with_asset(
    //     &self,
    //     dir: &str,
    //     name: &str,
    //     file_ext: &str,
    // ) -> Option<PathBuf> {
    //     // All "dir" here is lowercase!

    //     //TODO make file_ext automatically resolve
    //     match self.metadatafiles.get(dir) {
    //         Some(metakey) => {
    //             for library in &metakey.libraries {
    //                 let path = format!(
    //                     "assets/cst_out_new/{}/{}/{}{}",
    //                     metakey.dir, library.name, name, file_ext
    //                 );
    //                 return Some(PathBuf::from(path));
    //             }
    //             None
    //         }
    //         None => None,
    //     }
    // }
    // fn find_member_path_with_asset_from_actor_name(
    //     &self,
    //     dir: &str,
    //     name: &str,
    //     file_ext: &str,
    // ) -> Option<PathBuf> {
    //     // All "dir" here is lowercase!

    //     //TODO make file_ext automatically resolve
    //     // if let Some(metakey) = self.metadatafiles.get(dir) {
    //     //     for library in &metakey.libraries {
    //     //         for (member_name, member) in &library.members {
    //     //             // member.name and member_name ARE NOT THE SAME THING
    //     //             //TODO make this sane
    //     //             if member.name == name {
    //     //                 let path = format!(
    //     //                     "assets/cst_out_new/{}/{}/{}{}",
    //     //                     metakey.dir, library.name, member_name, file_ext
    //     //                 );
    //     //                 return Some(PathBuf::from(path));
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     None
    // }
    // fn find_member_path_from_actor_name(
    //     &self,
    //     dir: &str,
    //     name: &str,
    //     file_ext: &str,
    // ) -> Option<PathBuf> {
    //     // All "dir" here is lowercase!

    //     //TODO make file_ext automatically resolve
    //     // if let Some(metakey) = self.metadatafiles.get(dir) {
    //     //     for library in &metakey.libraries {
    //     //         for (member_name, member) in &library.members {
    //     //             // member.name and member_name ARE NOT THE SAME THING
    //     //             //TODO make this sane
    //     //             if member.name == name {
    //     //                 let path = format!(
    //     //                     "cst_out_new/{}/{}/{}{}",
    //     //                     metakey.dir, library.name, member_name, file_ext
    //     //                 );
    //     //                 return Some(PathBuf::from(path));
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     None
    // }
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

                let castar_entry_type_raw = match &endian {
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

                let castar_entry_length = match &endian {
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
                            "Lctx", "FXmp", "Cinf", "MCsL", "Sord", "VWCF", "VWFI", "VWLB", "VWSC",
                            "Fmap", "SCRF", "DRCF", "VWFM", "VWtk",
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

        let mut header_buffer = [0u8; size_of::<MacromediaCastEntryHeader>()];

        let mut cast_members = Vec::<(u32, u32)>::new(); // These should be only one member list per library?

        for (index, cast_library) in &cast_libraries_map {
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

        for (num, slot) in &cast_members {
            let subfile = &files[slot.clone() as usize];
            file.seek(SeekFrom::Start(subfile.entry_offset.into()));
            let cast_member_preheader: MacromediaCastEntryHeader = MacromediaCastEntryHeader {
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
            let cast_memer_cast_end_data_length = file.read_u32::<byteorder::BigEndian>().unwrap();

            file.seek(SeekFrom::Current(cast_member_cast_data_length.into()));

            if cast_member_cast_type == 1 {
                let unknown1 = file.read_u16::<byteorder::BigEndian>().unwrap(); //ignoring endianness for unknowns...

                let image_pos_y = file.read_i16::<byteorder::BigEndian>().unwrap(); // these are always BE for some reason
                let image_pos_x = file.read_i16::<byteorder::BigEndian>().unwrap();

                bitmap_meta.insert(
                    slot.clone(),
                    MacromediaCastBitmapMetadata {
                        //image struct is always BE!
                        unknown1: unknown1,
                        image_pos_y: image_pos_y,
                        image_pos_x: image_pos_x,
                        image_height: file.read_i16::<byteorder::BigEndian>().unwrap()
                            - image_pos_y,
                        image_width: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                        _garbage: file.read_u64::<byteorder::BigEndian>().unwrap(), // ignoring endianness for garbage
                        image_reg_y: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_y,
                        image_reg_x: file.read_i16::<byteorder::BigEndian>().unwrap() - image_pos_x,
                        image_bit_is_opaque: file.read_u8().unwrap(), // it remains unclear
                        image_bit_depth: file.read_u8().unwrap(), // possibly just u8 but only the second nibble?
                        image_bit_alpha: file.read_u8().unwrap(),
                        unknown2: file.read_u8().unwrap(),
                        _image_palette: file.read_u16::<byteorder::BigEndian>().unwrap(),
                    },
                );
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
                    let cast_member_preheader: MacromediaCastEntryHeader =
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
                    let cast_member_cast_data_length =
                        file.read_u32::<byteorder::BigEndian>().unwrap(); // this one is always BE?
                    let cast_memer_cast_end_data_length = match &endian {
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

                                let unknown1 = file.read_u64::<byteorder::BigEndian>().unwrap(); //ignoring endianness of unknown values

                                let bitmap_meta = bitmap_meta.get(slot).unwrap();

                                let mut pad = 0;
                                if bitmap_meta.image_width % 2 != 0 {
                                    // if image width is divisible by 2 pad equals image height?
                                    pad = bitmap_meta.image_height;
                                }

                                if bitmap_meta.image_bit_depth > 32 {
                                    // bit field mode
                                } else if ((bitmap_meta.image_width + bitmap_meta.image_height)
                                    + pad) as u32
                                    == linked_file.entry_length
                                {
                                    // direct palette mode?
                                } else {
                                    // other mode??
                                    let mut rgba_data = Vec::<u8>::with_capacity(
                                        ((bitmap_meta.image_height as i32
                                            * bitmap_meta.image_width as i32)
                                            * 4) as usize,
                                    );

                                    let mut pixel_written = 0;

                                    let mut x_pix = 0;

                                    let is_opaque = if let Some(numvec) = OPAQUE.get(*dir) {
                                        numvec.contains(num)
                                    } else {
                                        false
                                    }; // is this expensive?

                                    while pixel_written
                                        < (bitmap_meta.image_height as i32
                                            * bitmap_meta.image_width as i32)
                                    {
                                        let byte = match file.read_u8() {
                                            Ok(val) => val,
                                            Err(lerror) => {
                                                eprint!(
                                                    "failed to read! {} too few bytes! bailing...",
                                                    file.stream_position().unwrap()
                                                );
                                                break;
                                            }
                                        } as u16;

                                        // we want Rgba8Uint data
                                        // looks like this per pixel: 0x00 0xFF 0XFF 0xFF

                                        if bitmap_meta.image_bit_depth == 32 {
                                            // do something
                                        } else {
                                            if 0x100 - byte > 127 {
                                                // lle mode
                                                for j in 0..(byte + 1) {
                                                    let val = 0xFF
                                                        - match file.read_u8() {
                                                            Ok(val) => val,
                                                            Err(lerror) => {
                                                                eprint!("failed to read! {} too few bytes!", file.stream_position().unwrap());
                                                                break;
                                                            }
                                                        }
                                                            as u32;

                                                    // convert to RGBA
                                                    let (r, g, b) = (
                                                        PALETTE_MAC[(val * 3) as usize],
                                                        PALETTE_MAC[((val * 3) + 1) as usize],
                                                        PALETTE_MAC[((val * 3) + 2) as usize],
                                                    );
                                                    let mut alpha: u8 = 0xff;
                                                    if !is_opaque
                                                        && val == bitmap_meta.image_bit_alpha as u32
                                                    {
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

                                                    if x_pix >= bitmap_meta.image_width {
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
                                                    - match file.read_u8() {
                                                        Ok(val) => val,
                                                        Err(lerror) => {
                                                            eprint!(
                                                                "failed to read! {} too few bytes!",
                                                                file.stream_position().unwrap()
                                                            );
                                                            break;
                                                        }
                                                    }
                                                        as u32;
                                                for j in 0..(0x101 - byte) {
                                                    let (r, g, b) = (
                                                        PALETTE_MAC[(val * 3) as usize],
                                                        PALETTE_MAC[((val * 3) + 1) as usize],
                                                        PALETTE_MAC[((val * 3) + 2) as usize],
                                                    );
                                                    let mut alpha: u8 = 0xff;
                                                    if !is_opaque
                                                        && val == bitmap_meta.image_bit_alpha as u32
                                                    {
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
                                                    if x_pix >= bitmap_meta.image_width {
                                                        x_pix = 0;
                                                        if bitmap_meta.image_width % 2 != 0 {
                                                            // destroy a single byte after each column for widths not-divisible-by-2
                                                            x_pix = -1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if (rgba_data.len()
                                        != ((bitmap_meta.image_height as i32
                                            * bitmap_meta.image_width as i32)
                                            * 4)
                                            as usize)
                                    {
                                        eprint!("file size error for {}, amount of bytes was {} expected {}", num, rgba_data.len(), ((bitmap_meta.image_height as i32*bitmap_meta.image_width as i32) * 4));
                                        if (rgba_data.len()
                                            > ((bitmap_meta.image_height as i32
                                                * bitmap_meta.image_width as i32)
                                                * 4)
                                                as usize)
                                        {
                                            eprint!(" dumping excess pixels, see what happens");
                                            rgba_data = rgba_data[0..((bitmap_meta.image_height
                                                as i32
                                                * bitmap_meta.image_width as i32)
                                                * 4)
                                                as usize]
                                                .to_vec();
                                        }
                                    }
                                    mulle_library.files.insert(num.clone(), MulleFile {
                                        mulle_type: MulleFileType::Bitmap,
                                        mulle_image: MulleImage {
                                            bitmap_metadata: bitmap_meta.clone(),
                                            image: images.add(Image::new(
                                                Extent3d {
                                                    width: bitmap_meta.image_width as u32,
                                                    height: bitmap_meta.image_height as u32,
                                                    depth_or_array_layers: 1
                                                },
                                                bevy::render::render_resource::TextureDimension::D2,
                                                rgba_data,
                                                TextureFormat::Rgba8UnormSrgb
                                            )),
                                        }
                                    });
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
        all_metadata
            .metadatafiles
            .insert(dir.to_string(), mulle_library);
        // mulle_library
    } // None
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

struct MacromediaCastMember {
    entry_type: String,
    entry_length: u32,
    data: Vec<u8>,
}

#[derive(Clone, Deserialize)]
pub struct MacromediaCastBitmapMetadata {
    unknown1: u16,
    pub image_pos_y: i16,
    pub image_pos_x: i16,
    pub image_height: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    pub image_width: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    _garbage: u64,
    pub image_reg_y: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    pub image_reg_x: i16, // appearently you need to subtract the pos elements of these to get the correct value?
    // possibly the data ends here if it's 1-but, but is it padded to fit?
    image_bit_depth: u8,
    image_bit_is_opaque: u8,
    image_bit_alpha: u8,
    unknown2: u8,
    _image_palette: u16,
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

struct MulleFile {
    mulle_type: MulleFileType,
    mulle_image: MulleImage,
}

#[derive(Clone)]
pub struct MulleImage {
    pub bitmap_metadata: MacromediaCastBitmapMetadata,
    pub image: Handle<Image>,
}

#[derive(PartialEq)]
enum MulleFileType {
    Bitmap,
    Sound,
    Text,
}
