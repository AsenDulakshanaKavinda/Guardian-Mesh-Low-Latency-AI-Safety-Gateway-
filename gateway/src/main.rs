
mod utils;


fn main() {
    let _guard = utils::logging::init_logging();
    tracing::info!("new log");
    println!("hello world!")
}

                


