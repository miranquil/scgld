use std::io::Write;
use std::process::{Command, Stdio};
use std::process;

use clap::Parser;

/// A filter for command line tools
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target binary name
    #[arg(long)]
    target: String,

    /// Keyword of filter
    #[arg(long)]
    keyword: String,

    /// If set, scgld will exit with target's status code
    #[arg(long)]
    status: bool,

    /// Target arguments
    target_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let target_args = args.target_args.join(" ");
    let target_child = Command::new(args.target)
        .arg(target_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute ls");
    let output = target_child.wait_with_output().expect("failed to get output");
    let status_code = output.status.code().unwrap();

    let mut grep_child = Command::new("grep")
        .arg(args.keyword)
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute grep");
    grep_child.stdin.as_mut().unwrap().write(&output.stdout).unwrap();
    let output = grep_child.wait_with_output().expect("failed to get output");
    let result = String::from_utf8(output.stdout).expect("failed to get utf8");
    print!("{}", result);

    if args.status {
        process::exit(status_code);
    }
}
