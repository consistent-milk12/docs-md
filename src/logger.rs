//! Logging related utils.

use std::fs::File;
use std::path::PathBuf;

use clap::ValueEnum;
use miette::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::{self};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Log level for debug output
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum LogLevel {
    /// No logging output
    #[default]
    Off,

    /// Errors only
    Error,

    /// Errors and warnings
    Warn,

    /// Informational messages
    Info,

    /// Debug messages (verbose)
    Debug,

    /// Trace messages (very verbose)
    Trace,
}

impl LogLevel {
    /// Convert to tracing level filter
    #[must_use]
    pub const fn to_tracing_level(self) -> LevelFilter {
        match self {
            Self::Off => LevelFilter::OFF,

            Self::Error => LevelFilter::ERROR,

            Self::Warn => LevelFilter::WARN,

            Self::Info => LevelFilter::INFO,

            Self::Debug => LevelFilter::DEBUG,

            Self::Trace => LevelFilter::TRACE,
        }
    }
}

/// Logger
pub struct Logger;

impl Logger {
    /// Initialize logging
    ///
    /// # Errors
    /// Returns an error if fails to create a File.
    pub fn init_logging(log_level: LogLevel, log_file: Option<&PathBuf>) -> Result<()> {
        let level: LevelFilter = log_level.to_tracing_level();

        // Skip initialization if logging is disabled
        if level == LevelFilter::OFF {
            return Ok(());
        }

        // Build the subscriber with env filter support
        let subscriber = tracing_subscriber::registry().with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        );

        if let Some(log_path) = log_file {
            // Log to file
            let file: File = File::create(log_path)
                .map_err(|e| miette::miette!("Failed to create file: {e:?}"))?;
            let layer = fmt::layer()
                .with_writer(file)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true);

            subscriber.with(layer).init();
        } else {
            // Log to stderr (doesn't interfere with progress bars on stdout)
            let layer = fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true)
                .with_target(true)
                .compact();

            subscriber.with(layer).init();
        }

        tracing::info!("Logging initialized at level: {log_level:?}");

        Ok(())
    }
}
