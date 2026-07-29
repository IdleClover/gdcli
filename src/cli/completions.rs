use std::{
    env,
    fs::{self, File},
    io::Write,
};

use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::{
    cli::Cli,
    error::{Result, fs::FsError, path::PathError, shell::ShellError},
};

pub fn handle(shell: Option<Shell>, install: bool) -> Result<()> {
    let shell = shell.or(guess_shell());
    let shell = shell.ok_or(ShellError::NotFound)?;

    if !install {
        return generate_gdcli(shell, &mut std::io::stdout());
    }

    match shell {
        Shell::Bash => generate_bash(),
        _ => Err(ShellError::AutoInstallationNotSupported.into()),
    }
}

fn guess_shell() -> Option<Shell> {
    let shell_path = env::var("SHELL").ok()?;
    let shell_name = shell_path.rsplit("/").next()?;

    match shell_name {
        "bash" => Some(Shell::Bash),
        "zsh" => Some(Shell::Zsh),
        "fish" => Some(Shell::Fish),
        "elvish" => Some(Shell::Elvish),
        _ => None,
    }
}

fn generate_gdcli(shell: Shell, buf: &mut dyn Write) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "gdcli", buf);
    Ok(())
}

fn generate_bash() -> Result<()> {
    let path = dirs::data_local_dir()
        .ok_or_else(|| PathError::NotFound("No local data dir found".into()))?
        .join("bash-completion/completions/gdcli");

    let parent = path.parent().ok_or(PathError::NoParent(path.clone()))?;

    fs::create_dir_all(parent).map_err(FsError::CreateDirsFailed)?;

    let mut file = File::create(&path).map_err(|e| FsError::CreateFailed {
        path: path.clone(),
        source: e,
    })?;
    generate_gdcli(Shell::Bash, &mut file)
}
