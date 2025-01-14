use anyhow::Result;

pub fn init() -> Result<()> {
    let debug_env: bool = std::env::var("DEBUG").is_ok();
    env_logger::builder()
        .filter_level(if debug_env {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();
    println!(
        "{} - v{:<8} - Testing GeekORM\n",
        geekorm::GEEKORM_BANNER,
        geekorm::GEEKORM_VERSION
    );

    Ok(())
}
