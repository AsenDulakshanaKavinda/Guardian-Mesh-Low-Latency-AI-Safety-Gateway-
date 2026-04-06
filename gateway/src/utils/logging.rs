use tracing_appender::non_blocking::{WorkerGuard};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initializes logging with both console and file outputs. The console output is human-readable, 
/// while the file output is in JSON format for easier parsing. The log files are rotated weekly and stored
/// in the "logs" directory with the name "app.log". The function returns a `WorkerGuard` to ensure that
/// all log messages are flushed before the application exits.
/// 
/// # Returns
/// A `WorkerGuard` that should be kept alive for the duration of the application to ensure that all log messages are properly flushed to the file.
/// 
/// # Example
/// ```
/// let _guard = init_logging();
/// tracing::info!("This is a log message.");
/// ```     
/// 
/// # Notes
/// - The logging level can be configured using the `RUST_LOG` environment variable. For example, setting `RUST_LOG=debug` will enable debug-level logging.
/// - The console output does not include the target field to reduce clutter, while the file output includes all log information in JSON format for better analysis and monitoring.

pub fn init_logging() -> WorkerGuard {
    let file_appender = rolling::weekly("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer()
        .with_target(false);

    let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .json()
            .with_ansi(false);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}

