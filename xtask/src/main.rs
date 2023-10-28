mod cargo;
mod changelog;
mod exec;
mod fs;
mod git;
mod krate;
mod options;
mod readme;
mod semver;
mod tasks;
mod toml;
mod workspace;

use crate::krate::{Krate, KratePaths};
use crate::semver::VersionChoice;
use crate::tasks::{Task, Tasks};
use duct::cmd;
use inquire::list_option::ListOption as InquireListOption;
use inquire::required;
use inquire::validator::Validation as InquireValidation;
use inquire::{MultiSelect as InquireMultiSelect, Select as InquireSelect, Text as InquireText};
use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;

type DynError = Box<dyn Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
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
        Some(task) => task.exec(args, &tasks),
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

    if !(cmd.is_empty() || cmd == "help" || cmd == "--help") {
        let msg = format!("Unrecognized Command! Received: '{}'", cmd);
        return Err(msg.into());
    }

    Ok(())
}

fn init_tasks() -> Tasks {
    let mut tasks = Tasks::new();

    tasks.add(vec![
        Task {
            name: "changelog".into(),
            description: "view changelog entries for the next version of all crates".into(),
            flags: task_flags! {},
            run: |_opts, fs, git, _cargo, workspace, _tasks| {
                println!(":::::::::::::::::::::::::::::::::::::");
                println!(":::: Viewing Unpublished Changes ::::");
                println!(":::::::::::::::::::::::::::::::::::::");
                println!();

                let krates = workspace.krates(&fs)?;
                let tags_text = git.tag(["--list", "--sort=v:refname"]).read()?;
                let mut tags: BTreeMap<String, String> = BTreeMap::new();

                for tag in tags_text.lines() {
                    let (name, version) = match tag.split_once('@') {
                        None => return Err(format!("Invalid tag: {}", tag).into()),
                        Some((n, v)) => (n.trim().to_string(), v.trim().to_string()),
                    };

                    tags.insert(name, version);
                }

                for (name, _version) in tags.iter() {
                    let krate = krates.get(name).unwrap_or_else(|| panic!("Could Not Find Crate: `{}`!", name));
                    let log = git.get_changelog(krate)?;

                    println!(":::: {} [changes: {}]", &krate.name, log.len());

                    if log.is_empty() {
                        println!("\t--- n/a ---");
                        println!();
                        continue;
                    }


                    for l in log.iter() {
                        println!("* {}", l);
                    }

                    println!();
                }

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "ci".into(),
            description: "run checks for CI".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, _cargo, _workspace, tasks| {
                println!(":::::::::::::::::::::::::::::::::");
                println!(":::: Checking Project for CI ::::");
                println!(":::::::::::::::::::::::::::::::::");
                println!();

                tasks
                    .get("spellcheck")
                    .unwrap()
                    .exec(vec![], tasks)?;
                tasks
                    .get("lint")
                    .unwrap()
                    .exec(vec![], tasks)?;
                tasks
                    .get("coverage")
                    .unwrap()
                    .exec(vec![], tasks)?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "clean".into(),
            description: "delete temporary files".into(),
            flags: task_flags! {},
            run: |_opts, fs, _git, cargo, workspace, _tasks| {
                println!("::::::::::::::::::::::::::::");
                println!(":::: Cleaning Workspace ::::");
                println!("::::::::::::::::::::::::::::");
                println!();

                workspace.clean(&fs, &cargo)?;
                workspace.create_dirs(&fs)?;

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
            run: |opts, _fs, _git, cargo, _workspace, tasks| {
                println!("::::::::::::::::::::::::::::::");
                println!(":::: Calculating Coverage ::::");
                println!("::::::::::::::::::::::::::::::");
                println!();

                let coverage_root = String::from("tmp/coverage");
                let report = format!("{}/html/index.html", &coverage_root);

                tasks.get("clean").unwrap().exec(vec![], tasks)?;
                cargo.coverage(&coverage_root).run()?;

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
                    cmd!("open", &report).run()?;
                }

                println!(":::: Report: {}", report);
                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "crate:add".into(),
            description: "add new crate to workspace".into(),
            flags: task_flags! {
                "dry-run" => "run thru steps but do not create new crate"
            },
            run: |_opts, fs, _git, cargo, workspace, _tasks| {
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
                let krate = Krate::new(
                    kind_flag,
                    "0.1.0",
                    &name,
                    description,
                    workspace.krates_path().join(&name)
                );

                workspace.add_krate(&fs, &cargo, krate)?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "crate:list".into(),
            description: "list workspace crates".into(),
            flags: task_flags! {},
            run: |_opts, fs, _git, _cargo, workspace, _tasks| {
                println!("::::::::::::::::::::::::::");
                println!(":::: Available Crates ::::");
                println!("::::::::::::::::::::::::::");
                println!();

                let krates = workspace.krates(&fs)?;

                for krate in krates.values() {
                    let kind = krate.kind.to_string().replace('-', "");
                    println!("* {} [{}]\n  ?? {}\n  >> {}\n", krate.name, kind, krate.description, krate.path.display());
                }

                println!();
                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "crate:publish".into(),
            description: "publish released crates to crates.io".into(),
            flags: task_flags! {
                "dry-run" => "run thru steps but do not publish"
            },
            run: |_opts, fs, git, cargo, workspace, _tasks| {
                println!(":::::::::::::::::::::::::::");
                println!(":::: Publishing Crates ::::");
                println!(":::::::::::::::::::::::::::");
                println!();

                let krates = workspace.krates(&fs)?;
                let tag_text = git.tag(["--points-at", "HEAD"]).read()?;
                let mut tags = vec![];

                for line in tag_text.lines() {
                    if line.contains('@') {
                        tags.push(line);
                    }
                }

                if tags.is_empty() {
                    println!(":::: Nothing to publish");
                    println!(":::: Done!");
                    println!();
                    return Ok(())
                }

                for tag in tags {
                    let (name, _ver) = tag.split_once('@').unwrap_or_else(|| panic!("Invalid Tag: `{}`!", tag));
                    let krate = krates.get(name).unwrap_or_else(|| panic!("Could Not Find Crate: `{}`!", name));
                    let message = format!("Publishing: {} at v{}", &krate.name, &krate.version);
                    println!("{}", &message);
                    cargo.publish_package(&krate.name).run()?;
                }

                println!();
                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "crate:release".into(),
            description: "prepate crates for publishing".into(),
            flags: task_flags! {
                "dry-run" => "run thru steps but do not save changes"
            },
            run: |_opts, fs, git, _cargo, workspace, _tasks| {
                println!("::::::::::::::::::::::::::");
                println!(":::: Releasing Crates ::::");
                println!("::::::::::::::::::::::::::");
                println!();

                let mut krates = workspace.krates(&fs)?;
                let question = InquireMultiSelect::new("Which crates should be published?", krates.keys().cloned().collect());
                let to_publish = question
                    .with_validator(|selections: &[InquireListOption<&String>]| {
                        if selections.is_empty() {
                            return Ok(InquireValidation::Invalid("Please select at least one crate!".into()));
                        }

                        Ok(InquireValidation::Valid)
                    })
                    .prompt()?;

                krates.retain(|_, k| to_publish.contains(&k.name));
                let mut tags: Vec<String> = Vec::new();
                for mut krate in krates.values().cloned() {
                    let log = git.get_changelog(&krate)?;
                    let version = krate.toml.get_version()?;
                    let options = VersionChoice::options(&version);
                    let message = format!("Version for `{}` [current: {}]", krate.name, version);
                    let question = InquireSelect::new(&message, options);
                    let choice = question.prompt()?;
                    krate.set_version(choice.get_version())?;
                    krate.changelog.update(&fs, &krate.clone(), log)?;
                    krate.toml.save(&fs)?;
                    git.add(&krate.changelog.path, [""]).run()?;
                    git.add(&krate.toml.path, [""]).run()?;
                    tags.push(krate.id());
                }

                let message = format!("Release:\n{}", tags.join("\n"));
                git.commit(message, [""]).run()?;

                for tag in tags {
                    git.create_tag(tag).run()?;
                }

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "dist".into(),
            description: "create release artifacts".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, cargo, workspace, _tasks| {
                println!(":::::::::::::::::::::::::::::::::::::::::::");
                println!(":::: Building Project for Distribution ::::");
                println!(":::::::::::::::::::::::::::::::::::::::::::");
                println!();

                let dist_dir = workspace.path().join("target/release");
                cargo.build(["--release"]).run()?;

                println!(":::: Artifacts: {}", dist_dir.display());
                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "doc".into(),
            description: "build project documentation".into(),
            flags: task_flags! {
                "dry-run" => "run thru steps but do not generate docs",
                "open" => "open rendered docs for viewing"
            },
            run: |opts, fs, _git, cargo, mut workspace, _tasks| {
                println!(":::::::::::::::::::::::::::");
                println!(":::: Building All Docs ::::");
                println!(":::::::::::::::::::::::::::");
                println!();
                println!(":::: Testing Examples...");
                println!();

                cargo.test(["--doc", "--all-features"]).run()?;

                println!(":::: Rendering Docs...");
                println!();

                let mut args = vec!["--workspace", "--no-deps", "--all-features"];

                if opts.has("open") {
                    args.push("--open");
                }

                cargo.doc(args).run()?;

                println!();
                println!(":::: Updating Workspace README...");

                let krates = workspace.krates(&fs)?;
                let readme_path = workspace.readme.path.clone();

                workspace.readme.update_crates_list(&fs, krates)?;

                println!(":::: Updated: {:?}", readme_path);

                if opts.has("open") {
                    cmd!("open", readme_path.to_str().unwrap()).run()?;
                }

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "lint".into(),
            description: "run the linter (clippy)".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, cargo, _workspace, _tasks| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Linting Project ::::");
                println!(":::::::::::::::::::::::::");
                println!();

                cargo.lint().run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "setup".into(),
            description: "bootstrap project for local development".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, cargo, _workspace, _tasks| {
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
                cargo.install(["grcov"]).run()?;
                cargo.install(["typos-cli"]).run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "spellcheck".into(),
            description: "finds spelling mistakes in source code and docs".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, _cargo, _workspace, _tasks| {
                println!(":::::::::::::::::::::::::::");
                println!(":::: Checking Spelling ::::");
                println!(":::::::::::::::::::::::::::");
                println!();

                cmd!("typos").run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "test".into(),
            description: "run all tests".into(),
            flags: task_flags! {},
            run: |_opts, _fs, _git, cargo, _workspace, _tasks| {
                println!(":::::::::::::::::::::::::");
                println!(":::: Testing Project ::::");
                println!(":::::::::::::::::::::::::");
                println!();

                cargo.test(["--all-features"]).run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
        Task {
            name: "todo".into(),
            description: "list open to-dos based on inline source code comments".into(),
            flags: task_flags! {},
            run: |_opts, _fs, git, _cargo, _workspace, _tasks| {
                println!(":::::::::::::::");
                println!(":::: TODOs ::::");
                println!(":::::::::::::::");
                println!();

                git.todos().run()?;

                println!(":::: Done!");
                println!();
                Ok(())
            },
        },
    ]);

    tasks
}
