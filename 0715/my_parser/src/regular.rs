use lazy_static::lazy_static;
use regex::Regex;

pub fn regular() {
    //regular expression
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
    (?P<ip>\S+)\s
    (?P<minus>\S*)\s
    (?P<uid>\S*)\s
    (?P<timedate>\[[^\]]+\])\s"
    (?P<method>[A-Z]*[^"]*)"\s
    (?P<code>[0-9]{3})\s
    (?P<size>[0-9]*)\s"
    (?P<url>[^"]*)"\s"
    (?P<info>[^"]*)"\n?"#,
        )
        .unwrap();
    }
}
