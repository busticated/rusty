mod krate;
mod options;
mod readme;
mod tasks;
mod toml;
mod workspace;

use crate::krate::KratePaths;
use crate::tasks::{Task, Tasks};
use crate::workspace::Workspace;
use duct::cmd;
use inquire::required;
use inquire::validator::Validation as InquireValidation;
use inquire::{Select as InquireSelect, Text as InquireText};
use regex::RegexBuilder;
use std::env;
use std::error::Error;
use std::path::PathBuf;

type DynError = Box<dyn Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let cargo_cmd = get_cargo_cmd();
    let root_path = get_root_path(&cargo_cmd)?;
    let mut workspace = Workspace::from_path(cargo_cmd, root_path)?;
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // drop executable path

    let cmd = match args.get(0) {
        Some(x) => x.clone(),
        None => "".to_string(),
    };

    if !args.is_empty() {
        args.remove(0); // drop task name / cmd
    }

    println!("::::::::::::::::::::::");
    println!(":::: Running Task ::::");
    println!("::::::::::::::::::::::");
    println!("Name: {}", cmd);
    println!("Args: {:?}", args);
    println!();

    let tasks = init_tasks();
    match tasks.get(cmd.clone()) {
        Some(task) => task.exec(args, &mut workspace, &tasks),
        None => print_help(cmd, args, tasks),
    }
}

fn print_help(cmd: String, _args: Vec<String>, tasks: Tasks) -> Result<(), DynError> {
    println!(":::::::::::::::::::::::::");
    println!(":::: Tasks & Options ::::");
    println!(":::::::::::::::::::::::::");
    println!();
    println!("{}", tasks.help()?);
    println!();

    if !cmd.is_empty() && cmd != "help" {
        let msg = format!("Unrecognized Command! Received: '{}'", cmd);
        return Err(msg.into());
    }

    Ok(())
}

