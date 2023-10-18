use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::fs::FS;
use crate::krate::Krate;

type DynError = Box<dyn Error>;

const README_MD: &str = "README.md";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Readme {
    pub path: PathBuf,
    text: String,
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
        // TODO (busticated): pull into FS wrapper?
        Ok(fs::read_to_string(&self.path)?)
    }

    pub fn load(&mut self) -> Result<Self, DynError> {
        self.text = self.read()?;
        Ok(self.clone())
    }

    pub fn create(&mut self, fs: &FS, krate: &Krate) -> Result<(), DynError> {
        self.text = self.render(&krate.name, &krate.description);
        self.save(fs)
    }

    pub fn save(&self, fs: &FS) -> Result<(), DynError> {
        Ok(fs.write(&self.path, &self.text)?)
    }

    pub fn render<N: AsRef<str>, D: AsRef<str>>(&self, name: N, description: D) -> String {
        let name = name.as_ref();
        let description = description.as_ref();
        let lines = vec![
            format!("# {}", name),
            "".to_string(),
            format!("[![Latest Version](https://img.shields.io/crates/v/{}.svg)](https://crates.io/crates/{})", name, name),
            format!("[![Documentation](https://docs.rs/{}/badge.svg)](https://docs.rs/{})", name, name),
            "[![CI Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions)".to_string(),
            "".to_string(),
            format!("{}", description),
            "".to_string(),
            "## Installation".to_string(),
            "".to_string(),
            "```shell".to_string(),
            format!("cargo add {}", name),
            "```".to_string(),
        ];
        lines.join("\n")
    }

    pub fn update_crates_list(
        &mut self,
        fs: &FS,
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
        self.text = updated.as_ref().to_owned();
        self.save(fs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let fake_crate_root = PathBuf::from("fake-crate-root");
        let readme = Readme::new(fake_crate_root);
        assert_eq!(readme.text, "");
        assert_eq!(readme.path, PathBuf::from("fake-crate-root/README.md"));
    }

    #[test]
    fn it_renders() {
        let fake_crate_root = PathBuf::from("fake-crate-root");
        let readme = Readme::new(fake_crate_root);
        assert_eq!(
            readme.render("my-crate", "my-crate description"),
            [
                "# my-crate",
                "",
                "[![Latest Version](https://img.shields.io/crates/v/my-crate.svg)](https://crates.io/crates/my-crate)",
                "[![Documentation](https://docs.rs/my-crate/badge.svg)](https://docs.rs/my-crate)",
                "[![CI Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions)",
                "",
                "my-crate description",
                "",
                "## Installation",
                "",
                "```shell",
                "cargo add my-crate",
                "```",
            ].join("\n")
        );
    }
}
