use duct::cmd;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

const CRATES_DIRNAME: &str = "crates";
const TMP_DIRNAME: &str = "tmp";
const COVERAGE_DIRNAME: &str = "coverage";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
}

impl Crate {
    pub fn new<T: AsRef<str>>(name: T, path: PathBuf) -> Self {
        let name = name.as_ref().to_owned();
        Crate { name, path }
    }

    pub fn tmp(&self) -> PathBuf {
        self.path.join(TMP_DIRNAME)
    }

    pub fn coverage(&self) -> PathBuf {
        self.tmp().join(COVERAGE_DIRNAME)
    }

    pub fn clean(&self) -> Result<(), DynError> {
        Ok(fs::remove_dir_all(self.tmp())?)
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        Ok(fs::create_dir_all(self.coverage())?)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Workspace {
    pub cargo: String,
}

impl Workspace {
    pub fn new<T: AsRef<str>>(cargo: T) -> Self {
        Workspace {
            cargo: cargo.as_ref().to_owned(),
        }
    }

    pub fn root(&self) -> Result<PathBuf, DynError> {
        let stdout = cmd!(
            &self.cargo,
            "locate-project",
            "--workspace",
            "--message-format",
            "plain",
        )
        .read()?;

        Ok(PathBuf::from(stdout.replace("Cargo.toml", "").trim()))
    }

    pub fn tmp(&self) -> PathBuf {
        self.root().unwrap().join(TMP_DIRNAME)
    }

    pub fn coverage(&self) -> PathBuf {
        self.tmp().join(COVERAGE_DIRNAME)
    }

    pub fn crates(&self) -> Result<HashMap<String, Crate>, DynError> {
        let crates_dir = self.root()?.join(CRATES_DIRNAME);
        let mut crates = HashMap::new();

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
        fs::remove_dir_all(self.tmp())?;
        let crates = self.crates()?;
        for c in crates.values() {
            c.clean()?;
        }

        Ok(())
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        fs::create_dir_all(self.coverage())?;
        let crates = self.crates()?;
        for c in crates.values() {
            c.create_dirs()?;
        }

        Ok(())
    }
}
