use anyhow::Result;
use rofi;
use std::{io::Read, os::unix::process::CommandExt, process::Command};

fn main() -> Result<()> {
    let dirs: Vec<String> = get_zoxide_list()?
        .iter()
        .map(|x| String::from(format!("{}", x)))
        .collect();

    let a = rofi::Rofi::new(&&dirs)
        .lines(10)
        .width(rofi::Width::Percentage(40))?
        .prompt("project to open in neovide:")
        .run()?;
    launch_neovide(&a)?;

    Ok(())
}

fn get_zoxide_list() -> Result<Vec<String>> {
    let output = Command::new("zoxide").args(["query", "-l"]).output()?;

    let mut buf: String = String::from("");

    output.stdout.as_slice().read_to_string(&mut buf)?;

    let result = format!("{}", &buf);

    let lined: Vec<String> = result
        .split('\n')
        .map(|x| String::from(x.trim().replace("/home/a/", "")))
        .collect();

    Ok(lined)
}

fn launch_neovide(path: &str) -> Result<()> {
    Err(Command::new("neovide").arg(path).exec().into())
}
