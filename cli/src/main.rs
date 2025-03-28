use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod commands;

/// Solana developer tooling that doesn't suck
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy a Solana program
    Deploy {
        /// Path to the program .so file
        #[arg(value_name = "PROGRAM_FILE")]
        program_file: Option<String>,
        
        /// Network to deploy to (localnet, devnet, testnet, mainnet)
        #[arg(short, long, default_value = "localnet")]
        network: String,
        
        /// Path to keypair file
        #[arg(short, long)]
        keypair: Option<String>,
    },
    
    /// Monitor transactions for a Solana program
    Monitor {
        /// Program ID to monitor
        #[arg(value_name = "PROGRAM_ID")]
        program_id: String,
        
        /// Network to monitor (localnet, devnet, testnet, mainnet)
        #[arg(short, long, default_value = "localnet")]
        network: String,
        
        /// Filter transactions by account
        #[arg(short, long)]
        account: Option<String>,
    },
    
    /// Run tests using LiteSVM
    Test {
        /// Path to test file or directory
        #[arg(value_name = "PATH")]
        path: Option<String>,
        
        /// Run tests in watch mode
        #[arg(short, long)]
        watch: bool,
    },
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Parse command line args
    let cli = Cli::parse();
    
    // Execute command
    match &cli.command {
        Commands::Deploy { program_file, network, keypair } => {
            println!("Would deploy program: {:?} to network: {}", program_file, network);
            // Here you would call commands::deploy::execute(program_file, network, keypair)
            Ok(())
        }
        
        Commands::Monitor { program_id, network, account } => {
            println!("Would monitor program ID: {} on network: {}", program_id, network);
            if let Some(acct) = account {
                println!("Filtering by account: {}", acct);
            }
            // Here you would call commands::monitor::execute(program_id, network, account)
            Ok(())
        }
        
        Commands::Test { path, watch } => {
            println!("Would run tests at path: {:?}", path);
            if *watch {
                println!("In watch mode");
            }
            // Here you would call commands::test::execute(path, watch)
            Ok(())
        }
    }
}