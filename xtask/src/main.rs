use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command},
};

use man::prelude::*;

const APP_NAME: &'static str = "xtask-demo";

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist_task()?,
        Some("hello") => hello_task(),
        Some("--help") => print_help(),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    println!("{}", r#"Tasks:
  hello - hello task
  dist - builds application and man pages
"#.trim()
    )
}

fn hello_task() {
    println!("Hello, task!");
}

fn dist_task() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary()?;
    dist_manpage()?;

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }
    let dst = project_root().join(format!("target/release/{}", APP_NAME));
    fs::copy(&dst, dist_dir().join(APP_NAME))?;
    Ok(())
}

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new(APP_NAME)
        .about("Greets the world")
        .render();
    fs::write(dist_dir().join(format!("{}.man", APP_NAME)), &page.to_string())?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
