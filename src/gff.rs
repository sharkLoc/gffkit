use bio::io::gff;
use anyhow::Error;

pub fn feature_select(
    gff: &str,
    types: Option<String>,
    key: &str,
    name: &str
) -> Result<(),Error>{
    let mut reader = gff::Reader::from_file(gff, gff::GffType::GFF3)?;
    for record in reader.records().flatten() {
        let rec = record.attributes();
        if let Some(ref types) = types {
            if types == record.feature_type() {
                if let Some(key) = rec.get(key) {
                    if key == name {
                        println!("{}\t{}\t{}\t{}\t{}\t{}",record.seqname(), record.feature_type(),record.start(),record.end(),record.strand().unwrap(),key);
                    }
                }
            }
        } else {
            if let Some(key) = rec.get(key) {
                if key == name {
                    println!("{}\t{}\t{}\t{}\t{}\t{}",record.seqname(), record.feature_type(),record.start(),record.end(),record.strand().unwrap(),key);
                }
            }
        }
    }
       
    Ok(())
}