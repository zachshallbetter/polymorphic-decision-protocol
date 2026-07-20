use std::env;
use std::fs;
use std::io::{self, Read};
use pdp_core::{
    math::{acs, score_model, flip_distance, committed_fraction, allocate_ladder, matrix_cell},
    AgentVerdict, ScoreDimension, LegSpec, cents::Cents, SizingBand
};

fn read_input(arg: Option<String>) -> io::Result<String> {
    match arg {
        Some(ref file_path) if file_path != "-" => fs::read_to_string(file_path),
        _ => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = args[1].as_str();
    match command {
        "acs" => {
            let input = read_input(args.get(2).cloned()).expect("Failed to read input");
            let verdicts: Vec<AgentVerdict> = serde_json::from_str(&input).expect("Invalid AgentVerdict JSON array");
            let result = acs(&verdicts);
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        "fd" => {
            let input = read_input(args.get(2).cloned()).expect("Failed to read input");
            let dimensions: Vec<ScoreDimension> = serde_json::from_str(&input).expect("Invalid ScoreDimension JSON array");
            let scored = score_model(&dimensions);
            if let Some(fd) = flip_distance(&scored) {
                println!(
                    "{{\n  \"distance\": {:.6},\n  \"sensitive_dimension\": {}\n}}",
                    fd.distance,
                    fd.sensitive_dimension.map_or("null".to_string(), |s| format!("\"{}\"", s))
                );
            } else {
                println!("{{\n  \"distance\": null,\n  \"sensitive_dimension\": null\n}}");
            }
        }
        "sizing" => {
            if args.len() < 8 {
                eprintln!("Usage: pdp-cli sizing <p> <b> <k> <sizing_band> <ruin_cap_dollars> <bankroll_dollars>");
                std::process::exit(1);
            }
            let p: f64 = args[2].parse().expect("Invalid p float");
            let b: f64 = args[3].parse().expect("Invalid b float");
            let k: f64 = args[4].parse().expect("Invalid k float");
            let band: SizingBand = match args[5].as_str() {
                "Full" | "full" => SizingBand::Full,
                "Half" | "half" => SizingBand::Half,
                "Quarter" | "quarter" => SizingBand::Quarter,
                "Kill" | "kill" => SizingBand::Kill,
                _ => panic!("Invalid SizingBand. Options: Full, Half, Quarter, Kill"),
            };
            let ruin_cap = Cents::from_dollars(args[6].parse().expect("Invalid ruin_cap float"));
            let bankroll = Cents::from_dollars(args[7].parse().expect("Invalid bankroll float"));

            let size = committed_fraction(p, b, k, band, ruin_cap, bankroll);
            println!("{{\n  \"committed_cents\": {},\n  \"committed_dollars\": {:.2}\n}}", size.0, size.to_dollars());
        }
        "matrix" => {
            if args.len() < 4 {
                eprintln!("Usage: pdp-cli matrix <acs_score> <flip_distance>");
                std::process::exit(1);
            }
            let acs_score: f64 = args[2].parse().expect("Invalid acs_score float");
            let fd_val: f64 = args[3].parse().expect("Invalid flip_distance float");
            let band = matrix_cell(acs_score, fd_val);
            println!("{{\n  \"sizing_band\": \"{:?}\"\n}}", band);
        }
        "allocate" => {
            if args.len() < 3 {
                eprintln!("Usage: pdp-cli allocate <total_dollars> [legs_json_file]");
                std::process::exit(1);
            }
            let total = Cents::from_dollars(args[2].parse().expect("Invalid total_dollars float"));
            let input = read_input(args.get(3).cloned()).expect("Failed to read input");
            let legs: Vec<LegSpec> = serde_json::from_str(&input).expect("Invalid LegSpec JSON array");
            match allocate_ladder(total, &legs) {
                Ok(ladder) => println!("{}", serde_json::to_string_pretty(&ladder).unwrap()),
                Err(err) => {
                    eprintln!("Allocation Error: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!(
        "PDP CLI Utility\n\n\
        Usage:\n  \
          pdp-cli acs [verdicts_json_file]      - Computes Consensus Consensus Score (ACS)\n  \
          pdp-cli fd [dimensions_json_file]     - Computes Scored Model and Flip Distance\n  \
          pdp-cli matrix <acs> <flip_distance>   - Looks up the decision SizingBand\n  \
          pdp-cli sizing <p> <b> <k> <band> <ruin> <bankroll> - Computes committed fraction\n  \
          pdp-cli allocate <total> [legs_json]  - Performs integer-cent rounding ladder allocation\n\n\
        Use '-' or omit the JSON file argument to read from stdin."
    );
}
