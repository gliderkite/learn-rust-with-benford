use std::collections::HashMap;
use std::fs;

fn main() {
    // initialize the logger implementation
    env_logger::init();

    let dataset = "datasets/census.csv";
    log::info!("Reading dataset from {}", dataset);
    let file = fs::File::open(dataset).expect("Cannot read dataset");
    let mut reader = csv::Reader::from_reader(file);

    // the digits frequency map
    let mut frequency = HashMap::new();

    log::info!("Parsing CSV records");
    for record in reader.records() {
        // record is a Result that we must unwrap before parsing its content
        let record = record.expect("Invalid record");

        if let Some(digit) = get_first_digit(&record) {
            log::trace!("Found digit '{}' in {:?}", digit, record);

            // find the value corresponding to the digit key or insert a new
            // entry with an initial value of 0
            let count = frequency.entry(digit).or_insert(0);
            // count is a mutable reference to the value in the HashMap
            // increment its value by 1 after dereferencing it
            *count += 1;
        } else {
            log::warn!("No valid digit found in {:?}", record);
        }
    }
    log::debug!("Frequency: {:?}", frequency);
}

/// Parses the given record and extract the first valid digit from the population
/// value if any is found. Returns None otherwise.
fn get_first_digit(record: &csv::StringRecord) -> Option<char> {
    log::trace!("Parsing record: {:?}", record);
    record
        .get(1)
        .and_then(|population| population.chars().next())
        .filter(|c| c.is_ascii_digit() && *c != '0')
}
