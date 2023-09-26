use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;

type DynError = Box<dyn Error>;

const CARGO_TOML: &str = "Cargo.toml";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Toml {
    path: PathBuf,
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
