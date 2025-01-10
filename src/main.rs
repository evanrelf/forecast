use clap::Parser as _;

#[derive(clap::Parser, Debug)]
struct Args {}

fn main() {
    let args = Args::parse();

    println!("{args:?}");
}
