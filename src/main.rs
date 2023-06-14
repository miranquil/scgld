use std::process::{self, Output};
use std::process::{Command, Stdio};

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

    let (output, status_code) = spawn_target(&args).unwrap();

    filter_print(output.stdout, &args.keyword);
    filter_print(output.stderr, &args.keyword);

    if args.status {
        process::exit(status_code);
    }
}

fn spawn_target(args: &Args) -> Result<(Output, i32), &'static str> {
    let target_args = args.target_args.join(" ");
    let target_child = Command::new(&args.target)
        .arg(target_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn target");
    let output = target_child.wait_with_output().expect("failed to wait output");
    let status_code = output.status.code().expect("failed to get status code");

    Ok((output, status_code))
}

fn filter_print(out: Vec<u8>, keyword: &String) {
    let out_str = String::from_utf8(out).unwrap();
    for line in out_str.lines() {
        if line.contains(keyword) {
            print!("{}\n", line);
        }
    }
}
