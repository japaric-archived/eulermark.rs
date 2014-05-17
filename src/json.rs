extern crate serialize;

use serialize::Decodable;
use benchmark::Metric;
use file::{read,write};
use language::Language;
use serialize::json;

pub fn parse_language_file(language_file: &Path) -> Language {
    let mut decoder = match json::from_str(read(language_file)) {
        Err(_) => fail!("{} is invalid json", language_file.display()),
        Ok(json) => json::Decoder::new(json),
    };

    match Decodable::decode(&mut decoder) {
        Err(_) => fail!("couldn't decode {}", language_file.display()),
        Ok(language) => language,
    }
}

pub fn save_metrics(id: &str, metrics: &Vec<Metric>) {
    let metrics_file = Path::new("metrics").join(id).with_extension("json");

    write(&metrics_file, json::Encoder::str_encode(metrics).as_slice());
}
