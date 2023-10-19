use crate::exec::{EnvVars, Execute};
use crate::options::Options;
use duct::Expression;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

#[derive(Clone, Debug, PartialEq)]
pub struct Cargo<'a> {
    pub bin: String,
    opts: &'a Options,
}

impl<'a> Execute for Cargo<'a> {
    fn bin(&self) -> String {
        self.bin.to_owned()
    }

    fn opts(&self) -> &Options {
        self.opts
    }
}

impl<'a> Cargo<'a> {
    pub fn new(opts: &'a Options) -> Cargo<'a> {
        let bin = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        Cargo { bin, opts }
    }

    pub fn workspace_path(&self) -> Result<PathBuf, DynError> {
        let (args, envs) = self.workspace_path_params();
        let stdout = self.exec_safe(args, envs).read()?;
        Ok(PathBuf::from(stdout.replace("Cargo.toml", "").trim()))
    }

    fn workspace_path_params(&self) -> (Vec<OsString>, EnvVars) {
        let args = self.build_args(
            ["locate-project", "--workspace", "--message-format", "plain"],
            [""],
        );
        (args, None)
    }

    pub fn create<P, U>(&self, path: P, arguments: U) -> Expression
    where
        P: Into<OsString>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.create_params(path, arguments);
        self.exec_unsafe(args, envs)
    }

    fn create_params<P, U>(&self, path: P, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        P: Into<OsString>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args(["new".into(), path.into()], arguments);
        (args, None)
    }

    pub fn install<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.install_params(arguments);
        self.exec_unsafe(args, envs)
    }

    fn install_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args([OsString::from("install")], arguments);
        (args, None)
    }

    pub fn build<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.build_params(arguments);
        self.exec_safe(args, envs)
    }

    fn build_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args([OsString::from("build")], arguments);
        (args, None)
    }

    pub fn clean<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.clean_params(arguments);
        self.exec_unsafe(args, envs)
    }

    fn clean_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args([OsString::from("clean")], arguments);
        (args, None)
    }

    pub fn test<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.test_params(arguments);
        self.exec_safe(args, envs)
    }

    fn test_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args([OsString::from("test")], arguments);
        (args, None)
    }

    pub fn coverage<P>(&self, path: P) -> Expression
    where
        P: Into<OsString>,
    {
        let (args, envs) = self.coverage_params(path);
        self.exec_unsafe(args, envs)
    }

    fn coverage_params<P>(&self, path: P) -> (Vec<OsString>, EnvVars)
    where
        P: Into<OsString>,
    {
        let mut profile_ptn: OsString = path.into();
        profile_ptn.push("/cargo-test-%p-%m.profraw");
        let args = self.build_args([OsString::from("test")], [""]);
        let envs = HashMap::from([
            ("CARGO_INCREMENTAL".into(), "0".into()),
            ("RUSTFLAGS".into(), "-Cinstrument-coverage".into()),
            ("LLVM_PROFILE_FILE".into(), profile_ptn),
        ]);

        (args, Some(envs))
    }

    pub fn lint(&self) -> Expression {
        let (args, envs) = self.lint_params();
        self.exec_safe(args, envs)
    }

    fn lint_params(&self) -> (Vec<OsString>, EnvVars) {
        let args = self.build_args(
            [OsString::from("clippy")],
            ["--all-targets", "--all-features", "--no-deps"],
        );
        let envs = HashMap::from([("RUSTFLAGS".into(), "-Dwarnings".into())]);

        (args, Some(envs))
    }

    pub fn doc<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.doc_params(arguments);
        self.exec_unsafe(args, envs)
    }

    fn doc_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args([OsString::from("doc")], arguments);
        (args, None)
    }

    pub fn publish_package<N: AsRef<str>>(&self, name: N) -> Expression {
        let (args, envs) = self.publish_package_params(name);
        self.exec_unsafe(args, envs)
    }

    fn publish_package_params<N: AsRef<str>>(&self, name: N) -> (Vec<OsString>, EnvVars) {
        let args = self.build_args(["publish", "--package", name.as_ref()], [""]);
        (args, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_flags;

    #[test]
    fn it_builds_args_for_getting_workspace_path() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.workspace_path_params();
        assert_eq!(
            args,
            ["locate-project", "--workspace", "--message-format", "plain"]
        );
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_create_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let path = PathBuf::from("fake-crate-path");
        let (args, envs) = cargo.create_params(path, ["--name", "my-crate", "--lib"]);
        assert_eq!(
            args,
            ["new", "fake-crate-path", "--name", "my-crate", "--lib"]
        );
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_install_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.install_params(["grcov"]);
        assert_eq!(args, ["install", "grcov"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_build_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.build_params(["--release"]);
        assert_eq!(args, ["build", "--release"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_clean_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.clean_params(["--release"]);
        assert_eq!(args, ["clean", "--release"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_test_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.test_params(["--doc"]);
        assert_eq!(args, ["test", "--doc"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_coverage_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let path = PathBuf::from("fake-coverage-path");
        let (args, envs) = cargo.coverage_params(path);
        let expected_envs = HashMap::from([
            ("CARGO_INCREMENTAL".into(), "0".into()),
            ("RUSTFLAGS".into(), "-Cinstrument-coverage".into()),
            (
                "LLVM_PROFILE_FILE".into(),
                "fake-coverage-path/cargo-test-%p-%m.profraw".into(),
            ),
        ]);

        assert_eq!(args, ["test"]);
        assert_eq!(envs, Some(expected_envs));
    }

    #[test]
    fn it_builds_args_for_the_lint_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.lint_params();
        let expected_envs = HashMap::from([("RUSTFLAGS".into(), "-Dwarnings".into())]);
        assert_eq!(
            args,
            ["clippy", "--all-targets", "--all-features", "--no-deps"]
        );
        assert_eq!(envs, Some(expected_envs));
    }

    #[test]
    fn it_builds_args_for_the_doc_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.doc_params(["--workspace", "--no-deps"]);
        assert_eq!(args, ["doc", "--workspace", "--no-deps"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_publish_package_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.publish_package_params("my-crate");
        assert_eq!(args, ["publish", "--package", "my-crate"]);
        assert_eq!(envs, None);
    }
}
