use crate::readme::Readme;
use crate::toml::Toml;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

type DynError = Box<dyn Error>;

const TMP_DIRNAME: &str = "tmp";
const COVERAGE_DIRNAME: &str = "coverage";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Krate {
    pub name: String,
    pub description: String,
    pub kind: KrateKind,
    pub path: PathBuf,
    pub readme: Readme,
    pub toml: Toml,
}

impl KratePaths for Krate {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Krate {
    pub fn new<N: AsRef<str>, D: AsRef<str>>(name: N, description: D, path: PathBuf) -> Self {
        let kind = KrateKind::Library; // TODO (busticated): lookup somehow instead of default'ing
        let name = name.as_ref().to_owned();
        let description = description.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        let toml = Toml::new(path.clone());
        Krate {
            kind,
            name,
            description,
            path,
            readme,
            toml,
        }
    }

    pub fn from_path(path: PathBuf) -> Result<Krate, DynError> {
        let toml = Toml::from_path(path.clone())?;
        let readme = Readme::from_path(path.clone())?;
        let kind = KrateKind::Library;
        let mut krate = Krate {
            kind,
            path,
            toml,
            readme,
            ..Default::default()
        };

        krate.name = krate.toml.get_name()?;
        krate.description = krate.toml.get_description()?;
        Ok(krate)
    }

    pub fn clean(&self) -> Result<(), DynError> {
        Ok(fs::remove_dir_all(self.tmp_path())?)
    }

    pub fn create_dirs(&self) -> Result<(), DynError> {
        Ok(fs::create_dir_all(self.coverage_path())?)
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum KrateKind {
    Library,
    Binary,
}

impl Default for KrateKind {
    fn default() -> Self {
        KrateKind::new()
    }
}

impl KrateKind {
    pub fn new() -> KrateKind {
        KrateKind::Library
    }
}

impl Display for KrateKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            KrateKind::Binary => "--bin",
            KrateKind::Library => "--lib",
        };

        write!(f, "{}", arch)
    }
}

impl FromStr for KrateKind {
    type Err = DynError;

    fn from_str(s: &str) -> Result<KrateKind, DynError> {
        match s {
            "binary" | "bin" | "--bin" => Ok(KrateKind::Binary),
            "library" | "lib" | "--lib" => Ok(KrateKind::Library),
            _ => Err(format!("Unrecognized input: {}", s).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_a_krate_kind() {
        let krate = KrateKind::new();
        assert_eq!(krate, KrateKind::Library);
    }

    #[test]
    fn it_initializes_a_krate_kind_with_defaults() {
        let krate = KrateKind::default();
        assert_eq!(krate, KrateKind::Library);
    }

    #[test]
    fn it_initializes_a_krate_kind_from_str() {
        let krate = KrateKind::from_str("library").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("lib").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("--lib").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("binary").unwrap();

        assert_eq!(krate, KrateKind::Binary);

        let krate = KrateKind::from_str("bin").unwrap();

        assert_eq!(krate, KrateKind::Binary);

        let krate = KrateKind::from_str("--bin").unwrap();

        assert_eq!(krate, KrateKind::Binary);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: \"Unrecognized input: NOPE!\""
    )]
    fn it_fails_to_initialize_when_krate_kind_cannot_be_determined_from_str() {
        KrateKind::from_str("NOPE!").unwrap();
    }

    #[test]
    fn it_initializes_a_krate() {
        let krate = Krate::new(
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.name, "my-crate");
        assert_eq!(krate.description, "my-crate's description");
        assert_eq!(krate.path, PathBuf::from("fake-crate"));
        assert_eq!(krate.kind, KrateKind::Library);
    }

    #[test]
    fn it_gets_path_to_krate() {
        let krate = Krate::new(
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.path(), PathBuf::from("fake-crate"));
    }

    #[test]
    fn it_gets_path_to_krate_tmp_dir() {
        let krate = Krate::new(
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.tmp_path(), PathBuf::from("fake-crate").join("tmp"));
    }

    #[test]
    fn it_gets_path_to_krate_coverage_dir() {
        let krate = Krate::new(
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(
            krate.coverage_path(),
            PathBuf::from("fake-crate").join("tmp").join("coverage")
        );
    }
}
