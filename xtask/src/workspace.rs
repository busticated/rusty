use crate::krate::{Krate, KrateKind, KratePaths};
use crate::readme::Readme;
use crate::toml::Toml;
use duct::cmd;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::Table;

type DynError = Box<dyn Error>;

const CRATES_DIRNAME: &str = "crates";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Workspace {
    pub path: PathBuf,
    pub cargo: String,
    pub readme: Readme,
    pub toml: Toml,
}

impl KratePaths for Workspace {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Workspace {
    pub fn new() -> Self {
        let cargo = get_cargo_cmd();
        let path = root_path(&cargo).unwrap();
        let readme = Readme::new(path.clone());
        let toml = Toml::new(path.clone());
        Workspace {
            cargo,
            path,
            readme,
            toml,
        }
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

    pub fn add_krate<K: AsRef<str>, N: AsRef<str>, D: AsRef<str>>(
        &self,
        kind: K,
        name: N,
        description: D,
    ) -> Result<Krate, DynError> {
        let path = self.krates_path().join(name.as_ref());
        let mut krate = Krate::new(name, description, path);

        krate.kind = KrateKind::like(kind.as_ref())?;

        cmd!(
            &self.cargo,
            "new",
            &krate.path,
            "--name",
            &krate.name,
            krate.kind.to_string()
        )
        .run()?;

        krate.readme.create(&krate.name, &krate.description)?;

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

    #[allow(dead_code)]
    pub fn manifest(&self) -> Result<Table, DynError> {
        self.toml.read()
    }
}

// UTILS //////////////////////////////////////////////////////////////////////
fn get_cargo_cmd() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_a_workspace() {
        let workspace = Workspace::new();
        assert!(!workspace.cargo.is_empty());
    }

    #[test]
    fn it_gets_path_to_workspace() {
        let workspace = Workspace::new();
        let root_path = root_path(get_cargo_cmd()).unwrap();
        assert_eq!(workspace.path(), root_path);
    }

    #[test]
    fn it_gets_path_to_workspace_tmp_dir() {
        let workspace = Workspace::new();
        let root_path = root_path(get_cargo_cmd()).unwrap();
        assert_eq!(workspace.tmp_path(), root_path.join("tmp"));
    }

    #[test]
    fn it_gets_path_to_workspace_coverage_dir() {
        let workspace = Workspace::new();
        let root_path = root_path(get_cargo_cmd()).unwrap();
        assert_eq!(
            workspace.coverage_path(),
            root_path.join("tmp").join("coverage")
        );
    }

    #[test]
    fn it_gets_path_to_workspace_manifest_file() {
        let workspace = Workspace::new();
        let root_path = root_path(get_cargo_cmd()).unwrap();
        assert_eq!(workspace.manifest_path(), root_path.join("Cargo.toml"));
    }
}
