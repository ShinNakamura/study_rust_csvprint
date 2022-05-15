use std::error::Error;
use std::io;
use std::collections::HashMap;

type MyResult = Result<(), Box<dyn Error>>;
type Record = HashMap<String, String>;

pub fn run() -> MyResult {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // ヘッダーの順番を守る
    let header = rdr.headers()?.clone();
    for result in rdr.deserialize() {
        let record: Record = result?;
    }
    Ok(())
}
