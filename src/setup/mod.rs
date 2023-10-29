use keyvalues_parser::Vdf;
use std::{borrow::Cow, fs::read_to_string, path::{Path, PathBuf}};

use self::libraryfolders_model::LibraryFolders;
use tasklist::Tasklist;
use thiserror::Error;

mod libraryfolders_model;

pub struct Setup {
    folder: String,
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Steam process not found")]
    ProcessNotFound,
    #[error("Can't parse path to dota folder")]
    PathError,
    #[error("Can't read config file")]
    FileReadError,
    #[error("Can't parse config file")]
    FileParseError,
    #[error("Can't find Dota 2 folder")]
    Dota2FolderNotFound,
}

const DOTA2_GAME_ID: &'static str = "570";

impl Setup {
    pub fn create_configuration() -> Result<Self, SetupError> {
        let libraryfolders_row = Self::open_libraryfolders()?;
        let libraryfolders = Self::parse_libraryfolders(libraryfolders_row)?;
        let folder = Self::find_dota2_folder(libraryfolders)?;

        Ok(Setup { folder: ".display()".to_string() })
    }

    fn parse_libraryfolders(row: String) -> Result<LibraryFolders, SetupError> {
        let mut vdf = match Vdf::parse(row.as_str()) {
            Ok(result) => Ok(result),
            Err(_) => Err(SetupError::FileParseError),
        }?;
        let obj = vdf.value.get_mut_obj().unwrap();

        // Switch all the entries with keys that are an index (0, 1, ...) to `"libraries"`
        let mut index = 0;
        while let Some(mut library) = obj.remove(index.to_string().as_str()) {
            obj.entry(Cow::from("libraries"))
                .or_insert(Vec::new())
                .push(library.pop().unwrap());

            index += 1;
        }

        let libraryfolders: Result<LibraryFolders, keyvalues_serde::Error> =
            keyvalues_serde::from_vdf(vdf);

        match libraryfolders {
            Err(_) => Err(SetupError::FileParseError),
            Ok(result) => Ok(result),
        }
    }

    fn open_libraryfolders() -> Result<String, SetupError> {
        let steam_execute = Self::lookup_steam_folder().ok_or(SetupError::ProcessNotFound)?;
        let path = Path::new(steam_execute.as_str());
        let steam_folder = path.parent().ok_or(SetupError::PathError)?;
        let libraryfolders_path = steam_folder.join("config").join("libraryfolders.vdf");

        return match read_to_string(libraryfolders_path.display().to_string()) {
            Ok(result) => Ok(result),
            Err(_) => Err(SetupError::FileReadError),
        };
    }

    fn find_dota2_folder(libraryfolders: LibraryFolders) -> Result<PathBuf, SetupError> {
        let mut libraryfolders_iter = libraryfolders.libraries.iter();

        let library = libraryfolders_iter.find(|item| {
            item.apps
                .iter()
                .find(|item| item.0 == DOTA2_GAME_ID)
                .is_some()
        });

        let steam_library = match library {
            Some(result) => Ok(result.clone().path),
            None => Err(SetupError::Dota2FolderNotFound),
        }?;
        let steam_library = Path::new(&steam_library);

        Ok(steam_library.join("steamapps").join("common").join("dota 2 beta"))
    }

    fn lookup_steam_folder() -> Option<String> {
        for process in unsafe { Tasklist::new() } {
            let filename = match process.get_file_info().get("OriginalFilename") {
                Some(str) => str.to_string(),
                None => "".to_string(),
            };

            if filename == "steam.exe" {
                return Some(process.get_path());
            }
        }

        return None;
    }

    fn join_
}
