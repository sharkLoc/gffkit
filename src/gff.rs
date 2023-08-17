use noodles_gff as gff;
use gff::record::attributes::field::Value;
use anyhow::Error;
use std::io::BufReader;
use std::fs::File;


pub fn feature_select(
    gff: &str,
    types: Option<String>,
    key: &str,
    name: &str
) -> Result<(),Error>{
    let mut reader = File::open(gff)
        .map(BufReader::new)
        .map(gff::Reader::new)?;
    let key_wrap = Value::from(name);

    for record in reader.records().flatten() {
        let rec = record.attributes();
        if let Some(ref types) = types {
            if types == record.ty() {
                if let Some(key) = rec.get(key) {
                    if key.eq(&key_wrap) {
                        println!("{}\t{}\t{}\t{}\t{}\t{}",
                            record.reference_sequence_name(),
                            record.ty(),
                            record.start(),
                            record.end(),
                            record.strand(),
                            key
                        );
                    }
                }
            }
        } else {
            if let Some(key) = rec.get(key) {
                if key.eq(&key_wrap) {
                    println!("{}\t{}\t{}\t{}\t{}\t{}",
                        record.reference_sequence_name(),
                        record.ty(),
                        record.start(),
                        record.end(),
                        record.strand(),
                        key
                    );
                }
            }
        }
    }
       
    Ok(())
}
