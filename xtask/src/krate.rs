use crate::readme::Readme;
use crate::toml::Toml;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use semver::Version;

type DynError = Box<dyn Error>;

const TMP_DIRNAME: &str = "tmp";
const COVERAGE_DIRNAME: &str = "coverage";
const SRC_DIRNAME: &str = "src";
const LIB_FILENAME: &str = "lib.rs";

#[derive(Clone, Debug)]
pub struct Krate {
    pub kind: KrateKind,
    pub version: Version,
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub readme: Readme,
    pub toml: Toml,
}

impl KratePaths for Krate {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Default for Krate {
    fn default() -> Self {
        let kind = KrateKind::default();
        let version = Version::new(0, 1, 0);
        let name = String::default();
        let description = String::default();
        let path = PathBuf::default();
        let readme = Readme::default();
        let toml = Toml::default();
        Krate {
            kind,
            version,
            name,
            description,
            path,
            readme,
            toml,
        }
    }
}

impl Krate {
    pub fn new<K: AsRef<str>, V: AsRef<str>, N: AsRef<str>, D: AsRef<str>>(
        kind: K,
        version: V,
        name: N,
        description: D,
        path: PathBuf,
    ) -> Self {
        let kind = KrateKind::new(kind.as_ref());
        let version = Version::parse(version.as_ref()).unwrap_or(Version::new(0, 1, 0));
        let name = name.as_ref().to_owned();
        let description = description.as_ref().to_owned();
        let readme = Readme::new(path.clone());
        let toml = Toml::new(path.clone());
        Krate {
            kind,
            version,
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
        let kind = KrateKind::from_path(path.clone())?;
        let name = toml.get_name()?;
        let description = toml.get_description()?;
        let version = toml.get_version()?;
        let krate = Krate {
            kind,
            version,
            name,
            description,
            path,
            readme,
            toml,
        };

        Ok(krate)
    }

    pub fn id(&self) -> String {
        format!("{}@{}", &self.name, self.version)
    }

    pub fn set_version(&mut self, version: Version) -> Result<(), DynError> {
        self.version = version;
        self.toml.set_version(&self.version)?;
        Ok(())
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

#[derive(Clone, Debug, Default, PartialEq)]
pub enum KrateKind {
    #[default]
    Library,
    Binary,
}

impl KrateKind {
    pub fn new<K: AsRef<str>>(kind: K) -> KrateKind {
        let kind = KrateKind::from_str(kind.as_ref());

        if kind.is_err() {
            return KrateKind::Library;
        }

        kind.unwrap()
    }

    pub fn from_path(path: PathBuf) -> Result<KrateKind, DynError> {
        let path = path.join(SRC_DIRNAME).join(LIB_FILENAME);

        if path.try_exists().is_err() {
            return Ok(KrateKind::Binary);
        }

        Ok(KrateKind::Library)
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
        match s.to_lowercase().trim() {
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
        let krate = KrateKind::new("bin");
        assert_eq!(krate, KrateKind::Binary);
    }

    #[test]
    fn it_initializes_a_krate_kind_as_a_library_if_lookup_fails() {
        let krate = KrateKind::new("NOPE!");
        assert_eq!(krate, KrateKind::Library);
    }

    #[test]
    fn it_initializes_a_krate_kind_with_defaults() {
        let krate = KrateKind::default();
        assert_eq!(krate, KrateKind::Library);
    }

    #[test]
    fn it_initializes_a_krate_kind_from_str() {
        let krate = KrateKind::from_str("Library").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("library").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("lib").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("--lib").unwrap();

        assert_eq!(krate, KrateKind::Library);

        let krate = KrateKind::from_str("Binary").unwrap();

        assert_eq!(krate, KrateKind::Binary);

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
            "lib",
            "1.0.0",
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.kind, KrateKind::Library);
        assert_eq!(krate.version, Version::new(1, 0, 0));
        assert_eq!(krate.name, "my-crate");
        assert_eq!(krate.description, "my-crate's description");
        assert_eq!(krate.path, PathBuf::from("fake-crate"));
    }

    #[test]
    fn it_initializes_a_default_krate() {
        let krate = Krate::default();
        assert_eq!(krate.kind, KrateKind::Library);
        assert_eq!(krate.version, Version::new(0, 1, 0));
        assert_eq!(krate.name, "");
        assert_eq!(krate.description, "");
        assert_eq!(krate.path, PathBuf::from(""));
        assert_eq!(krate.readme.path, PathBuf::from(""));
        assert_eq!(krate.toml.path, PathBuf::from(""));
    }

    #[test]
    fn it_sets_krate_version() {
        let mut krate = Krate::new(
            "lib",
            "0.1.0",
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );

        krate.set_version(Version::new(1, 0, 0)).unwrap();

        assert_eq!(krate.version.to_string(), "1.0.0");
        assert_eq!(krate.toml.get_version().unwrap().to_string(), "1.0.0");
    }

    #[test]
    fn it_gets_path_to_krate() {
        let krate = Krate::new(
            "lib",
            "0.1.0",
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.path(), PathBuf::from("fake-crate"));
    }

    #[test]
    fn it_gets_path_to_krate_tmp_dir() {
        let krate = Krate::new(
            "lib",
            "0.1.0",
            "my-crate",
            "my-crate's description",
            PathBuf::from("fake-crate"),
        );
        assert_eq!(krate.tmp_path(), PathBuf::from("fake-crate").join("tmp"));
    }

    #[test]
    fn it_gets_path_to_krate_coverage_dir() {
        let krate = Krate::new(
            "lib",
            "0.1.0",
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
