use crate::readme::Readme;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::Table;

type DynError = Box<dyn Error>;

const TMP_DIRNAME: &str = "tmp";
const COVERAGE_DIRNAME: &str = "coverage";
const CARGO_TOML: &str = "Cargo.toml";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Krate {
    pub name: String,
    pub path: PathBuf,
    pub readme: Readme,
}

impl KratePaths for Krate {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Krate {
    pub fn new<T: AsRef<str>>(name: T, path: PathBuf) -> Self {
        let name = name.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        Krate { name, path, readme }
    }

    pub fn clean(&self) -> Result<(), DynError> {
        Ok(fs::remove_dir_all(self.tmp_path())?)
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        Ok(fs::create_dir_all(self.coverage_path())?)
    }

    pub fn manifest(&self) -> Result<Table, DynError> {
        let toml_path = self.manifest_path();
        let toml = fs::read_to_string(toml_path)?;
        Ok(toml.parse::<Table>()?)
    }
}

pub trait KratePaths {
    fn path(&self) -> PathBuf;

    fn tmp_path(&self) -> PathBuf {
        self.path().join(TMP_DIRNAME)
    }

    fn coverage_path(&self) -> PathBuf {
        self.tmp_path().join(COVERAGE_DIRNAME)
    }

    fn manifest_path(&self) -> PathBuf {
        self.path().join(CARGO_TOML)
    }
}
