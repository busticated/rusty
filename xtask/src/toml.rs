use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;
use semver::Version;

type DynError = Box<dyn Error>;

const CARGO_TOML: &str = "Cargo.toml";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Toml {
    pub path: PathBuf,
    data: Table,
}

impl Toml {
    pub fn new(crate_root: PathBuf) -> Self {
        Toml {
            path: crate_root.join(CARGO_TOML),
            ..Default::default()
        }
    }

    pub fn from_path(crate_root: PathBuf) -> Result<Self, DynError> {
        let mut toml = Toml::new(crate_root);
        toml.load()
    }

    pub fn read(&self) -> Result<Table, DynError> {
        let data = fs::read_to_string(&self.path)?;
        Ok(data.parse::<Table>()?)
    }

    pub fn load(&mut self) -> Result<Self, DynError> {
        self.data = self.read()?;
        Ok(self.clone())
    }

    pub fn create<N: AsRef<str>, D: AsRef<str>>(
        &self,
        name: N,
        description: D,
    ) -> Result<(), DynError> {
        self.save(self.render(name, description))
    }

    pub fn save(&self, data: String) -> Result<(), DynError> {
        Ok(fs::write(&self.path, data)?)
    }

    pub fn render<N: AsRef<str>, D: AsRef<str>>(&self, name: N, description: D) -> String {
        let name = name.as_ref();
        let description = description.as_ref();
        let lines = vec![
            "[package]".to_string(),
            format!("name = \"{}\"", name),
            format!("description = \"{}\"", description),
            "version = \"0.1.0\"".to_string(),
            "edition.workspace = true".to_string(),
            "license.workspace = true".to_string(),
            "authors.workspace = true".to_string(),
            "repository.workspace = true".to_string(),
            "".to_string(),
            "[dependencies]".to_string(),
        ];
        lines.join("\n")
    }

    pub fn get_version(&self) -> Result<Version, DynError> {
        let pkg = self
            .data
            .get("package")
            .ok_or(format_section_missing_msg("package", &self.path))?;
        let version = pkg
            .get("version")
            .ok_or(format_field_missing_msg("version", &self.path))?
            .as_str()
            .ok_or(format_invalid_field_msg("version", &self.path))?;

        Ok(Version::parse(version)?)
    }

    pub fn get_name(&self) -> Result<String, DynError> {
        let pkg = self
            .data
            .get("package")
            .ok_or(format_section_missing_msg("package", &self.path))?;
        let name = pkg
            .get("name")
            .ok_or(format_field_missing_msg("name", &self.path))?
            .as_str()
            .ok_or(format_invalid_field_msg("name", &self.path))?;

        Ok(name.to_string())
    }

    pub fn get_description(&self) -> Result<String, DynError> {
        let pkg = self
            .data
            .get("package")
            .ok_or(format_section_missing_msg("package", &self.path))?;
        let description = pkg
            .get("description")
            .ok_or(format_field_missing_msg("description", &self.path))?
            .as_str()
            .ok_or(format_invalid_field_msg("description", &self.path))?;

        Ok(description.to_string())
    }
}

// UTILS //////////////////////////////////////////////////////////////////////
fn format_section_missing_msg(field: &str, path: &Path) -> String {
    format!(
        "Error: toml is missing `{}` section! See: {}",
        field,
        path.display()
    )
}

fn format_field_missing_msg(field: &str, path: &Path) -> String {
    format!(
        "Error: toml is missing `{}` field! See: {}",
        field,
        path.display()
    )
}

fn format_invalid_field_msg(field: &str, path: &Path) -> String {
    format!(
        "Error: Could not convert `{}` to str! See: {}",
        field,
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let fake_crate_root = PathBuf::from("fake-crate-root");
        let toml = Toml::new(fake_crate_root);
        assert_eq!(toml.data.len(), 0);
        assert_eq!(toml.path, PathBuf::from("fake-crate-root/Cargo.toml"));
    }

    #[test]
    fn it_renders() {
        let fake_crate_root = PathBuf::from("fake-crate-root");
        let toml = Toml::new(fake_crate_root);
        assert_eq!(
            toml.render("my-crate", "my-crate description"),
            [
                "[package]",
                "name = \"my-crate\"",
                "description = \"my-crate description\"",
                "version = \"0.1.0\"",
                "edition.workspace = true",
                "license.workspace = true",
                "authors.workspace = true",
                "repository.workspace = true",
                "",
                "[dependencies]",
            ]
            .join("\n")
        );
    }
}
