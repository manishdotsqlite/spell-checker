use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
        /// The file to check for spelling errors
        #[arg(short, long)]
        pub file: String,

        /// This selects whether to use the bloom filter or database. 
        #[arg(short, long, default_value = "bloom")]
        pub mode: String,
}