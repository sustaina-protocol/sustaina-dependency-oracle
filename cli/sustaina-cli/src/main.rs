use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

mod parser;
mod resolver;
mod errors;

use errors::CliError;

#[derive(Parser)]
#[command(name = "sustaina")]
#[command(about = "Sustaina Protocol - Dependency Analyzer and Split Generator")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to Soroban registry contract RPC endpoint
    #[arg(
        global = true,
        long,
        default_value = "https://soroban-testnet.stellar.org"
    )]
    registry_rpc: String,

    /// Enable debug logging
    #[arg(global = true, short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Cargo.toml and query registry for dependency addresses
    Fund {
        /// Path to Cargo.toml manifest
        #[arg(short, long, default_value = "Cargo.toml")]
        manifest: PathBuf,

        /// Percentage of stream to split to dependencies (1-100)
        #[arg(short, long, default_value = "10")]
        split_percentage: u32,

        /// Output format
        #[arg(short, long, default_value = "cli", value_parser = ["cli", "json"])]
        format: String,
    },

    /// Show dependency graph and funding status
    Status {
        /// Path to Cargo.toml manifest
        #[arg(short, long, default_value = "Cargo.toml")]
        manifest: PathBuf,
    },

    /// Generate XDR for Sustaina Split contract deployment
    Deploy {
        /// Path to Cargo.toml manifest
        #[arg(short, long, default_value = "Cargo.toml")]
        manifest: PathBuf,

        /// Percentage of stream to split to dependencies
        #[arg(short, long, default_value = "10")]
        split_percentage: u32,

        /// Your Stellar address (sender)
        #[arg(short, long)]
        from: String,

        /// Factory contract ID for Split deployment
        #[arg(short, long)]
        factory_id: String,

        /// Output file for XDR command (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    match cli.command {
        Commands::Fund {
            manifest,
            split_percentage,
            format,
        } => {
            handle_fund(&manifest, split_percentage, &format, &cli.registry_rpc).await?
        }

        Commands::Status { manifest } => handle_status(&manifest, &cli.registry_rpc).await?,

        Commands::Deploy {
            manifest,
            split_percentage,
            from,
            factory_id,
            output,
        } => {
            handle_deploy(
                &manifest,
                split_percentage,
                &from,
                &factory_id,
                output,
                &cli.registry_rpc,
            )
            .await?
        }
    }

    Ok(())
}

async fn handle_fund(
    manifest: &PathBuf,
    split_percentage: u32,
    format: &str,
    registry_rpc: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if split_percentage == 0 || split_percentage > 100 {
        return Err(Box::new(CliError::InvalidPercentage(split_percentage)));
    }

    log::info!("🔍 Analyzing {}...", manifest.display());

    let deps = parser::extract_dependencies(manifest)?;
    log::info!("📦 Found {} dependencies", deps.len());

    if deps.is_empty() {
        println!("No dependencies found in {}.", manifest.display());
        return Ok(());
    }

    let resolved = resolver::resolve_addresses(&deps, registry_rpc).await?;

    let registered: Vec<_> = resolved.iter().filter(|r| r.is_registered).collect();

    match format {
        "json" => {
            let output = serde_json::json!({
                "total_dependencies": deps.len(),
                "registered_dependencies": registered.len(),
                "split_percentage": split_percentage,
                "dependencies": resolved,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!("\n📊 Dependency Analysis Results");
            println!("├─ Total dependencies: {}", deps.len());
            println!("├─ Registered on-chain: {}", registered.len());
            println!("├─ Unregistered: {}", deps.len() - registered.len());
            println!("└─ Split percentage: {}%\n", split_percentage);

            for dep in &resolved {
                if dep.is_registered {
                    println!("  ✅ {} -> {}", dep.name, dep.address);
                } else {
                    println!("  ❌ {} (Not registered)", dep.name);
                }
            }

            if !registered.is_empty() {
                let bps_per_dep = (split_percentage * 100) / registered.len() as u32;
                println!(
                    "\n⚙️  BPS per registered dependency: {}",
                    bps_per_dep
                );
            }
        }
    }

    Ok(())
}

async fn handle_status(
    manifest: &PathBuf,
    registry_rpc: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("📋 Checking status of {}...", manifest.display());

    let deps = parser::extract_dependencies(manifest)?;
    let resolved = resolver::resolve_addresses(&deps, registry_rpc).await?;

    let registered = resolved.iter().filter(|r| r.is_registered).count();

    println!("\n📊 Dependency Status");
    println!("├─ Total: {}", deps.len());
    println!("├─ Registered: {}", registered);
    println!("└─ Unregistered: {}\n", deps.len() - registered);

    println!("Details:");
    for dep in resolved {
        if dep.is_registered {
            println!("  ✅ {:<40} → {}", dep.name, dep.address);
        } else {
            println!("  ❌ {:<40} (pending registration)", dep.name);
        }
    }

    Ok(())
}

async fn handle_deploy(
    manifest: &PathBuf,
    split_percentage: u32,
    from: &str,
    factory_id: &str,
    output: Option<PathBuf>,
    registry_rpc: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if split_percentage == 0 || split_percentage > 100 {
        return Err(Box::new(CliError::InvalidPercentage(split_percentage)));
    }

    log::info!("🚀 Generating Split deployment XDR...");

    let deps = parser::extract_dependencies(manifest)?;
    let resolved = resolver::resolve_addresses(&deps, registry_rpc).await?;

    let registered: Vec<_> = resolved.iter().filter(|r| r.is_registered).collect();

    if registered.is_empty() {
        return Err(Box::new(CliError::NoRegisteredDependencies));
    }

    let bps_per_dep = (split_percentage * 100) / registered.len() as u32;

    let mut xdr_command = format!(
        "soroban contract invoke \\\n  --id {} \\\n  --source-account {} \\\n  -- deploy_split",
        factory_id, from
    );

    for dep in &registered {
        xdr_command.push_str(&format!(
            " \\\n  --recipient {} --bps {}",
            dep.address, bps_per_dep
        ));
    }

    if let Some(output_path) = output {
        fs::write(&output_path, &xdr_command)?;
        log::info!("✅ XDR command written to {}", output_path.display());
    }

    println!("\n🎯 Sustaina Split Deployment Command:\n");
    println!("{}\n", xdr_command);
    println!("This command will route {}% of your stream to {} dependencies.", split_percentage, registered.len());

    Ok(())
}
