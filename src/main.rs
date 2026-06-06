use bitcode::{Decode, Encode};
use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use itertools::Itertools as _;

#[derive(Parser)]
#[command(name = "comma-cache")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(alias = "ls", about = "List cache entries (default)")]
    List,
    #[command(alias = "rm", about = "Remove a cache entry")]
    Remove {
        /// The cache key to remove
        key: String,
    },
}

#[derive(Encode, Decode)]
struct CacheData(std::collections::HashMap<String, CacheEntry>);

#[derive(Encode, Decode, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CacheEntry {
    pub derivation: String,
    pub path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    let path = xdg::BaseDirectories::new()
        .get_state_file("comma/choices")
        .ok_or_else(|| eyre!("cache file not found"))?;
    let bytes = std::fs::read(&path)?;
    let mut data = bitcode::decode::<CacheData>(&bytes)?;

    match args.command.unwrap_or(Commands::List) {
        Commands::List => {
            for (k, v) in data.0.into_iter().sorted() {
                println!("{k} => {}", v.derivation);
            }
        }
        Commands::Remove { key } => {
            let value = data
                .0
                .remove(&key)
                .ok_or_else(|| eyre!("Key {key} not found in the cache"))?;
            let encoded = bitcode::encode(&data);
            let mut tmp = tempfile::Builder::new()
                .prefix("choices.tmp.")
                .tempfile_in(path.parent().expect("path has no parent"))?;
            std::io::Write::write_all(&mut tmp, &encoded)?;
            tmp.persist(&path)?;
            println!("Removed: {key} => {}", value.derivation);
        }
    }
    Ok(())
}
