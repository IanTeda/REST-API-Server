use tracing::subscriber::set_global_default;
use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;

// Register tracing subscriber as global default to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    set_global_default(subscriber).expect("setting default subscriber failed");
}

// Compose multiple layers into a tracing subscriber.
pub fn get_subscriber() -> FmtSubscriber {
    FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish()
}
