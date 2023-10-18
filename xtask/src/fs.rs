use std::fs;
use std::path::Path;
use crate::options::Options;

type IOResult = std::io::Result<()>;

#[derive(Clone, Debug, PartialEq)]
pub struct FS<'a> {
    opts: &'a Options,
}

impl<'a> FS<'a> {
    pub fn new(opts: &'a Options) -> FS<'a> {
        FS { opts }
    }

    pub fn write<P: AsRef<Path>, D: AsRef<[u8]>>(&self, path: P, data: D) -> IOResult {
        if self.opts.has("dry-run") {
            let path = path.as_ref().to_string_lossy();
            println!("Skipping: write {}", path);
            return Ok(());
        }

        fs::write(path, data)
    }

    pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> IOResult {
        if self.opts.has("dry-run") {
            let path = path.as_ref().to_string_lossy();
            println!("Skipping: remove_dir_all {}", path);
            return Ok(());
        }

        fs::remove_dir_all(path)
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> IOResult {
        if self.opts.has("dry-run") {
            let path = path.as_ref().to_string_lossy();
            println!("Skipping: create_dir_all {}", path);
            return Ok(());
        }

        fs::create_dir_all(path)
    }

    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> std::io::Result<fs::ReadDir> {
        fs::read_dir(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_flags;

    #[test]
    fn it_initializes() {
        let opts = Options::new(vec![], task_flags! {}).unwrap();
        let _ = FS::new(&opts);
    }
}
