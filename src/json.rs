extern crate serialize;

use benchmark::Metric;
use file::{read,write};
use hash::Hashes;
use language::Language;
use serialize::Decodable;
use serialize::json;
use std::collections::HashMap;

pub fn parse_language_file(language_file: &Path) -> Language {
    let mut decoder = match json::from_str(read(language_file).as_slice()) {
        Err(_) => fail!("{} is invalid json", language_file.display()),
        Ok(json) => json::Decoder::new(json),
    };

    match Decodable::decode(&mut decoder) {
        Err(_) => fail!("couldn't decode {}", language_file.display()),
        Ok(language) => language,
    }
}

pub fn load_hashes(id: &str) -> Hashes {
    let file = Path::new(format!("hashes/{}.json", id));

    if file.exists() {
        match json::from_str(read(&file).as_slice()) {
            Err(_) => HashMap::new(),
            Ok(json) => {
                match Decodable::decode(&mut json::Decoder::new(json)) {
                    Err(_) => HashMap::new(),
                    Ok(map) => map,
                }
            }
        }
    } else {
        HashMap::new()
    }
}

pub fn load_metrics(id: &str) -> Vec<Metric> {
    let file = Path::new(format!("metrics/{}.json", id));

    if file.exists() {
        match json::from_str(read(&file).as_slice()) {
            Err(_) => Vec::new(),
            Ok(json) => {
                match Decodable::decode(&mut json::Decoder::new(json)) {
                    Err(_) => Vec::new(),
                    Ok(map) => map,
                }
            }
        }
    } else {
        Vec::new()
    }
}

pub fn save_hashes(id: &str, hashes: &Hashes) {
    let hashes_file = Path::new(format!("hashes/{}.json", id));

    write(&hashes_file, json::Encoder::str_encode(hashes).as_slice());
}

pub fn update_metrics(id: &str, mut new_metrics: Vec<Metric>) {
    let metrics_file = Path::new(format!("metrics/{}.json", id));
    let mut old_metrics = load_metrics(id);
    let mut metrics = Vec::new();

    old_metrics.sort_by(|a, b| {
        a.language().cmp(&b.language())
    });
    new_metrics.sort_by(|a, b| {
        a.language().cmp(&b.language())
    });

    loop {
        match (old_metrics.pop(), new_metrics.pop()) {
            (None, None) => break,
            (Some(old), None) => {
                metrics.push(old);
            },
            (None, Some(new)) => {
                metrics.push(new);
            },
            (Some(old), Some(new)) => {
                match old.language().cmp(&new.language()) {
                    Less => {
                        metrics.push(new);
                        old_metrics.push(old);
                    },
                    Equal => {
                        let diff = (new.median() / old.median() - 1.0) * 100.0;

                        if new.lower() > old.upper() {
                            println!("* {} regressed by {:.2}%",
                                     new.language(),
                                     diff)
                        } else if new.upper() < old.lower() {
                            println!("* {} improved by {:.2}%",
                                     new.language(),
                                     -diff)
                        }

                        metrics.push(new);
                    },
                    Greater => {
                        metrics.push(old);
                        new_metrics.push(new);
                    },
                }
            }
        }
    }

    metrics.sort_by(|a, b| {
        ((1000.0 * a.median()) as u64).cmp(&((1000.0 * b.median()) as u64))
    });

    write(&metrics_file, json::Encoder::str_encode(&metrics).as_slice());
}
