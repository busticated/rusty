use crate::krate::{Krate, KratePaths};
use crate::readme::Readme;
use crate::toml::Toml;
use duct::cmd;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

const CRATES_DIRNAME: &str = "crates";

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    pub path: PathBuf,
    pub cargo_cmd: String,
    pub readme: Readme,
    pub toml: Toml,
}

impl KratePaths for Workspace {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Workspace {
    #[allow(dead_code)]
    pub fn new<C: AsRef<str>>(cargo_cmd: C, path: PathBuf) -> Self {
        let cargo_cmd = cargo_cmd.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        let toml = Toml::new(path.clone());
        Workspace {
            cargo_cmd,
            path,
            readme,
            toml,
        }
    }

    pub fn from_path<C: AsRef<str>>(cargo_cmd: C, path: PathBuf) -> Result<Workspace, DynError> {
        let cargo_cmd = cargo_cmd.as_ref().to_owned();
        let readme = Readme::from_path(path.clone())?;
        let toml = Toml::from_path(path.clone())?;
        Ok(Workspace {
            cargo_cmd,
            path,
            readme,
            toml,
        })
    }

    pub fn krates_path(&self) -> PathBuf {
        self.path().join(CRATES_DIRNAME)
    }

    pub fn krates(&self) -> Result<BTreeMap<String, Krate>, DynError> {
        let mut krates = BTreeMap::new();

        for entry in fs::read_dir(self.krates_path())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let krate = Krate::from_path(path.clone())?;
                krates.insert(krate.name.clone(), krate);
            }
        }

        Ok(krates)
    }

    pub fn add_krate<K: AsRef<str>, V: AsRef<str>, N: AsRef<str>, D: AsRef<str>>(
        &self,
        kind: K,
        version: V,
        name: N,
        description: D,
    ) -> Result<Krate, DynError> {
        let path = self.krates_path().join(name.as_ref());
        let krate = Krate::new(kind, version, name, description, path);

        cmd!(
            &self.cargo_cmd,
            "new",
            &krate.path,
            "--name",
            &krate.name,
            krate.kind.to_string()
        )
        .run()?;

        krate.readme.create(&krate.name, &krate.description)?;
        krate.toml.create(&krate.name, &krate.description)?;

        Ok(krate)
    }

    pub fn clean(&self) -> Result<(), DynError> {
        fs::remove_dir_all(self.tmp_path())?;
        let krates = self.krates()?;

        for krate in krates.values() {
            krate.clean()?;
        }

        Ok(())
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        fs::create_dir_all(self.coverage_path())?;
        let krates = self.krates()?;

        for krate in krates.values() {
            krate.create_dirs()?;
        }

        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_a_workspace() {
        let workspace = Workspace::new("fake-cargo", PathBuf::from("fake-root"));
        assert!(!workspace.cargo_cmd.is_empty());
    }

    #[test]
    fn it_gets_path_to_workspace() {
        let workspace = Workspace::new("fake-cargo", PathBuf::from("fake-root"));
        assert!(!workspace.cargo_cmd.is_empty());
        assert_eq!(workspace.path(), PathBuf::from("fake-root"));
    }

    #[test]
    fn it_gets_path_to_workspace_tmp_dir() {
        let path = PathBuf::from("fake-root");
        let workspace = Workspace::new("fake-cargo", path.clone());
        assert_eq!(workspace.tmp_path(), path.join("tmp"));
    }

    #[test]
    fn it_gets_path_to_workspace_coverage_dir() {
        let path = PathBuf::from("fake-root");
        let workspace = Workspace::new("fake-cargo", path.clone());
        assert_eq!(workspace.coverage_path(), path.join("tmp").join("coverage"));
    }
}
