use crate::exec::Execute;
use crate::options::Options;
use crate::Krate;
use duct::Expression;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;

type DynError = Box<dyn Error>;

#[derive(Clone, Debug, PartialEq)]
pub struct Git<'a> {
    pub bin: String,
    opts: &'a Options,
}

impl<'a> Execute for Git<'a> {
    fn bin(&self) -> String {
        self.bin.to_owned()
    }

    fn opts(&self) -> &Options {
        self.opts
    }
}

impl<'a> Git<'a> {
    pub fn new(opts: &'a Options) -> Git<'a> {
        let bin = "git".to_string();
        Git { bin, opts }
    }

    pub fn add<P, U>(&self, path: P, arguments: U) -> Expression
    where
        P: AsRef<Path>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.add_params(path, arguments);
        self.exec_unsafe(args, None)
    }

    fn add_params<P, U>(&self, path: P, arguments: U) -> Vec<OsString>
    where
        P: AsRef<Path>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(
            [OsString::from("add"), path.as_ref().to_owned().into()],
            arguments,
        )
    }

    pub fn commit<M, U>(&self, message: M, arguments: U) -> Expression
    where
        M: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.commit_params(message, arguments);
        self.exec_unsafe(args, None)
    }

    fn commit_params<M, U>(&self, message: M, arguments: U) -> Vec<OsString>
    where
        M: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(["commit", "--message", message.as_ref()], arguments)
    }

    pub fn tag<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.tag_params(arguments);
        self.exec_safe(args, None)
    }

    fn tag_params<U>(&self, arguments: U) -> Vec<OsString>
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(["tag"], arguments)
    }

    pub fn create_tag<T>(&self, tag: T) -> Expression
    where
        T: AsRef<str>,
    {
        let args = self.create_tag_params(tag);
        self.exec_unsafe(args, None)
    }

    fn create_tag_params<T>(&self, tag: T) -> Vec<OsString>
    where
        T: AsRef<str>,
    {
        self.tag_params([tag.as_ref(), "--message", tag.as_ref()])
    }

    pub fn todos(&self) -> Expression {
        let args = self.todos_params();
        self.exec_safe(args, None)
    }

    fn todos_params(&self) -> Vec<OsString> {
        let ptn = r"TODO\s?\(.*\)|todo!\(\)";

        self.build_args(
            [
                "grep",
                "-P",
                "-e",
                ptn,
                "--ignore-case",
                "--heading",
                "--break",
                "--context",
                "2",
                "--full-name",
                "--line-number",
                "--",
                ":!./target/*",
                ":!./tmp/*",
            ],
            [""],
        )
    }

    pub fn get_changelog(&self, krate: &Krate) -> Result<Vec<String>, DynError> {
        let (prefix, args) = self.get_changelog_params(krate);
        let history = self.exec_safe(args, None).read()?;
        Ok(self.fmt_changelog(prefix, history))
    }

    fn get_changelog_params(&self, krate: &Krate) -> (String, Vec<OsString>) {
        let range = format!("{}@{}..HEAD", &krate.name, &krate.version);
        let query = format!(r"--grep=\[{}\]", &krate.name);
        let fmt = String::from("--pretty=format:%B");
        let prefix = format!("[{}]", &krate.name);
        let args = self.build_args(["log"], [range, query, fmt]);
        (prefix, args)
    }

    fn fmt_changelog(&self, prefix: String, history: String) -> Vec<String> {
        history
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| str::to_string(x.replace(&prefix, "").trim()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_flags;
    use std::path::{Path, PathBuf};

    #[test]
    fn it_builds_args_for_the_add_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.add_params(Path::new("path/to/file"), [""]);
        assert_eq!(args, ["add", "path/to/file"]);
    }

    #[test]
    fn it_builds_args_for_the_commit_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.commit_params("my message", ["--one", "--two"]);
        assert_eq!(
            args,
            ["commit", "--message", "my message", "--one", "--two"]
        );
    }

    #[test]
    fn it_builds_args_for_the_tag_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.tag_params(["--points-at", "HEAD"]);
        assert_eq!(args, ["tag", "--points-at", "HEAD"]);
    }

    #[test]
    fn it_builds_args_for_creating_a_tag() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.create_tag_params("my-tag");
        assert_eq!(args, ["tag", "my-tag", "--message", "my-tag"]);
    }

    #[test]
    fn it_builds_args_for_getting_todos() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.todos_params();
        assert_eq!(
            args,
            [
                "grep",
                "-P",
                "-e",
                r"TODO\s?\(.*\)|todo!\(\)",
                "--ignore-case",
                "--heading",
                "--break",
                "--context",
                "2",
                "--full-name",
                "--line-number",
                "--",
                ":!./target/*",
                ":!./tmp/*"
            ]
        );
    }

    #[test]
    fn it_builds_args_for_getting_changelog() {
        let path = PathBuf::from("my-crate");
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let krate = Krate::new("lib", "0.1.0", "my-crate", "", path);
        let git = Git::new(&opts);
        let (prefix, args) = git.get_changelog_params(&krate);
        assert_eq!(prefix, "[my-crate]");
        assert_eq!(
            args,
            [
                "log",
                "my-crate@0.1.0..HEAD",
                "--grep=\\[my-crate\\]",
                "--pretty=format:%B"
            ]
        );
    }

    #[test]
    fn it_formats_changelog() {
        let prefix = String::from("[my-crate]");
        let history = format!("{prefix} commit 01\n{prefix} commit 02\n");
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let log = git.fmt_changelog(prefix, history);
        assert_eq!(log, vec!["commit 01", "commit 02"]);
    }
}
