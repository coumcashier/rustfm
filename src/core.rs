use std::{path::PathBuf, fs::{self, DirEntry}, fmt::Error, rc::Rc};

use async_std::fs::ReadDir;
use log::warn;

pub(crate) struct App {
    pub current_path: PathBuf,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut s = Self {
            current_path: PathBuf::from("./").canonicalize().unwrap(),
        };
        s
    }
    
    pub fn list_dir(&self)-> Result<std::fs::ReadDir, std::io::Error>{
        let dir_listing =  std::fs::read_dir(self.current_path.to_str().unwrap());
        dir_listing
    }


    pub fn parent_path(&self) -> PathBuf {
        let mut parent_path = self.current_path.clone();
        parent_path.pop();
        parent_path
    }

    pub(crate) fn change_path(&mut self, path: PathBuf) {
        if !path.is_absolute() {
            warn!("Cannot Change Path: Path is not absolute.");
            return;
        }
        if !path.exists() {
            warn!("Cannot Change Path: Path does not exist.");
            return;
        }
        if !path.is_dir() {
            warn!("Cannot Change Path: Path is not a directory.");
            return;
        }

        self.current_path = path;
    }

}