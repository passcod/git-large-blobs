use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Stop looking for blobs smaller than this in bytes
    #[arg(long, default_value_t = 0)]
    cutoff: u64,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
    println!("Hello, world!");
}
