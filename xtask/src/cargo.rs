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
pub(crate) struct Cargo<'a> {
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
    pub(crate) fn new(opts: &'a Options) -> Cargo<'a> {
        let bin = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        Cargo { bin, opts }
    }

    pub(crate) fn workspace_path(&self) -> Result<PathBuf, DynError> {
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

    pub(crate) fn create<P, U>(&self, path: P, arguments: U) -> Expression
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

    pub(crate) fn install<U>(&self, arguments: U) -> Expression
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
        // NOTE: `--locked` makes the tool build from its own committed
        // `Cargo.lock` - without it, an unrelated upstream release can break
        // `setup` on a repo where nothing changed
        let args = self.build_args(
            [OsString::from("install"), OsString::from("--locked")],
            arguments,
        );
        (args, None)
    }

    pub(crate) fn build<U>(&self, arguments: U) -> Expression
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

    pub(crate) fn clean<U>(&self, arguments: U) -> Expression
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

    pub(crate) fn test<U>(&self, arguments: U) -> Expression
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

    pub(crate) fn coverage_clean(&self) -> Expression {
        let (args, envs) = self.coverage_clean_params();
        self.exec_unsafe(args, envs)
    }

    fn coverage_clean_params(&self) -> (Vec<OsString>, EnvVars) {
        // NOTE: profiling data accumulates across runs - without this, stale
        // data from a previous run is folded into the report
        let args = self.build_args([OsString::from("llvm-cov")], ["clean", "--workspace"]);
        (args, None)
    }

    pub(crate) fn coverage(&self) -> Expression {
        let (args, envs) = self.coverage_params();
        self.exec_unsafe(args, envs)
    }

    fn coverage_params(&self) -> (Vec<OsString>, EnvVars) {
        // NOTE: `--no-report` runs the tests and collects profiling data but
        // renders nothing - `coverage_report()` below turns that single
        // collection run into both html and lcov output
        let args = self.build_args(
            [OsString::from("llvm-cov")],
            ["--workspace", "--all-features", "--no-report"],
        );
        (args, None)
    }

    pub(crate) fn coverage_report<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let (args, envs) = self.coverage_report_params(arguments);
        self.exec_unsafe(args, envs)
    }

    fn coverage_report_params<U>(&self, arguments: U) -> (Vec<OsString>, EnvVars)
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.build_args(
            [OsString::from("llvm-cov"), OsString::from("report")],
            arguments,
        );
        (args, None)
    }

    pub(crate) fn format(&self, check: bool) -> Expression {
        let (args, envs) = self.format_params(check);
        self.exec_safe(args, envs)
    }

    fn format_params(&self, check: bool) -> (Vec<OsString>, EnvVars) {
        let mut arguments = vec![OsString::from("--all")];

        if check {
            arguments.push("--check".into());
        }

        let args = self.build_args([OsString::from("fmt")], arguments);
        (args, None)
    }

    pub(crate) fn lint(&self) -> Expression {
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

    pub(crate) fn doc<U>(&self, arguments: U) -> Expression
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
        // NOTE: rustdoc only *warns* on problems like broken intra-doc links,
        // so they slip by unnoticed - this promotes them to build failures
        let envs = HashMap::from([("RUSTDOCFLAGS".into(), "-Dwarnings".into())]);
        (args, Some(envs))
    }

    pub(crate) fn update_lockfile(&self) -> Expression {
        let (args, envs) = self.update_lockfile_params();
        // NOTE: mutates `Cargo.lock`, so it must respect `--dry-run`
        self.exec_unsafe(args, envs)
    }

    fn update_lockfile_params(&self) -> (Vec<OsString>, EnvVars) {
        // NOTE: `--workspace` limits the update to workspace members, so only
        // the version entries we just bumped change. `--offline` guarantees a
        // release cannot quietly re-resolve external dependencies
        let args = self.build_args([OsString::from("update")], ["--workspace", "--offline"]);
        (args, None)
    }

    pub(crate) fn publish_package<N: AsRef<str>>(&self, name: N) -> Expression {
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
        assert_eq!(args, ["install", "--locked", "grcov"]);
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
        let (args, envs) = cargo.coverage_params();

        assert_eq!(
            args,
            ["llvm-cov", "--workspace", "--all-features", "--no-report"]
        );
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_coverage_clean_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.coverage_clean_params();

        assert_eq!(args, ["llvm-cov", "clean", "--workspace"]);
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_coverage_report_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) =
            cargo.coverage_report_params(["--html", "--output-dir", "fake-coverage-path"]);

        assert_eq!(
            args,
            [
                "llvm-cov",
                "report",
                "--html",
                "--output-dir",
                "fake-coverage-path"
            ]
        );
        assert_eq!(envs, None);
    }

    #[test]
    fn it_builds_args_for_the_format_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.format_params(false);
        assert_eq!(args, ["fmt", "--all"]);
        assert_eq!(envs, None);

        let (args, envs) = cargo.format_params(true);
        assert_eq!(args, ["fmt", "--all", "--check"]);
        assert_eq!(envs, None);
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
        let expected_envs = HashMap::from([("RUSTDOCFLAGS".into(), "-Dwarnings".into())]);

        assert_eq!(args, ["doc", "--workspace", "--no-deps"]);
        assert_eq!(envs, Some(expected_envs));
    }

    #[test]
    fn it_builds_args_for_the_update_lockfile_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let cargo = Cargo::new(&opts);
        let (args, envs) = cargo.update_lockfile_params();

        assert_eq!(args, ["update", "--workspace", "--offline"]);
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
