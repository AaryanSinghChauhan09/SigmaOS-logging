use anyhow::Result;
use clap::Parser;
use logging::logging;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = logging::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = logging::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-logging running");
    Ok(())
}
