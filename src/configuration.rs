use std::{
    borrow::Cow,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    ql::Predicate,
    schedule::{self, Schedule, SkeletonEvent, TimePair},
    ui::{
        self,
        component::ErrorModal,
        widget::{UpcomingEventsWidgetState, WidgetManagerState},
        Theme,
    },
};
use dioxus::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::{fs, io};

pub const HOME_CONFIGURATION_FILE_NAME: &str = ".diesel";

#[derive(Default, Serialize, Deserialize)]
pub struct HomeConfiguration {
    configuration_path: Option<String>,
}

impl HomeConfiguration {
    pub fn read() -> io::Result<Option<Self>> {
        let str = fs::read_to_string(Self::path())?;
        Ok(serde_json::from_str(&str).ok())
    }

    pub fn create_if_does_not_exist(&self) -> io::Result<()> {
        use std::fs::OpenOptions;
        let json = serde_json::to_vec(self).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(Self::path())?;
        file.write(&json)?;
        Ok(())
    }

    pub fn path() -> PathBuf {
        let mut path = dirs::home_dir().expect("failed to get home directory");
        path.push(HOME_CONFIGURATION_FILE_NAME);
        path
    }
}
