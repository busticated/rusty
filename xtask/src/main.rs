mod workspace;

use duct::cmd;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use workspace::Workspace;

type DynError = Box<dyn Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let workspace = Workspace::new(cargo);

    match task.as_deref() {
        Some("ci") => ci(&workspace)?,
        Some("clean") => clean(&workspace)?,
        Some("coverage") => coverage(&workspace)?,
        Some("crates") => crates(&workspace)?,
        Some("doc") => doc(&workspace)?,
        Some("dist") => dist(&workspace)?,
        Some("lint") => lint(&workspace)?,
        Some("setup") => setup(&workspace)?,
        Some("test") => test(&workspace)?,
        _ => help(),
    }

    Ok(())
}

// COMMANDS ///////////////////////////////////////////////////////////////////
fn help() {
    eprintln!(":::::::::::::::::::::::::::");
    eprintln!(":::: Tasks & Options ::::");
    eprintln!(":::::::::::::::::::::::::::");
    eprintln!(
        "
:::: Available Tasks:
> ci\t\trun checks for CI
> clean\tdelete temporary files
> coverage\trun tests and generate html code coverage report
> crates\tlist workspace crates
> doc\t\tbuild project documentation
> dist\t\tcreate release artifacts
> lint\t\trun the linter (clippy)
> setup\t\tbootstrap project for local development
> test\t\trun all tests
"
    )
}

fn ci(workspace: &Workspace) -> Result<(), DynError> {
    println!(":::::::::::::::::::::::::::::::::");
    println!(":::: Checking Project for CI ::::");
    println!(":::::::::::::::::::::::::::::::::");

    lint(workspace)?;
    coverage(workspace)?;

    println!(":::: Done!");

    Ok(())
}
fn clean(workspace: &Workspace) -> Result<(), DynError> {
    println!("::::::::::::::::::::::::::::");
    println!(":::: Cleaning Workspace ::::");
    println!("::::::::::::::::::::::::::::");

    workspace.clean().unwrap_or(());
    workspace.create_dirs()?;

    println!(":::: Done!");

    Ok(())
}

fn coverage(workspace: &Workspace) -> Result<(), DynError> {
    let coverage_root = PathBuf::from("tmp/coverage");
    clean(workspace)?;

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
}

fn crates(workspace: &Workspace) -> Result<(), DynError> {
    println!("::::::::::::::::::::::::::");
    println!(":::: Available Crates ::::");
    println!("::::::::::::::::::::::::::");

    let crates = workspace.crates()?;

    for c in crates.values() {
        println!("* {}: {}", c.name, c.path.to_str().unwrap());
    }

    Ok(())
}

fn doc(workspace: &Workspace) -> Result<(), DynError> {
    println!(":::::::::::::::::::::::::::::::");
    println!(":::: Building Project Docs ::::");
    println!(":::::::::::::::::::::::::::::::");

    cmd!(&workspace.cargo, "test", "--doc").run()?;
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
}

fn dist(workspace: &Workspace) -> Result<(), DynError> {
    let dist_dir = workspace.root()?.join("target/release");
    println!(":::::::::::::::::::::::::::::::::::::::::::");
    println!(":::: Building Project for Distribution ::::");
    println!(":::::::::::::::::::::::::::::::::::::::::::");

    cmd!(&workspace.cargo, "build", "--release").run()?;

    println!(":::: Done!");
    println!(":::: Artifacts: {}", dist_dir.display());
    Ok(())
}

fn lint(workspace: &Workspace) -> Result<(), DynError> {
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
}

fn setup(workspace: &Workspace) -> Result<(), DynError> {
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
}

fn test(workspace: &Workspace) -> Result<(), DynError> {
    println!("::::::::::::::::::::::::::::");
    println!(":::: Testing Project ::::");
    println!("::::::::::::::::::::::::::::");

    cmd!(&workspace.cargo, "test").run()?;

    println!(":::: Done!");
    Ok(())
}
