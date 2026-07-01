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
    ToyBank,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::ToyBank => {
            let energy = bil_math_examples::toy_bank::run_toy_bank_sheaf_residual();
            println!("toy_bank_sheaf_residual_energy={energy}");
        }
    }
}