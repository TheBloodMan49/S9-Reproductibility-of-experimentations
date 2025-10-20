use core::f64::consts::E;
use std::fs;
use std::io::Write;
use std::path::Path;
use clap::Parser;
use num_bigfloat::BigFloat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Calculation method: "iterative" or "logarithmic"
    #[arg(short, long, default_value = "logarithmic")]
    method: String,

    /// Also test with e approximated to specified decimal places
    #[arg(short = 'd', long)]
    approximate_decimals: Option<usize>,

    /// Number of years (n)
    #[arg(short = 'y', long, default_value_t = 50)]
    years: usize,

    /// Output file path
    #[arg(short, long, default_value = "./investment_test.txt")]
    output: String,

    /// Output in CSV format
    #[arg(short = 'c', long)]
    csv: bool,

    /// Use high-precision BigFloat arithmetic
    #[arg(long)]
    big: bool,
}

fn iterative_method(initial: f64, years: usize) -> f64 {
    let mut investment = initial;
    for year in 1..=years {
        investment = (investment - 1.0) * f64::from(year as i32);
    }
    investment - 1.0
}

/// Logarithmic method using f64 (stable)
fn logarithmic_method(initial: f64, years: usize) -> f64 {
    if years == 0 {
        return initial - 1.0;
    }

    let mut log_fact_n = 0.0_f64;
    for k in 1..=years {
        log_fact_n += (k as f64).ln();
    }

    let mut log_fact_k = 0.0_f64;
    let mut sum_scaled = 1.0_f64; // include n!
    for k in (0..years).rev() {
        if k > 0 {
            log_fact_k += (k as f64).ln();
        }
        let log_ratio = log_fact_k - log_fact_n;
        sum_scaled += log_ratio.exp();
    }

    let result = (initial - sum_scaled) * log_fact_n.exp();
    result
}

/// Logarithmic method using BigFloat (arbitrary precision)
fn logarithmic_method_big(initial: f64, years: usize) -> BigFloat {

    let mut log_fact_n = BigFloat::from_f64(0.0);
    for k in 1..=years {
        let ln_k = BigFloat::from_f64(k as f64).ln();
        log_fact_n += ln_k;
    }

    let mut log_fact_k = BigFloat::from_f64(0.0);
    let mut sum_scaled = BigFloat::from_f64(1.0);
    for k in (0..years).rev() {
        if k > 0 {
            log_fact_k += BigFloat::from_f64(k as f64).ln();
        }
        let log_ratio = log_fact_k - log_fact_n;
        sum_scaled += log_ratio.exp();
    }

    let initial_bf = BigFloat::from_f64(initial);
    let log_fact_n_exp = log_fact_n.exp();

    // R = n! * (e - Σ(k!)/n!)
    (initial_bf - sum_scaled) * log_fact_n_exp
}

fn log_to_stdout_and_file(file: &mut fs::File, message: &str) {
    print!("{}", message);
    write!(file, "{}", message).expect("Unable to write to file");
}

fn write_csv_headers(file: &mut fs::File) {
    writeln!(file, "method,description,initial_value,years,result").expect("Unable to write to file");
}

fn main() {
    let args = Args::parse();

    // Validate method
    let method = args.method.to_lowercase();
    if method != "iterative" && method != "logarithmic" {
        eprintln!("Error: Method must be either 'iterative' or 'logarithmic'");
        std::process::exit(1);
    }

    // Adjust output path based on CSV flag
    let mut output_path = args.output.clone();
    if args.csv && !output_path.ends_with(".csv") {
        if let Some(pos) = output_path.rfind('.') {
            output_path.replace_range(pos.., ".csv");
        } else {
            output_path.push_str(".csv");
        }
    }

    let file_exists = Path::new(&output_path).exists();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("Unable to open or create file");
    if args.csv && !file_exists {
        write_csv_headers(&mut file);
    }

    let (description, initial) = if let Some(decimals) = args.approximate_decimals {
        let factor = 10_f64.powi(decimals as i32);
        let approximated = (E * factor).round() / factor;
        (format!("e approximated to {} decimals", decimals), approximated)
    } else {
        ("e from core::f64::consts::E".to_string(), E)
    };

    let result_str = match method.as_str() {
        "iterative" => iterative_method(initial, args.years).to_string(),
        "logarithmic" => {
            if args.big {
                logarithmic_method_big(initial, args.years).to_string()
            } else {
                format!("{:.10}", logarithmic_method(initial, args.years))
            }
        }
        _ => unreachable!(),
    };

    if args.csv {
        let csv_line = format!("{},{},{},{},{}\n",
            method, description, initial, args.years, result_str);
        log_to_stdout_and_file(&mut file, &csv_line);
    } else {
        log_to_stdout_and_file(&mut file, &format!("Using method: {}\n", method));
        log_to_stdout_and_file(&mut file, &format!("Testing for {} years\n\n", args.years));
        log_to_stdout_and_file(&mut file, &format!("With {}:\n", description));
        log_to_stdout_and_file(&mut file, &format!("Initial value: {}\n", initial));
        log_to_stdout_and_file(&mut file, &format!("After {} years you can retrieve: {}\n", args.years, result_str));
    }

    println!("Results written to {}", output_path);
}
