use ::logging::{logging, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = logging::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-logging operational");
    Ok(())
}
