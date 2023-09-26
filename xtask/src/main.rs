mod krate;
mod readme;
mod tasks;
mod toml;
mod workspace;

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
    let mut workspace = Workspace::new();
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // drop executable path

    let tasks = init_tasks();
    let cmd = match args.get(0) {
        Some(x) => x.clone(),
        None => "".to_string(),
    };

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
    println!(":::: Available Tasks:");
    println!("{}", tasks.help()?);

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
            run: |args, workspace, tasks| {
                println!(":::::::::::::::::::::::::::::::::");
                println!(":::: Checking Project for CI ::::");
                println!(":::::::::::::::::::::::::::::::::");

                tasks
                    .get("lint")
                    .unwrap()
                    .exec(args.clone(), workspace, tasks)?;
                tasks
                    .get("coverage")
                    .unwrap()
                    .exec(args, workspace, tasks)?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            name: "clean".into(),
            description: "delete temporary files".into(),
            run: |_, workspace, _| {
                println!("::::::::::::::::::::::::::::");
                println!(":::: Cleaning Workspace ::::");
                println!("::::::::::::::::::::::::::::");

                workspace.clean().unwrap_or(());
                workspace.create_dirs()?;
                cmd!(&workspace.cargo, "clean").run()?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            // TODO (mirande): oof. coverage is a bit h0rked atm - see:
            // https://github.com/mozilla/grcov/issues/1103
            // https://github.com/mozilla/grcov/issues/556
            // https://github.com/mozilla/grcov/issues/802
            // https://github.com/mozilla/grcov/issues/1042
            name: "coverage".into(),
            description: "run tests and generate html code coverage report".into(),
            run: |args, workspace, tasks| {
                let coverage_root = PathBuf::from("tmp/coverage");
                tasks.get("clean").unwrap().exec(args, workspace, tasks)?;

                println!("::::::::::::::::::::::::::");
                println!(":::: Running Coverage ::::");
                println!("::::::::::::::::::::::::::");

                cmd!(&workspace.cargo, "test")
                    .env("CARGO_INCREMENTAL", "0")
                    .env("RUSTFLAGS", "-Cinstrument-coverage")
                    .env(
                        "LLVM_PROFILE_FILE",
                        format!("{}/cargo-test-%p-%m.profraw", coverage_root.display()),
                    )
                    .run()?;

                println!(":::: Done!\n");
                println!(":::::::::::::::::::::::::::");
                println!(":::: Generating Report ::::");
                println!(":::::::::::::::::::::::::::");

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

                println!(":::: Done!");
                println!(":::: Report: {}/html/index.html", coverage_root.display());
                Ok(())
            },
        },
        Task {
            name: "crate:add".into(),
            description: "add new crate to workspace".into(),
            run: |_args, workspace, _tasks| {
                println!(":::::::::::::::::::");
                println!(":::: Add Crate ::::");
                println!(":::::::::::::::::::");

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

                // TODO (mirande): find work-around to set `description`
                // see: https://github.com/rust-lang/cargo/issues/12736
                let question = InquireText::new("What does your crate do?");
                let description = question
                    .with_validator(required!())
                    .prompt()?;

                let question = InquireSelect::new("Crate type?", vec!["--lib", "--bin"]);
                let kind_flag = question.prompt()?;

                workspace.add_krate(kind_flag, &name, &description)?;

                Ok(())
            },
        },
        Task {
            name: "crate:list".into(),
            description: "list workspace crates".into(),
            run: |_, workspace, _| {
                println!("::::::::::::::::::::::::::");
                println!(":::: Available Crates ::::");
                println!("::::::::::::::::::::::::::");

                let krates = workspace.krates()?;

                for krate in krates.values() {
                    println!("* {}: {}", krate.name, krate.path.to_str().unwrap());
                }

                Ok(())
            },
        },
        Task {
            name: "dist".into(),
            description: "create release artifacts".into(),
            run: |_, workspace, _| {
                let dist_dir = workspace.path.join("target/release");
                println!(":::::::::::::::::::::::::::::::::::::::::::");
                println!(":::: Building Project for Distribution ::::");
                println!(":::::::::::::::::::::::::::::::::::::::::::");

                cmd!(&workspace.cargo, "build", "--release").run()?;

                println!(":::: Done!");
                println!(":::: Artifacts: {}", dist_dir.display());
                Ok(())
            },
        },
        Task {
            name: "doc".into(),
            description: "build project documentation".into(),
            run: |_, workspace, _| {
                println!(":::::::::::::::::::::::::::");
                println!(":::: Building All Docs ::::");
                println!(":::::::::::::::::::::::::::");

                println!();
                println!(":::: Updating Workspace README...");
                println!(":::: Done: {}", workspace.readme.path.display());

                let krates = workspace.krates()?;
                workspace.readme.update_crates_list(krates)?;

                println!();
                println!(":::: Testing Examples...");

                cmd!(&workspace.cargo, "test", "--doc").run()?;

                println!();
                println!(":::: Rendering Docs...");

                cmd!(
                    &workspace.cargo,
                    "doc",
                    "--workspace",
                    "--no-deps",
                    "--open"
                )
                .run()?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            name: "lint".into(),
            description: "run the linter (clippy)".into(),
            run: |_, workspace, _| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Linting Project ::::");
                println!(":::::::::::::::::::::::::");

                cmd!(
                    &workspace.cargo,
                    "clippy",
                    "--all-targets",
                    "--all-features",
                    "--no-deps"
                )
                .env("RUSTFLAGS", "-Dwarnings")
                .run()?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            name: "setup".into(),
            description: "bootstrap project for local development".into(),
            run: |_, workspace, _| {
                println!("::::::::::::::::::::::::::::");
                println!(":::: Setting up Project ::::");
                println!("::::::::::::::::::::::::::::");

                // TODO (mirande): "error: could not create link from
                // 'C:\Users\runneradmin\.cargo\bin\rustup.exe'
                // to 'C:\Users\runneradmin\.cargo\bin\cargo.exe'"
                // see: https://github.com/rust-lang/rustup/issues/1367
                //cmd!("rustup", "update").run()?;
                cmd!("rustup", "toolchain", "list", "--verbose").run()?;
                // TODO (mirande): is there a way to includes these in Cargo.toml or similar?
                cmd!("rustup", "component", "add", "clippy").run()?;
                cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;
                cmd!(&workspace.cargo, "install", "grcov").run()?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            name: "test".into(),
            description: "run all tests".into(),
            run: |_, workspace, _| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Testing Project ::::");
                println!(":::::::::::::::::::::::::");

                cmd!(&workspace.cargo, "test").run()?;

                println!(":::: Done!");
                Ok(())
            },
        },
        Task {
            name: "todo".into(),
            description: "list open to-dos based on inline source code comments".into(),
            run: |_, _, _| {
                println!(":::::::::::::::");
                println!(":::: TODOs ::::");
                println!(":::::::::::::::");
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
                Ok(())
            },
        },
    ]);

    tasks
}
