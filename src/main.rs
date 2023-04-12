use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use actix_cli::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Check if git is installed
    Command::new("git").arg("--version").output()?;
    // Check if cache path exists
    println!("{:?}", Path::new("~/.actix-template/").exists());

    if !Path::new("~/.actix-template/").exists() {
        // Create cache path
        fs::create_dir_all("~/.actix-template/")?;
        println!("Cache path created at ~/.actix-template/");
        // Clone template repo
        Command::new("git")
            .arg("clone")
            .arg("git@github.com:yinho999/actix-template.git")
            .arg("~/.actix-template/")
            .output()?;

        println!("Template cloned to ~/.actix-template/");
    }
    match cli.subcmd {
        actix_cli::cli::subcommand::SubCommand::Init(args) => {
            // Create project path
            Command::new("mkdir").arg("-p").arg(&args.name).output()?;
            // Copy template to project path
            Command::new("cp")
                .arg("-r")
                .arg("~/.actix-template/*")
                .arg(&args.name)
                .output()?;

            // loop through files in project and replace actix-template with project name
            find_and_replace_dir(Path::new(&args.name), "actix-template", &args.name)?;
            // Change directory to project path
            Command::new("cd").arg(&args.name).output()?;
        }
    }
    Ok(())
}

fn find_and_replace_dir(path: &Path, from: &str, to: &str) -> Result<()> {
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_and_replace_dir(&path, from, to)?;
        } else {
            find_and_replace_file(&path, from, to)?;
        }
    }
    Ok(())
}

fn find_and_replace_file(path: &Path, from: &str, to: &str) -> Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(from, to);
    file.write_all(contents.as_bytes())?;
    Ok(())
}
