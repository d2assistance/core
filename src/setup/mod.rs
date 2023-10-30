use keyvalues_parser::Vdf;
use std::{
    borrow::Cow,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

use self::libraryfolders_model::LibraryFolders;
use tasklist::Tasklist;
use thiserror::Error;

mod libraryfolders_model;

/// Create configurations for GSI and other things
pub struct Setup {}

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

    #[error("Can't copy config file")]
    CantCopyConfig,
}

const DOTA2_GAME_ID: &'static str = "570";
const CONFIG_FILENAME: &'static str = "gamestate_integration_dota-helper-2.cfg";

#[derive(Debug)]
enum ConfigStatus {
    EXIST,
    EMPTY,
}

impl Setup {
    pub fn run() -> Result<(), SetupError> {
        let status = Self::check_config()?;

        match status {
            ConfigStatus::EXIST => Ok(()),
            ConfigStatus::EMPTY => Self::create_configuration(),
        }?;

        Ok(())
    }

    fn check_config() -> Result<ConfigStatus, SetupError> {
        let dota2_folder = Self::get_dota2_folder()?;
        let path = Self::get_path_to_config(dota2_folder);
        let result = match path.try_exists() {
            Ok(result) => Ok(result),
            Err(_) => Err(SetupError::FileReadError),
        }?;

        Ok(match result {
            true => ConfigStatus::EXIST,
            false => ConfigStatus::EMPTY,
        })
    }

    fn create_configuration() -> Result<(), SetupError> {
        let dota2_folder = Self::get_dota2_folder()?;
        Self::write_gsi_config(dota2_folder)?;

        Ok(())
    }

    fn get_dota2_folder() -> Result<PathBuf, SetupError> {
        let libraryfolders_row = Self::open_libraryfolders()?;
        let libraryfolders = Self::parse_libraryfolders(libraryfolders_row)?;
        let dota2_folder: PathBuf = Self::find_dota2_folder(libraryfolders)?;

        Ok(dota2_folder)
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

        Ok(steam_library
            .join("steamapps")
            .join("common")
            .join("dota 2 beta"))
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

    fn write_gsi_config(dota2_folder: PathBuf) -> Result<(), SetupError> {
        let filename = Self::get_path_to_config(dota2_folder);

        match fs::copy("assets/config.cfg", filename) {
            Err(_) => Err(SetupError::CantCopyConfig),
            Ok(res) => Ok(res),
        }?;

        Ok(())
    }

    fn get_path_to_config(dota2_folder: PathBuf) -> PathBuf {
        dota2_folder
            .clone()
            .join("game")
            .join("dota")
            .join("cfg")
            .join("gamestate_integration")
            .join(CONFIG_FILENAME.to_string())
    }
}
