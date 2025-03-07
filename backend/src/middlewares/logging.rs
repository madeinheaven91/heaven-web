use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error
};
use log::{LevelFilter, debug, warn};
use std::io::Write;
use std::{env, fs};

use chrono::Local;

use crate::shared::statics::CONFIG;

struct Logger {
    path: String,
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        fs::exists(format!("logs/{}.log", self.path)).unwrap_or(false)
            && CONFIG.log_level != LevelFilter::Off
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // Print to console
            let level = match record.level() {
                log::Level::Error => "\x1b[31mERROR\x1b[0m",
                log::Level::Warn => "\x1b[93mWARN\x1b[0m",
                log::Level::Info => "\x1b[34mINFO\x1b[0m",
                log::Level::Debug => "\x1b[37mDEBUG\x1b[0m",
                log::Level::Trace => "\x1b[35mTRACE\x1b[0m",
            };
            let entry = format!(
                "{} | {} | {}",
                level,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            );

            println!("{entry}");

            // Write to file
            let level = match record.level() {
                log::Level::Error => "ERROR",
                log::Level::Warn => "WARN",
                log::Level::Info => "INFO",
                log::Level::Debug => "DEBUG",
                log::Level::Trace => "TRACE",
            };
            let entry = format!(
                "{} | {} | {}",
                level,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            );

            let file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("logs/{}.log", self.path));
            if let Ok(mut file) = file {
                let _ = writeln!(file, "{}", entry);
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logging() {
    let log_dir = fs::create_dir("logs");
    match log_dir {
        Ok(_) => {
            debug!("Log directory created")
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => warn!("Couldn't create log directory: {}", err),
        },
    }

    let log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("logs/{}.log", CONFIG.log_file.clone()));
    match log_file {
        Ok(_) => debug!("Log file created"),
        Err(err) => warn!("Couldn't create log file: {}", err),
    }

    let logger = Box::leak(Box::new(Logger {
        path: CONFIG.log_file.clone(),
    }));

    log::set_logger(logger).unwrap();
    log::set_max_level(get_log_level());
}

fn get_log_level() -> LevelFilter {
    match env::var("RUST_LOG").as_deref() {
        Ok("error") => LevelFilter::Error,
        Ok("warn") => LevelFilter::Warn,
        Ok("info") => LevelFilter::Info,
        Ok("debug") => LevelFilter::Debug,
        Ok("trace") => LevelFilter::Trace,
        _ => LevelFilter::Info, // Default level
    }
}

#[allow(dead_code)]
pub async fn log_requests_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    
    debug!("{} {} {:?}", req.head().method, req.head().uri, req.head().version);
    debug!(
        "{}",
        req.headers()
            .iter()
            .fold(
                String::new(),
                |acc, (k,v)| format!("{}{}: {}\n", acc, k, v.to_str().unwrap())
            )
    );
    next.call(req).await
}
