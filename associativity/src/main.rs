use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use indicatif::ProgressBar;
use std::fs;
use std::io::Write;
use std::fs::File;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Minimum value of the range
    #[arg(short = 'l', long, default_value_t = -1000.0, allow_hyphen_values = true)]
    range_min: f64,

    /// Maximum value of the range
    #[arg(short = 'u', long, default_value_t = 1000.0)]
    range_max: f64,

    /// Number of samples to test
    #[arg(short = 'n', long, default_value_t = 1_000_000)]
    samples: usize,

    /// Random number generator seed (optional)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Output file path
    #[arg(short, long, default_value = "./associativity_test.txt")]
    output: String,

    /// Float type to test (32 or 64)
    #[arg(short = 't', long, default_value_t = 64)]
    float_type: usize,
}

fn associativity_32(a: f32, b: f32, c: f32) -> bool {
    (a + b) + c == a + (b + c)
}

fn associativity_64(a: f64, b: f64, c: f64) -> bool {
    (a + b) + c == a + (b + c)
}

fn test_associativity_32(nb: usize, range_min: f32, range_max: f32, file: &mut File, rng: &mut impl Rng) {
    let mut bad_cpt = 0_usize;
    let bar = ProgressBar::new(nb as u64);
    println!("Testing float32 from {range_min} to {range_max} with {nb} samples");
    writeln!(file, "Testing float32 from {range_min} to {range_max} with {nb} samples")
        .expect("Unable to write file");
    
    for _ in 0..nb {
        let a = rng.gen_range(range_min..=range_max);
        let b = rng.gen_range(range_min..=range_max);
        let c = rng.gen_range(range_min..=range_max);
        if !associativity_32(a, b, c) {
            bad_cpt += 1;
        }
        bar.inc(1);
    }
    
    bar.finish();
    println!("Failure rate: {:.6}%\n", bad_cpt as f64 / nb as f64 * 100.0);
    writeln!(file, "Failure rate: {:.6}%\n", bad_cpt as f64 / nb as f64 * 100.0)
        .expect("Unable to write file");
}

fn test_associativity_64(nb: usize, range_min: f64, range_max: f64, file: &mut File, rng: &mut impl Rng) {
    let mut bad_cpt = 0_usize;
    let bar = ProgressBar::new(nb as u64);
    println!("Testing float64 from {range_min} to {range_max} with {nb} samples");
    writeln!(file, "Testing float64 from {range_min} to {range_max} with {nb} samples")
        .expect("Unable to write file");
    
    for _ in 0..nb {
        let a = rng.gen_range(range_min..=range_max);
        let b = rng.gen_range(range_min..=range_max);
        let c = rng.gen_range(range_min..=range_max);
        if !associativity_64(a, b, c) {
            bad_cpt += 1;
        }
        bar.inc(1);
    }
    
    bar.finish();
    println!("Failure rate: {:.6}%\n", bad_cpt as f64 / nb as f64 * 100.0);
    writeln!(file, "Failure rate: {:.6}%\n", bad_cpt as f64 / nb as f64 * 100.0)
        .expect("Unable to write file");
}

fn main() {
    let args = Args::parse();
    
    // Initialize RNG with seed if provided, otherwise use random seed
    let mut rng = match args.seed {
        Some(seed) => {
            println!("Using seed: {seed}");
            StdRng::seed_from_u64(seed)
        }
        None => {
            println!("Using random seed");
            StdRng::from_entropy()
        }
    };
    
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&args.output)
        .expect("Unable to open file");
    
    // Run the test with provided parameters
    match args.float_type {
        32 => test_associativity_32(
            args.samples,
            args.range_min as f32,
            args.range_max as f32,
            &mut file,
            &mut rng,
        ),
        64 => test_associativity_64(
            args.samples,
            args.range_min,
            args.range_max,
            &mut file,
            &mut rng,
        ),
        _ => eprintln!("Unsupported float type: {}", args.float_type)
    }
    
}