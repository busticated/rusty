use crate::fs::FS;
use crate::krate::Krate;
use regex::RegexBuilder;
use semver::Version;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

const CHANGELOG_MD: &str = "CHANGELOG.md";
const MARKER_START: &str = "<!-- next-version-start -->";
const MARKER_END: &str = "<!-- next-version-end -->";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Changelog {
    pub path: PathBuf,
    text: String,
}

impl Changelog {
    pub fn new(crate_root: PathBuf) -> Self {
        Changelog {
            text: String::new(),
            path: crate_root.join(CHANGELOG_MD),
        }
    }

    pub fn from_path(crate_root: PathBuf) -> Result<Self, DynError> {
        let mut changelog = Changelog::new(crate_root);
        changelog.load()
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
        self.text = self.render(&krate.name, &krate.version);
        self.save(fs)
    }

    pub fn save(&self, fs: &FS) -> Result<(), DynError> {
        Ok(fs.write(&self.path, &self.text)?)
    }

    pub fn render<N: AsRef<str>>(&self, name: N, version: &Version) -> String {
        let name = name.as_ref();
        let lines = vec![
            format!("# `{}` Changelog", name),
            MARKER_START.to_string(),
            MARKER_END.to_string(),
            format!("## v{}", version),
            "".to_string(),
            "* Initial release 🎊🎉".to_string(),
            "".to_string(),
        ];
        lines.join("\n")
    }

    pub fn update(&mut self, fs: &FS, krate: &Krate, log: Vec<String>) -> Result<(), DynError> {
        if log.is_empty() {
            return Ok(());
        }
        self.load()?;
        let mut changes = format!("{}\n{}\n", MARKER_START, MARKER_END);
        changes.push_str(format!("## v{}\n\n", &krate.version).as_str());
        for msg in log.iter() {
            if !msg.is_empty() {
                changes.push_str(format!("* {}\n", &msg).as_str());
            }
        }
        changes.push('\n');
        let ptn = format!(r"{}[\s\S]*?{}", MARKER_START, MARKER_END);
        let re = RegexBuilder::new(ptn.as_str())
            .case_insensitive(true)
            .multi_line(true)
            .build()?;
        let updated = re.replace(&self.text, &changes);
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
        let changelog = Changelog::new(fake_crate_root);
        assert_eq!(changelog.text, "");
        assert_eq!(
            changelog.path,
            PathBuf::from("fake-crate-root/CHANGELOG.md")
        );
    }

    #[test]
    fn it_renders() {
        let fake_crate_root = PathBuf::from("fake-crate-root");
        let version = Version::new(0, 1, 0);
        let changelog = Changelog::new(fake_crate_root);
        assert_eq!(
            changelog.render("my-crate", &version),
            [
                "# `my-crate` Changelog",
                "<!-- next-version-start -->",
                "<!-- next-version-end -->",
                "## v0.1.0",
                "",
                "* Initial release 🎊🎉",
                "",
            ]
            .join("\n")
        );
    }
}
