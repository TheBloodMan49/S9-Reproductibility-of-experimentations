#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use rand::Rng as _;
use indicatif::ProgressBar;
use std::fs;
use std::io::Write;
use std::fs::File;

fn associativity_64(a: f64, b: f64, c: f64) -> bool {
    return (a + b) + c == a + (b + c)
}

fn associativity_32(a: f32, b: f32, c: f32) -> bool {
    return (a + b) + c == a + (b + c)
}

fn test_associativity_64(nb: usize, range_min: f64, range_max: f64, file: &mut File) {
    let mut bad_cpt = 0_usize;
    let mut rng = rand::thread_rng();
    let bar = ProgressBar::new(nb as u64);

    println!("Testing float64 from {range_min} to {range_max} with {nb} samples");
    writeln!(file, "Testing float64 from {range_min} to {range_max} with {nb} samples").expect("Unable to write file");

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
    writeln!(file, "Failure rate: {:.6}%\n", bad_cpt as f64 / nb as f64 * 100.0).expect("Unable to write file");
}

fn test_associativity_32(nb: usize, range_min: f32, range_max: f32, file: &mut File) {
    let mut bad_cpt = 0_usize;
    let mut rng = rand::thread_rng();
    let bar = ProgressBar::new(nb as u64);

    println!("Testing float32 from {range_min} to {range_max} with {nb} samples");
    writeln!(file, "Testing float32 from {range_min} to {range_max} with {nb} samples").expect("Unable to write file");

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

    println!("Failure rate: {:.6}%\n", bad_cpt as f32 / nb as f32 * 100.0);
    writeln!(file, "Failure rate: {:.6}%\n", bad_cpt as f32 / nb as f32 * 100.0).expect("Unable to write file");
}

fn main() {
    fs::write("./associativity_test.txt", "").expect("Unable to write file");
    let mut file = fs::OpenOptions::new().append(true).open("./associativity_test.txt").expect("Unable to open file");
    test_associativity_64(1_000_000, -1_000_000.0, 1_000_000.0, &mut file);
    test_associativity_64(1_000_000, -1_000.0, 1_000.0, &mut file);
    test_associativity_64(1_000_000, 0.0, 1_000_000.0, &mut file);
    test_associativity_64(1_000_000, 0.0, 1_000.0, &mut file);
    test_associativity_64(1_000_000, -1.0, 1.0, &mut file);
    test_associativity_64(1_000_000, 0.0, 1.0, &mut file);
    test_associativity_32(1_000_000, -1.0, 1.0, &mut file);
    test_associativity_32(1_000_000, 0.0, 1.0, &mut file);
}
