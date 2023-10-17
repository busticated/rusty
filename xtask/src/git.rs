use crate::options::Options;
use duct::{cmd, Expression};
use std::ffi::OsString;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Git<'a> {
    opts: &'a Options,
}

impl<'a> Git<'a> {
    pub fn new(opts: &'a Options) -> Git<'a> {
        Git { opts }
    }

    pub fn cmd(&self, args: Vec<OsString>) -> Expression {
        let mut args = args.clone();

        if self.opts.has("dry-run") {
            args.insert(0, "skipping:".into());
            args.insert(1, "git".into());
            // TODO (busticated): windows? see: https://stackoverflow.com/a/61857874/579167
            return cmd("echo", args);
        }

        cmd("git", args)
    }

    fn build_args<U, UU>(&self, args1: U, args2: UU) -> Vec<OsString>
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
        UU: IntoIterator,
        UU::Item: Into<OsString>,
    {
        let mut args = args1
            .into_iter()
            .map(Into::<OsString>::into)
            .collect::<Vec<_>>();

        args.extend(
            args2
                .into_iter()
                .map(Into::<OsString>::into)
                .collect::<Vec<_>>(),
        );

        args.retain(|a| !a.is_empty());
        args
    }

    pub fn add<P, U>(&self, path: P, arguments: U) -> Expression
    where
        P: AsRef<Path>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.add_raw(path, arguments);
        self.cmd(args)
    }

    fn add_raw<P, U>(&self, path: P, arguments: U) -> Vec<OsString>
    where
        P: AsRef<Path>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(
            vec![OsString::from("add"), path.as_ref().to_owned().into()],
            arguments,
        )
    }

    pub fn commit<M, U>(&self, message: M, arguments: U) -> Expression
    where
        M: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.commit_raw(message, arguments);
        self.cmd(args)
    }

    fn commit_raw<M, U>(&self, message: M, arguments: U) -> Vec<OsString>
    where
        M: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(vec!["commit", "--message", message.as_ref()], arguments)
    }

    pub fn tag<T, U>(&self, tag: T, arguments: U) -> Expression
    where
        T: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.tag_raw(tag, arguments);
        self.cmd(args)
    }

    fn tag_raw<T, U>(&self, tag: T, arguments: U) -> Vec<OsString>
    where
        T: AsRef<str>,
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(
            vec!["tag", tag.as_ref(), "--message", tag.as_ref()],
            arguments,
        )
    }

    pub fn get_tags<U>(&self, arguments: U) -> Expression
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        let args = self.get_tags_raw(arguments);
        self.cmd(args)
    }

    fn get_tags_raw<U>(&self, arguments: U) -> Vec<OsString>
    where
        U: IntoIterator,
        U::Item: Into<OsString>,
    {
        self.build_args(vec!["tag"], arguments)
    }

    pub fn todos(&self) -> Expression {
        let args = self.todos_raw();
        self.cmd(args)
    }

    fn todos_raw(&self) -> Vec<OsString> {
        let ptn = r"TODO\s?\(.*\)|todo!\(\)";

        self.build_args(
            vec![
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use crate::task_flags;

    #[test]
    fn it_initializes() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let _ = Git::new(&opts);
    }

    #[test]
    fn it_builds_args() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.build_args(["one"], vec!["two", "three"]);
        assert_eq!(args, ["one", "two", "three"]);
    }

    #[test]
    fn it_builds_args_for_the_add_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.add_raw(Path::new("path/to/file"), [""]);
        assert_eq!(args, ["add", "path/to/file"]);
    }

    #[test]
    fn it_builds_args_for_the_commit_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.commit_raw("my message", ["--one", "--two"]);
        assert_eq!(
            args,
            ["commit", "--message", "my message", "--one", "--two"]
        );
    }

    #[test]
    fn it_builds_args_for_the_tag_subcommand() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.tag_raw("my tag", ["--one", "--two"]);
        assert_eq!(
            args,
            ["tag", "my tag", "--message", "my tag", "--one", "--two"]
        );
    }

    #[test]
    fn it_builds_args_for_getting_tags() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.get_tags_raw(["--points-at", "HEAD"]);
        assert_eq!(args, ["tag", "--points-at", "HEAD"]);
    }

    #[test]
    fn it_builds_args_for_getting_todos() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let git = Git::new(&opts);
        let args = git.todos_raw();
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
}
