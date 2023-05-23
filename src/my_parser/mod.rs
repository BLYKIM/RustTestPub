mod parse;
mod regular;
mod to_ts;

pub use parse::{file_to_vec, parsec};
pub use regular::regular;
use std::path::Path;
pub use to_ts::to_timestamp_nano;

//use regex::internal::Inst;
pub fn my_parser() {
    use std::time::Instant;

    let now = Instant::now();

    let path = Path::new("./src/log/access-training-1M.log");

    parse::parsec(path);

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
}
