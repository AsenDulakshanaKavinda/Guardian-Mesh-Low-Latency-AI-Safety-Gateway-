
mod utils;


fn main() {
    let _guard = utils::logging::init_logging();
    tracing::info!("new log");
    tracing::debug!("debug log");
    tracing::warn!("warn log");
    tracing::error!("error log");
    tracing::trace!("trace log");
    println!("hello world!")
}

                


