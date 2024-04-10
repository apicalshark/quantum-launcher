use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FabricJSON {
    pub mainClass: String,
    pub arguments: Arguments,
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    pub jvm: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub name: String,
    pub url: String,
}

impl Library {
    pub fn get_path(&self) -> String {
        let parts: Vec<&str> = self.name.split(':').collect();
        format!(
            "{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}-{}{MAIN_SEPARATOR}{}.jar",
            parts[0].replace('.', MAIN_SEPARATOR_STR),
            parts[1],
            parts[2],
            parts[1],
            parts[2],
            parts[0].replace(' ', "_")
        )
    }
}
