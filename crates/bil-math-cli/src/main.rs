use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "bil-math")]
#[command(about = "Pure Rust mathematical examples for BIL")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    HolonomyLoop,
    ToyBank,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::HolonomyLoop => {
            let report = bil_math_examples::holonomy_obstruction::run_holonomy_loop_obstruction();
            println!("holonomy_identity_deviation={}", report.identity_deviation);
            println!("holonomy_trace={}", report.holonomy_trace);
            println!("holonomy_tolerance={}", report.tolerance);
            println!("holonomy_is_trivial={}", report.is_trivial);
        }
        Command::ToyBank => {
            let energy = bil_math_examples::toy_bank::run_toy_bank_sheaf_residual();
            println!("toy_bank_sheaf_residual_energy={energy}");
        }
    }
}
