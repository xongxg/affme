use affme::affirm;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let name = Args::parse().name;
    let output = affirm(&name);
    println!("{}", output);
}
