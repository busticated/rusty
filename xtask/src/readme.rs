use crate::krate::Krate;
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

    pub fn from_path(crate_root: PathBuf) -> Result<Self, DynError> {
        let mut readme = Readme::new(crate_root);
        readme.load()
    }

    pub fn read(&self) -> Result<String, DynError> {
        Ok(fs::read_to_string(&self.path)?)
    }

    pub fn load(&mut self) -> Result<Self, DynError> {
        self.text = self.read()?;
        Ok(self.clone())
    }

    pub fn create<N: AsRef<str>, D: AsRef<str>>(
        &self,
        name: N,
        description: D,
    ) -> Result<(), DynError> {
        let text = format!(
            "# {}\n\n{}\n\n## Installation\n\n```shell\ncargo add {}\n```\n",
            name.as_ref(),
            description.as_ref(),
            name.as_ref(),
        );
        self.save(text)
    }

    pub fn save(&self, data: String) -> Result<(), DynError> {
        Ok(fs::write(&self.path, data)?)
    }

    pub fn update_crates_list(
        &mut self,
        mut krates: BTreeMap<String, Krate>,
    ) -> Result<(), DynError> {
        self.load()?;
        let marker_start = "<!-- crate-list-start -->";
        let marker_end = "<!-- crate-list-end -->";
        let mut entries = String::from(marker_start);
        let ptn = format!(r"{}[\s\S]*?{}", marker_start, marker_end);
        let re = RegexBuilder::new(ptn.as_str())
            .case_insensitive(true)
            .multi_line(true)
            .build()?;

        for krate in krates.values_mut() {
            krate.toml.load()?;
            let name = krate.toml.get_name()?;
            let description = krate.toml.get_description()?;
            let entry = format!("\n* [{}](crates/{})\n\t* {}", name, name, description);
            entries.push_str(&entry);
        }

        entries.push('\n');
        entries.push_str(marker_end);
        let updated = re.replace(&self.text, &entries);
        self.save(updated.as_ref().to_owned())
    }
}
