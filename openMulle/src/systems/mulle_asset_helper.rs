use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use serde_json;

use bevy::prelude::*;

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
}

fn parse_meta(mut allMetadata: ResMut<MulleAssetHelp>) {
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
                    allMetadata
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
    pub castType: u32,
    pub name: String,
    pub imagePosY: Option<i32>,
    pub imagePosX: Option<i32>,
    pub imageHeight: Option<u32>,
    pub imageWidth: Option<u32>,
    pub imageRegY: Option<i32>,
    pub imageRegX: Option<i32>,
    pub imageBitAlpha: Option<u32>,
    pub imageBitDepth: Option<u32>,
    pub imagePalette: Option<u32>,
    pub imageHash: Option<i128>,
    #[serde(skip)]
    pub soundCuePoints: Option<Vec<Vec<(u32, String)>>>,
    pub soundLooped: Option<bool>,
    pub soundLength: Option<u32>,
    pub soundSampleRate: Option<u32>,
}