fn init_tasks() -> Tasks {
    let mut tasks = Tasks::new();

    tasks.add(vec![
        Task {
            name: "ci".into(),
            description: "run checks for CI".into(),
            flags: task_flags! {},
            run: |_opts, workspace, tasks| {
                println!(":::::::::::::::::::::::::::::::::");
                println!(":::: Checking Project for CI ::::");
                println!(":::::::::::::::::::::::::::::::::");
                println!();

                tasks
                    .get("lint")
                    .unwrap()
                    .exec(vec![], workspace, tasks)?;
                tasks
                    .get("coverage")
                    .unwrap()
                    .exec(vec![], workspace, tasks)?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "clean".into(),
            description: "delete temporary files".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!("::::::::::::::::::::::::::::");
                println!(":::: Cleaning Workspace ::::");
                println!("::::::::::::::::::::::::::::");
                println!();

                workspace.clean().unwrap_or(()); // ignore error
                workspace.create_dirs()?;
                cmd!(&workspace.cargo_cmd, "clean", "--release").run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            // TODO (busticated): oof. coverage is a bit h0rked atm - see:
            // https://github.com/mozilla/grcov/issues/1103
            // https://github.com/mozilla/grcov/issues/556
            // https://github.com/mozilla/grcov/issues/802
            // https://github.com/mozilla/grcov/issues/1042
            name: "coverage".into(),
            description: "run tests and generate html code coverage report".into(),
            flags: task_flags! {
                "open" => "open coverage report for viewing"
            },
            run: |opts, workspace, tasks| {
                println!("::::::::::::::::::::::::::::::");
                println!(":::: Calculating Coverage ::::");
                println!("::::::::::::::::::::::::::::::");
                println!();

                let coverage_root = PathBuf::from("tmp/coverage").display().to_string();
                let report = format!("{}/html/index.html", &coverage_root);

                tasks.get("clean").unwrap().exec(vec![], workspace, tasks)?;

                cmd!(&workspace.cargo_cmd, "test")
                    .env("CARGO_INCREMENTAL", "0")
                    .env("RUSTFLAGS", "-Cinstrument-coverage")
                    .env(
                        "LLVM_PROFILE_FILE",
                        format!("{}/cargo-test-%p-%m.profraw", &coverage_root),
                    )
                    .run()?;

                println!(":::: Done!");
                println!();
                println!(":::::::::::::::::::::::::::");
                println!(":::: Generating Report ::::");
                println!(":::::::::::::::::::::::::::");
                println!();

                cmd!(
                    "grcov",
                    ".",
                    "--binary-path",
                    "./target/debug/deps",
                    "--source-dir",
                    ".",
                    "--output-types",
                    "html,lcov",
                    "--branch",
                    "--ignore-not-existing",
                    "--ignore",
                    "../*",
                    "--ignore",
                    "/*",
                    "--ignore",
                    "xtask/*",
                    "--ignore",
                    "*/tests/*",
                    "--output-path",
                    &coverage_root,
                )
                .run()?;


                if opts.has("open"){
                    cmd!("open", report).run()?;
                } else {
                    println!(":::: Done!");
                    println!(":::: Report: {}", report);
                    println!();
                }

                Ok(())
            },
        },
        Task {
            name: "crate:add".into(),
            description: "add new crate to workspace".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!(":::::::::::::::::::");
                println!(":::: Add Crate ::::");
                println!(":::::::::::::::::::");
                println!();

                let question = InquireText::new("Crate name?");
                let name = question
                    .with_validator(required!())
                    .with_validator(|input: &str| {
                        let ptn = r"^[a-z0-9-]*$";
                        let re = RegexBuilder::new(ptn).build()?;

                        if re.is_match(input) {
                            Ok(InquireValidation::Valid)
                        } else {
                            Ok(InquireValidation::Invalid(
                                "name must be dash delimited lowercase alphanumeric - e.g. 'my-crate-01'".into(),
                            ))
                        }
                    })
                    .prompt()?;

                let question = InquireText::new("What does your crate do?");
                let description = question
                    .with_validator(required!())
                    .prompt()?;

                let question = InquireSelect::new("Crate type?", vec!["--lib", "--bin"]);
                let kind_flag = question.prompt()?;

                workspace.add_krate(kind_flag, &name, &description)?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "crate:list".into(),
            description: "list workspace crates".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!("::::::::::::::::::::::::::");
                println!(":::: Available Crates ::::");
                println!("::::::::::::::::::::::::::");
                println!();

                let krates = workspace.krates()?;

                for krate in krates.values() {
                    println!("* {}: {}", krate.name, krate.path.display());
                }

                println!();
                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "dist".into(),
            description: "create release artifacts".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!(":::::::::::::::::::::::::::::::::::::::::::");
                println!(":::: Building Project for Distribution ::::");
                println!(":::::::::::::::::::::::::::::::::::::::::::");
                println!();

                let dist_dir = workspace.path().join("target/release");
                cmd!(&workspace.cargo_cmd, "build", "--release").run()?;

                println!(":::: Done!");
                println!(":::: Artifacts: {}", dist_dir.display());
                println!();

                Ok(())
            },
        },
        Task {
            name: "doc".into(),
            description: "build project documentation".into(),
            flags: task_flags! {
                "open" => "open rendered docs for viewing"
            },
            run: |opts, workspace, _tasks| {
                println!(":::::::::::::::::::::::::::");
                println!(":::: Building All Docs ::::");
                println!(":::::::::::::::::::::::::::");
                println!();
                println!(":::: Updating Workspace README...");
                println!(":::: Done: {}", workspace.readme.path.display());
                println!();

                let krates = workspace.krates()?;
                workspace.readme.update_crates_list(krates)?;

                println!(":::: Testing Examples...");
                println!();

                cmd!(&workspace.cargo_cmd, "test", "--doc").run()?;

                println!(":::: Rendering Docs...");
                println!();

                let mut args = vec!["doc", "--workspace", "--no-deps"];

                if opts.has("open") {
                    args.push("--open");
                }

                cmd(&workspace.cargo_cmd, args).run()?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "lint".into(),
            description: "run the linter (clippy)".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Linting Project ::::");
                println!(":::::::::::::::::::::::::");
                println!();

                cmd!(
                    &workspace.cargo_cmd,
                    "clippy",
                    "--all-targets",
                    "--all-features",
                    "--no-deps"
                )
                .env("RUSTFLAGS", "-Dwarnings")
                .run()?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "setup".into(),
            description: "bootstrap project for local development".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!("::::::::::::::::::::::::::::");
                println!(":::: Setting up Project ::::");
                println!("::::::::::::::::::::::::::::");
                println!();

                // TODO (busticated): "error: could not create link from
                // 'C:\Users\runneradmin\.cargo\bin\rustup.exe'
                // to 'C:\Users\runneradmin\.cargo\bin\cargo.exe'"
                // see: https://github.com/rust-lang/rustup/issues/1367
                //cmd!("rustup", "update").run()?;
                cmd!("rustup", "toolchain", "list", "--verbose").run()?;
                // TODO (busticated): is there a way to includes these in Cargo.toml or similar?
                cmd!("rustup", "component", "add", "clippy").run()?;
                cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;
                cmd!(&workspace.cargo_cmd, "install", "grcov").run()?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "test".into(),
            description: "run all tests".into(),
            flags: task_flags! {},
            run: |_opts, workspace, _tasks| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Testing Project ::::");
                println!(":::::::::::::::::::::::::");
                println!();

                cmd!(&workspace.cargo_cmd, "test").run()?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
        Task {
            name: "todo".into(),
            description: "list open to-dos based on inline source code comments".into(),
            flags: task_flags! {},
            run: |_opts, _workspace, _tasks| {
                println!(":::::::::::::::");
                println!(":::: TODOs ::::");
                println!(":::::::::::::::");
                println!();

                // so we don't include this fn in the list (x_X)
                let mut ptn = String::from("TODO");
                ptn.push_str(" (.*)");

                cmd!(
                    "git",
                    "grep",
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
                )
                .run()?;

                println!(":::: Done!");
                println!();

                Ok(())
            },
        },
    ]);

    tasks
}

fn get_cargo_cmd() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn get_root_path<T: AsRef<str>>(cargo_cmd: T) -> Result<PathBuf, DynError> {
    let stdout = cmd!(
        cargo_cmd.as_ref().to_owned(),
        "locate-project",
        "--workspace",
        "--message-format",
        "plain",
    )
    .read()?;

    Ok(PathBuf::from(stdout.replace("Cargo.toml", "").trim()))
}
