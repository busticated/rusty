use crate::workspace::Crate;
use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

const README_MD: &str = "README.md";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Readme {
    pub text: String,
    pub path: PathBuf,
}

impl Readme {
    pub fn new(crate_root: PathBuf) -> Self {
        Readme {
            text: String::new(),
            path: crate_root.join(README_MD),
        }
    }

    pub fn load(&mut self) -> Result<&Self, DynError> {
        self.text = fs::read_to_string(&self.path)?;
        Ok(self)
    }

    pub fn save(&self, data: String) -> Result<(), DynError> {
        Ok(fs::write(&self.path, data)?)
    }

    pub fn update_crates_list(&mut self, crates: BTreeMap<String, Crate>) -> Result<(), DynError> {
        self.load()?;
        let marker_start = "<!-- crate-list-start -->";
        let marker_end = "<!-- crate-list-end -->";
        let mut entries = String::from(marker_start);
        let ptn = format!(r"{}[\s\S]*?{}", marker_start, marker_end);
        let re = RegexBuilder::new(ptn.as_str())
            .case_insensitive(true)
            .multi_line(true)
            .build()?;

        for c in crates.values() {
            let manifest = c.manifest()?;
            let pkg = manifest
                .get("package")
                .ok_or("Cargo.toml is missing `package` section!")?;
            let name = pkg
                .get("name")
                .ok_or("Cargo.toml is missing `name` field!")?
                .as_str()
                .ok_or("Could not convert `name` to str")?;
            let description = pkg
                .get("description")
                .ok_or("Cargo.toml is missing `description` field!")?
                .as_str()
                .ok_or("Could not convert `description` to str")?;
            let entry = format!("\n* [{}](crates/{})\n\t* {}", name, name, description);
            entries.push_str(&entry);
        }

        entries.push('\n');
        entries.push_str(marker_end);
        let updated = re.replace(&self.text, &entries);
        self.save(updated.as_ref().to_owned())
    }
}
