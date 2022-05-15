use std::error::Error;
use std::io;
use std::io::Write;
use std::collections::HashMap;

type MyResult = Result<(), Box<dyn Error>>;
type Record = HashMap<String, String>;

pub fn run() -> MyResult {
    let input = io::BufReader::new(io::stdin());
    let mut rdr = csv::Reader::from_reader(input);
    let mut wtr = io::BufWriter::new(io::stdout());
    // ヘッダーの順番を守る
    let header = rdr.headers()?.clone();
    let mut rec_n = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        rec_n += 1;
        writeln!(wtr, "<!-- RECORD BEGIN {} -->", rec_n)?;
        for h in header.iter() {
            writeln!(wtr, "<!-- COLUMN BEGIN {} -->", h)?;
            match record.get(h) {
                Some(val) => writeln!(wtr, "{}", val)?,
                None => (),
            }
            writeln!(wtr, "<!-- COLUMN END   {} -->", h)?;
        }
        writeln!(wtr, "<!-- RECORD END   {} -->", rec_n)?;
    }
    Ok(())
}
