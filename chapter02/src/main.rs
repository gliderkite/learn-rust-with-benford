use std::fs;

fn main() {
    let dataset = "datasets/census.csv";
    println!("Reading dataset from {}", dataset);
    let file = fs::File::open(dataset).expect("Cannot read dataset");
    let mut reader = csv::Reader::from_reader(file);

    println!("Parsing CSV records");
    for record in reader.records() {
        // record is a Result that we must unwrap before parsing its content
        let record = record.expect("Invalid record");
        println!("{:?}", record);
    }
}
