use std::fmt::{Display, Formatter};
use semver::{BuildMetadata, Prerelease, Version};

#[derive(Clone, Debug, PartialEq)]
pub enum VersionChoice {
    Major(Version),
    Minor(Version),
    Patch(Version),
}

impl Display for VersionChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            VersionChoice::Major(v) => format!("Major: {}", v),
            VersionChoice::Minor(v) => format!("Minor: {}", v),
            VersionChoice::Patch(v) => format!("Patch: {}", v),
        };

        write!(f, "{}", msg)
    }
}

impl VersionChoice {
    pub fn options(version: &Version) -> Vec<VersionChoice> {
        vec![
            VersionChoice::Major(increment_major(version)),
            VersionChoice::Minor(increment_minor(version)),
            VersionChoice::Patch(increment_patch(version)),
        ]
    }

    pub fn get_version(&self) -> Version {
        match self {
            VersionChoice::Major(v) => v.clone(),
            VersionChoice::Minor(v) => v.clone(),
            VersionChoice::Patch(v) => v.clone(),
        }
    }
}

pub fn increment_major(version: &Version) -> Version {
    let mut v = version.clone();
    v.major += 1;
    v.minor = 0;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
    v
}

pub fn increment_minor(version: &Version) -> Version {
    let mut v = version.clone();
    v.minor += 1;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
    v
}

pub fn increment_patch(version: &Version) -> Version {
    let mut v = version.clone();
    v.patch += 1;
    v.pre = Prerelease::EMPTY;
    v.build = BuildMetadata::EMPTY;
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_version_choice_options() {
        let version = Version::new(1, 0, 0);
        let options = VersionChoice::options(&version);
        assert_eq!(options.len(), 3);
        assert_eq!(options[0], VersionChoice::Major(Version::new(2, 0, 0)));
        assert_eq!(options[1], VersionChoice::Minor(Version::new(1, 1, 0)));
        assert_eq!(options[2], VersionChoice::Patch(Version::new(1, 0, 1)));
    }

    #[test]
    fn it_gets_version() {
        let choice = VersionChoice::Major(Version::new(1, 0, 0));
        assert_eq!(choice.get_version(), Version::new(1, 0, 0));
    }

    #[test]
    fn it_displays_version_choice_text() {
        let choice = VersionChoice::Major(Version::new(1, 0, 0));
        assert_eq!(format!("{}", choice), "Major: 1.0.0");
    }

    #[test]
    fn it_increments_major_version() {
        let version = Version::new(1, 0, 0);
        assert_eq!(increment_major(&version), Version::new(2, 0, 0));
    }

    #[test]
    fn it_increments_minor_version() {
        let version = Version::new(1, 0, 0);
        assert_eq!(increment_minor(&version), Version::new(1, 1, 0));
    }

    #[test]
    fn it_increments_patch_version() {
        let version = Version::new(1, 0, 0);
        assert_eq!(increment_patch(&version), Version::new(1, 0, 1));
    }
}
