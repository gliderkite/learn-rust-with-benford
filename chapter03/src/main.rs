use std::fs;

fn main() {
    // initialize the logger implementation
    env_logger::init();

    let dataset = "datasets/census.csv";
    log::info!("Reading dataset from {}", dataset);
    let file = fs::File::open(dataset).expect("Cannot read dataset");
    let mut reader = csv::Reader::from_reader(file);

    log::info!("Parsing CSV records");
    for record in reader.records() {
        // record is a Result that we must unwrap before parsing its content
        let record = record.expect("Invalid record");
        log::trace!("{:?}", record);
    }
}
