use crate::cargo::Cargo;
use crate::fs::FS;
use crate::krate::{Krate, KratePaths};
use crate::readme::Readme;
use crate::toml::Toml;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::{Path, PathBuf};

type DynError = Box<dyn Error>;

const CRATES_DIRNAME: &str = "crates";

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    pub path: PathBuf,
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
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        let toml = Toml::new(path.clone());
        Workspace { path, readme, toml }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Workspace, DynError> {
        let path = path.as_ref().to_owned();
        let readme = Readme::from_path(path.clone())?;
        let toml = Toml::from_path(path.clone())?;
        Ok(Workspace { path, readme, toml })
    }

    pub fn krates_path(&self) -> PathBuf {
        self.path().join(CRATES_DIRNAME)
    }

    pub fn krates(&self, fs: &FS) -> Result<BTreeMap<String, Krate>, DynError> {
        let mut krates = BTreeMap::new();

        for entry in fs.read_dir(self.krates_path())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let krate = Krate::from_path(path.clone())?;
                krates.insert(krate.name.clone(), krate);
            }
        }

        Ok(krates)
    }

    pub fn add_krate(&self, fs: &FS, cargo: &Cargo, mut krate: Krate) -> Result<Krate, DynError> {
        let kind = krate.kind.to_string();
        cargo
            .create(&krate.path, ["--name", &krate.name, &kind])
            .run()?;
        krate.readme.create(fs, &krate.clone())?;
        krate.toml.create(fs, &krate.clone())?;
        Ok(krate)
    }

    pub fn clean(&self, fs: &FS, cargo: &Cargo) -> Result<(), DynError> {
        use std::io::ErrorKind;

        match fs.remove_dir_all(self.tmp_path()) {
            Err(e) if e.kind() == ErrorKind::NotFound => (),
            Err(e) => return Err(Box::new(e)),
            Ok(()) => (),
        };

        let krates = self.krates(fs)?;

        for krate in krates.values() {
            krate.clean(fs)?;
        }

        cargo.clean(["--release"]).run()?;
        Ok(())
    }

    pub fn create_dirs(&self, fs: &FS) -> Result<(), DynError> {
        fs.create_dir_all(self.coverage_path())?;
        let krates = self.krates(fs)?;

        for krate in krates.values() {
            krate.create_dirs(fs)?;
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
        let fake_path = PathBuf::from("fake-path");
        let workspace = Workspace::new(fake_path);
        assert_eq!(workspace.path, PathBuf::from("fake-path"));
    }

    #[test]
    fn it_gets_path_to_workspace() {
        let fake_path = PathBuf::from("fake-path");
        let workspace = Workspace::new(fake_path);
        assert_eq!(workspace.path(), PathBuf::from("fake-path"));
    }

    #[test]
    fn it_gets_path_to_workspace_tmp_dir() {
        let fake_path = PathBuf::from("fake-path");
        let workspace = Workspace::new(&fake_path);
        assert_eq!(workspace.tmp_path(), fake_path.join("tmp"));
    }

    #[test]
    fn it_gets_path_to_workspace_coverage_dir() {
        let fake_path = PathBuf::from("fake-path");
        let workspace = Workspace::new(&fake_path);
        assert_eq!(
            workspace.coverage_path(),
            fake_path.join("tmp").join("coverage")
        );
    }
}
