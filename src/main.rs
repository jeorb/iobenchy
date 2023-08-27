use std::{path::PathBuf, fs::File};
use std::io::Write;

use clap::{Parser, Subcommand};

use std::iter::repeat_with;
use fastrand;

use std::time::Instant;

/// Benchmark storage and network latency and bandwidth
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Test file IO
    File {
        /// Path to the directory to benchmark
        #[arg(long, value_name = "PATH", default_value = "./")]
        path: PathBuf,

        /// Number of bytes to write
        #[arg(long, value_name = "BYTES", default_value_t = 4 * 1024 * 1024)]
        bytes: usize,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::File { path, bytes }) => benchmark_directory(path, bytes.clone()),
        None => Ok({})
    }
}

fn random_bytes(length: usize) -> Vec<u8> {
    let mut rng = fastrand::Rng::new();
    repeat_with(|| rng.u8(..)).take(length).collect()
}

fn benchmark_directory(path: &PathBuf, num_bytes: usize) -> std::io::Result<()>{
    println!("Benchmarking disk IO for '{}'", path.display());
    let filename = path.join(".iobenchy.tmp1");
    let mut f = File::create(filename.clone())?;

    println!("Generating {} bytes of random data.", num_bytes);
    let start = Instant::now();
    let bytes = random_bytes(num_bytes);
    let duration = start.elapsed();
    println!("Generated {} bytes of random data in {:?}.", num_bytes, duration);

    println!("Writing {} bytes of random data to {}.", num_bytes, filename.display());
    let start = Instant::now();
    f.write_all(&bytes)?;
    let flush_result = f.flush();
    drop(f);
    let duration = start.elapsed();
    match flush_result {
        Ok(()) => {
            println!("Wrote {} bytes of random data to {} in {:?}.", num_bytes, filename.display(), duration);
        },
        Err(e) => {
            println!("Error flushing {} bytes of random data to {} (took {:?}). {}.", num_bytes, filename.display(), duration, e);
        },
    }
    Ok(())
}