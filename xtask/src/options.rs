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
        let args = args
            .iter()
            .map(|x| x.trim().replace('-', "").to_lowercase())
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
