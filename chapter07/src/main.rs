use std::collections::{BTreeMap, HashMap};
use std::{env, fs};

fn main() {
    env_logger::init();

    let dataset = match env::args().nth(1) {
        Some(path) => path,
        _ => panic!("The path of the census file must be specified"),
    };

    log::info!("Reading dataset from {}", dataset);
    let file = fs::File::open(dataset).expect("Cannot read dataset");
    let mut reader = csv::Reader::from_reader(file);

    let mut frequency = HashMap::new();
    for n in 1..=9 {
        let digit = std::char::from_digit(n, 10).expect("Invalid digit");
        frequency.insert(digit, 0);
    }

    log::info!("Parsing CSV records");
    for record in reader.records().filter_map(Result::ok) {
        if let Some(digit) = get_first_digit(&record) {
            log::trace!("Found digit '{}' in {:?}", digit, record);
            let count = frequency.get_mut(&digit).expect("Digit not found");
            *count += 1;
        } else {
            log::warn!("No valid digit found in {:?}", record);
        }
    }
    log::debug!("Frequency: {:?}", frequency);

    let total: usize = frequency.values().sum();
    let percentage: BTreeMap<char, f32> = frequency
        .into_iter()
        .map(|(digit, count)| (digit, count as f32 / total as f32))
        .collect();
    log::info!("Percentage: {:#.2?}", percentage);
}

fn get_first_digit(record: &csv::StringRecord) -> Option<char> {
    log::trace!("Parsing record: {:?}", record);
    record
        .get(1)
        .and_then(|population| population.chars().next())
        .filter(|c| c.is_ascii_digit() && *c != '0')
}
