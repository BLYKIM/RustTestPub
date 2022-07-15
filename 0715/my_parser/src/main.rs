use my_parser::parse;
use std::path::Path;
//use regex::internal::Inst;
fn main() {
    use std::time::Instant;

    let now = Instant::now();

    let path = Path::new("./src/log/access-training-1M.log");
    
    parse::parsec(path);
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
