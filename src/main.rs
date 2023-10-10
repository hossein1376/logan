use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_log_files(path: &str) -> Result<HashMap<String, HashMap<String, String>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data: HashMap<String, HashMap<String, String>> = HashMap::new();

    for line in reader.lines() {
        let mut entry: HashMap<String, String> = serde_json::from_str(line?.as_str())?;

        let time = entry.remove("time").unwrap();
        data.insert(time, entry);
    }

    Ok(data)
}


fn main() {
    let res = process_log_files("./file.log").unwrap();
    println!("{:#?}", res)
}
