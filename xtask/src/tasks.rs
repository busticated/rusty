use crate::options::Options;
use crate::workspace::Workspace;
use std::collections::BTreeMap;
use std::error::Error;

type DynError = Box<dyn Error>;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub flags: BTreeMap<String, String>,
    pub run: fn(opts: Options, &mut Workspace, &Tasks) -> Result<(), DynError>,
}

impl Task {
    #[allow(dead_code)]
    pub fn new<N: AsRef<str>, D: AsRef<str>>(
        name: N,
        description: D,
        flags: BTreeMap<String, String>,
        run: fn(args: Options, &mut Workspace, &Tasks) -> Result<(), DynError>,
    ) -> Self {
        Task {
            name: name.as_ref().to_owned(),
            description: description.as_ref().to_owned(),
            flags,
            run,
        }
    }

    pub fn exec(
        &self,
        args: Vec<String>,
        workspace: &mut Workspace,
        tasks: &Tasks,
    ) -> Result<(), DynError> {
        let opts = Options::new(args, self.flags.clone())?;
        (self.run)(opts, workspace, tasks)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tasks {
    map: BTreeMap<String, Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, tasks: Vec<Task>) {
        for task in tasks.iter() {
            self.map.insert(task.name.clone(), task.clone());
        }
    }

    pub fn get<T: AsRef<str>>(&self, name: T) -> Option<&Task> {
        self.map.get(name.as_ref())
    }

    pub fn help(&self) -> Result<String, DynError> {
        let separator = ".".to_string();
        let mut lines = String::new();
        let mut max_col_width = 0;
        let padding = 4;

        for name in self.map.keys() {
            let char_count = name.char_indices().count();

            if max_col_width < char_count {
                max_col_width = char_count;
            }
        }

        for task in self.map.values() {
            let char_count = task.name.char_indices().count();
            let spaces = separator.repeat(max_col_width - char_count + padding);
            let line = format!("> {}{}{}\n", task.name, spaces, task.description);

            lines.push_str(&line);

            for (name, description) in task.flags.iter() {
                let separator = " ".to_string();
                let spaces = separator.repeat(max_col_width + padding);
                let line = format!("\n{}  > --{} | {}\n", spaces, name, description);
                lines.push_str(&line);
            }

            lines.push('\n');
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_flags;

    #[test]
    fn it_initializes_a_task() {
        let flags = BTreeMap::from([("foo".into(), "does the foo".into())]);
        let task = Task::new("test", "my test task", flags, |_, _, _| Ok(()));
        assert_eq!(task.name, "test");
        assert_eq!(task.description, "my test task");
    }

    #[test]
    fn it_executes_a_task() {
        let tasks = Tasks::new();
        let mut workspace = Workspace::new("fake-cargo", std::path::PathBuf::from("fake-root"));
        let flags = BTreeMap::from([("foo".into(), "does the foo".into())]);
        let task = Task::new("test", "my test task", flags, |_, _, _| Ok(()));
        task.exec(vec![], &mut workspace, &tasks).unwrap();
    }

    #[test]
    fn it_initializes_tasks() {
        let tasks = Tasks::new();
        assert_eq!(tasks.map.len(), 0);
    }

    #[test]
    fn it_add_a_task() {
        let mut tasks = Tasks::new();
        let flags = BTreeMap::from([("foo".into(), "does the foo".into())]);
        let task1 = Task::new("one", "task 01", flags.clone(), |_, _, _| Ok(()));
        let task2 = Task::new("two", "task 02", flags, |_, _, _| Ok(()));

        tasks.add(vec![task1, task2]);

        assert_eq!(tasks.map.len(), 2);
        assert_eq!(tasks.get("one").unwrap().description, "task 01");
        assert_eq!(tasks.get("two").unwrap().description, "task 02");
    }

    #[test]
    fn it_gets_a_task() {
        let mut tasks = Tasks::new();
        let flags = BTreeMap::from([("foo".into(), "does the foo".into())]);
        let task1 = Task::new("one", "task 01", flags.clone(), |_, _, _| Ok(()));
        let task2 = Task::new("two", "task 02", flags, |_, _, _| Ok(()));

        tasks.add(vec![task1, task2]);
        let task = tasks.get("one").unwrap();

        assert_eq!(task.name, "one");
        assert_eq!(task.description, "task 01");
        assert_eq!(tasks.map.len(), 2);
    }

    #[test]
    fn it_gets_help_text() {
        let mut tasks = Tasks::new();
        tasks.add(vec![
            Task {
                name: "one".into(),
                description: "task 01".into(),
                flags: task_flags! {
                    "foo" => "does the foo",
                    "bar" => "enables bar",
                },
                run: |_, _, _| Ok(()),
            },
            Task {
                name: "two".into(),
                description: "task 02".into(),
                flags: task_flags! {
                    "baz" => "invokes a baz",
                },
                run: |_, _, _| Ok(()),
            },
        ]);

        assert_eq!(
            tasks.help().unwrap(),
            [
                "> one....task 01",
                "",
                "         > --bar | enables bar",
                "",
                "         > --foo | does the foo",
                "",
                "> two....task 02",
                "",
                "         > --baz | invokes a baz",
                "",
                "",
            ]
            .join("\n")
        );
    }
}
