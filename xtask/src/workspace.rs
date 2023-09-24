use crate::readme::Readme;
use duct::cmd;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::Table;

type DynError = Box<dyn Error>;

const CRATES_DIRNAME: &str = "crates";
const TMP_DIRNAME: &str = "tmp";
const COVERAGE_DIRNAME: &str = "coverage";
const CARGO_TOML: &str = "Cargo.toml";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
    pub readme: Readme,
}

impl Crate {
    pub fn new<T: AsRef<str>>(name: T, path: PathBuf) -> Self {
        let name = name.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        Crate { name, path, readme }
    }

    pub fn tmp_path(&self) -> PathBuf {
        self.path.join(TMP_DIRNAME)
    }

    pub fn coverage_path(&self) -> PathBuf {
        self.tmp_path().join(COVERAGE_DIRNAME)
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.path.join(CARGO_TOML)
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Workspace {
    pub path: PathBuf,
    pub cargo: String,
    pub readme: Readme,
}

impl Workspace {
    pub fn new<T: AsRef<str>>(cargo: T) -> Self {
        let path = root_path(&cargo).unwrap();
        let cargo = cargo.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        Workspace {
            cargo,
            path,
            readme,
        }
    }

    pub fn tmp_path(&self) -> PathBuf {
        self.path.join(TMP_DIRNAME)
    }

    pub fn coverage_path(&self) -> PathBuf {
        self.tmp_path().join(COVERAGE_DIRNAME)
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.path.join(CARGO_TOML)
    }

    pub fn crates(&self) -> Result<BTreeMap<String, Crate>, DynError> {
        let crates_dir = self.path.join(CRATES_DIRNAME);
        let mut crates = BTreeMap::new();

        for entry in fs::read_dir(crates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().into_string().unwrap();
                crates.insert(name.clone(), Crate::new(name, path));
            }
        }

        Ok(crates)
    }

    pub fn clean(&self) -> Result<(), DynError> {
        fs::remove_dir_all(self.tmp_path())?;
        let crates = self.crates()?;
        for c in crates.values() {
            c.clean()?;
        }

        Ok(())
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        fs::create_dir_all(self.coverage_path())?;
        let crates = self.crates()?;
        for c in crates.values() {
            c.create_dirs()?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn manifest(&self) -> Result<Table, DynError> {
        let toml_path = self.manifest_path();
        let toml = fs::read_to_string(toml_path)?;
        Ok(toml.parse::<Table>()?)
    }
}

// UTILS //////////////////////////////////////////////////////////////////////
fn root_path<T: AsRef<str>>(cargo: T) -> Result<PathBuf, DynError> {
    let stdout = cmd!(
        cargo.as_ref().to_owned(),
        "locate-project",
        "--workspace",
        "--message-format",
        "plain",
    )
    .read()?;

    Ok(PathBuf::from(stdout.replace("Cargo.toml", "").trim()))
}
