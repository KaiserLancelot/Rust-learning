use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    fmt::init();

    let number_of_yaks: i32 = 3;
    info!(number_of_yaks, "preparing to shave yaks");
}
