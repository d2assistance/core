// use std::process::{Command, ExitStatus};
use std::io::Result;
use tokio::process::{Child, Command};

// pub fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
//     Command::new(exe).args(args).spawn()?.wait()
// }

async fn execute(exe: &str, args: Option<&[&str]>) {
    let args = args.unwrap_or(&[]);

    match Command::new(exe).args(args).spawn() {
        Ok(mut child) => { child.wait().await; },
        Err(error) => {
            println!("Cannot spawn: {}", error.to_string());
        }
    }
}

#[tokio::main]
async fn main() {
    tokio::join!(
        execute("d2assist.exe", None),
        execute("sh", Some(&["assets/run-ui.sh"])),
        execute("sh", Some(&["assets/run-electron.sh"]))
    );
}
