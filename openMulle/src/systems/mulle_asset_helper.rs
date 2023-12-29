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
