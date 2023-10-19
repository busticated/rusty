use crate::options::Options;
use duct::{cmd, Expression};
use std::collections::HashMap;
use std::ffi::OsString;

pub type EnvVars = Option<HashMap<OsString, OsString>>;

pub trait Execute {
    fn bin(&self) -> String;

    fn opts(&self) -> &Options;

    fn exec_safe(&self, args: Vec<OsString>, envs: EnvVars) -> Expression {
        if envs.is_none() {
            return cmd(self.bin(), args);
        }

        let envs = envs.unwrap();
        let mut exp = cmd(self.bin(), args);

        for (key, value) in envs.iter() {
            exp = exp.env(key, value);
        }

        exp
    }

    fn exec_unsafe(&self, args: Vec<OsString>, envs: EnvVars) -> Expression {
        if self.opts().has("dry-run") {
            let mut args = args.clone();
            args.insert(0, "skipping:".into());
            args.insert(1, self.bin().into());
            // TODO (busticated): windows? see: https://stackoverflow.com/a/61857874/579167
            return cmd("echo", args);
        }

        self.exec_safe(args, envs)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_flags;

    struct TestExecutable {
        bin: String,
        opts: Options,
    }

    impl Execute for TestExecutable {
        fn bin(&self) -> String {
            self.bin.to_owned()
        }

        fn opts(&self) -> &Options {
            &self.opts
        }
    }

    impl TestExecutable {
        fn new(opts: Options) -> Self {
            TestExecutable {
                bin: "test".to_string(),
                opts,
            }
        }
    }

    #[test]
    fn it_builds_args() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let fake = TestExecutable::new(opts);
        let args = fake.build_args(["one"], ["two", "three"]);
        assert_eq!(args, ["one", "two", "three"]);
    }
}
