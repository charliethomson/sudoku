use std::time::SystemTime;

mod paths {
    use std::path::PathBuf;

    use anyhow::bail;
    use chrono::Utc;
    use consts::PRODUCT_NAME;

    pub fn local_dir() -> anyhow::Result<PathBuf> {
        let Some(dir) = dirs::config_local_dir() else {
            bail!("Failed to locate config local dir");
        };
        return Ok(dir);
    }

    pub fn config_dir() -> anyhow::Result<PathBuf> {
        Ok(local_dir()?.join(PRODUCT_NAME))
    }
    pub fn logs_base_dir() -> anyhow::Result<PathBuf> {
        let config_dir = config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        Ok(config_dir.join("logs"))
    }

    pub fn logs_dir(module_name: &str) -> anyhow::Result<PathBuf> {
        let logs_base_dir = logs_base_dir()?;
        if !logs_base_dir.exists() {
            std::fs::create_dir_all(&logs_base_dir)?;
        }
        let this = logs_base_dir.join(module_name);
        if !this.exists() {
            std::fs::create_dir_all(&this)?;
        }
        Ok(this)
    }
    pub fn shared_log_file() -> anyhow::Result<PathBuf> {
        let slug = Utc::now().format("shared-%Y%m%d.log").to_string();
        Ok(logs_base_dir()?.join(slug))
    }
    pub fn log_file(module_name: &str) -> anyhow::Result<PathBuf> {
        let slug = Utc::now()
            .format(&format!("{module_name}-%Y%m%d.log"))
            .to_string();
        Ok(logs_dir(module_name)?.join(slug))
    }
}

pub fn setup_logger(module_name: &'static str) -> anyhow::Result<()> {
    let module = module_name.to_string();

    let shared_log_file = paths::shared_log_file()?;
    let log_file = paths::log_file(module_name)?;


    if !shared_log_file.exists() {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&shared_log_file)?;
    }
    if !log_file.exists() {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&log_file)?;
    }

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{} {} {}] {}",
                module,
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(shared_log_file)?)
        .chain(fern::log_file(log_file)?)
        .apply()?;

    log::info!("Logger configured for module: {}", module_name);
    Ok(())
}
