use args::Arguments;
use bloomfilter::Bloom_Filter;
use clap::Parser;
use std::time::Instant;

mod bloomfilter;
mod args;

#[tokio::main]
async fn main() {
    let mut bloomfilter = Bloom_Filter::new();

    let args = Arguments::parse();

    if args.mode == "bloom" {
        let start = Instant::now();
        let _ = bloomfilter.load_filter();
        let _ = bloomfilter.check_file_bf(&args.file).await;
        let duration = start.elapsed();
        println!("Bloom filter check completed in: {:?}", duration);
    } else if args.mode == "db" {
        let start = Instant::now();
        let _ = bloomfilter.check_file_db(&args.file).await;
        let duration = start.elapsed();
        println!("Database check completed in: {:?}", duration);
    } else {
        eprintln!("Invalid mode. Use 'bloom' or 'db'.");
    }
}