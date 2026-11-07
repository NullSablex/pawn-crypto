use samp::log::LevelFilter;
use samp::logger::LoggerConfig;

pub struct Logger;

impl Logger {
    pub fn init() {
        if let Err(error) = samp::enable_logger_with!(
            LoggerConfig::new(env!("CARGO_PKG_NAME"))
                .directory("logs")
                .filename("crypto.log")
                .prefix("[Crypto]")
                .level(LevelFilter::Info)
                .compress_archives(true)
                .banner_with(|meta| {
                    vec![
                        format!("{} {}", meta.name, meta.version),
                        format!("Author and maintainer: {}", meta.authors),
                        format!("Compiled: {} at {}", env!("BUILD_DATE"), env!("BUILD_TIME")),
                        format!("Repository: {}", meta.repository),
                    ]
                })
                .from_env()
        ) {
            eprintln!("[Crypto] Failed to install the logger: {error}");
        }
    }

    pub fn flush() {
        samp::logger::flush();
    }
}
