use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    target: String,
    #[arg(long)]
    keyword: String,
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

    let grep_child = Command::new("grep")
        .arg(args.keyword)
        .stdin(Stdio::from(target_child.stdout.unwrap()))
        .spawn()
        .expect("failed to execute grep");

    let output = grep_child.wait_with_output().expect("failed to get output");
    let result = String::from_utf8(output.stdout).expect("failed to get utf8");
    print!("{}", result)
}
