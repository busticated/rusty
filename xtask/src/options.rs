use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;

type DynError = Box<dyn Error>;
type TaskFlags = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub args: Vec<String>,
    pub flags: TaskFlags,
}

#[allow(dead_code)]
impl Options {
    pub fn new(args: Vec<String>, flags: TaskFlags) -> Result<Self, DynError> {
        let re = Regex::new(r"^-*")?;
        let args = args
            .iter()
            .map(|x| re.replace_all(x.to_lowercase().trim(), "").to_string())
            .collect();

        for arg in &args {
            if !flags.contains_key(arg) {
                return Err(format!("Unrecognized argument! {}", arg).into());
            }
        }

        Ok(Options { args, flags })
    }

    pub fn has<F: AsRef<str>>(&self, flag: F) -> bool {
        let flag = flag.as_ref().trim().to_lowercase();
        for arg in &self.args {
            if arg == &flag {
                return true;
            }
        }

        false
    }
}

#[macro_export]
macro_rules! task_flags {
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::BTreeMap::from([$(($k.to_string(), $v.to_string()),)*])
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let flags = task_flags! {};
        let args = vec![];
        let opts = Options::new(args, flags).unwrap();
        assert_eq!(opts.flags.len(), 0);
        assert_eq!(opts.args.len(), 0);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: \"Unrecognized argument! nope\""
    )]
    fn it_fails_to_initialize_when_args_has_unrecognized_items() {
        let flags = task_flags! {};
        let args = vec!["nope".into()];
        Options::new(args, flags).unwrap();
    }

    #[test]
    fn it_checks_if_flag_is_set() {
        let flags = task_flags! { "test-ok" => "it's a test" };
        let args = vec!["--test-ok".into()];
        let opts = Options::new(args, flags).unwrap();
        assert!(opts.has("test-ok"));
        assert!(!opts.has("nope"));
    }
}
