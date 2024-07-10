use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

fn main() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stdout);

    tracing_subscriber::Registry::default()
        .with(stdout_log)
        .init();

    info!("Logger initialized!");

    let pos = Position {
        x: 3.234,
        y: -1.223,
    };

    debug!(?pos.x, ?pos.y);
    debug!(%pos.x, %pos.y);
    debug!(target: "app_events", position = ?pos, "New position");
    debug!(name: "completed", position = ?pos);
}
