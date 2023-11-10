use crate::common::cli::Arguments;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

pub fn initialise(args: &Arguments) {
    let level = match args.verbose.log_level().get_or_insert(log::Level::Info) {
        log::Level::Error => Level::ERROR,
        log::Level::Warn => Level::WARN,
        log::Level::Info => Level::INFO,
        log::Level::Debug => Level::DEBUG,
        log::Level::Trace => Level::TRACE,
    };

    if args.debug {
        console_subscriber::init();
    } else {
        let subscriber = FmtSubscriber::builder()
            .json()
            .with_max_level(level)
            .with_thread_ids(true)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
    debug!("Tracing initialised.");
}
