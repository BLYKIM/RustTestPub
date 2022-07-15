use my_app::{roxyy::call_roxy, settings::trace_info};
use tracing::Level; // {error, info, Level};
use tracing_subscriber::FmtSubscriber;
fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let a: i32 = 2022;
    trace_info(&format!("{}", a));

    call_roxy();
}
