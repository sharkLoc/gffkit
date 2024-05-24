use crate::{
    error::GffError,
    utils::{file_reader, file_writer},
};
use log::{info, trace};
use noodles::{
    core::{region::Interval, Region},
    fasta::{self, fai, indexed_reader, record, Record},
    gff::{self, record::attributes::field::Value},
};
use std::error::Error;
use std::time::Instant;

#[allow(clippy::too_many_arguments)]
pub fn extract_seq(
    fasta: &str,
    index: bool,
    gff: &str,
    types: Option<String>,
    key: &str,
    name: &str,
    out: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // index fasta file
    if index {
        info!("create index file for: {}", fasta);
        let fai_name = format!("{}.fai", fasta);
        let fai = file_writer(Some(&fai_name), 0u32)?;

        let faidx = fasta::index(fasta)?;
        let mut fai_writer = fai::Writer::new(fai);
        fai_writer.write_index(&faidx)?;
        info!("index done, write index file to: {}", fai_name);
    }

    info!("read gff file: {}", gff);
    let mut reader = file_reader(Some(&gff)).map(gff::Reader::new)?;
    info!("read genome file: {}", fasta);
    let mut fa_index_reader = indexed_reader::Builder::default()
        .build_from_path(fasta)
        .map_err(GffError::FaidxNotExists)?;
    let mut fo = file_writer(out.as_ref(), 6u32).map(fasta::Writer::new)?;

    let key_wrap = Value::from(name);
    for record in reader.records().flatten() {
        let rec = record.attributes();
        if let Some(ref types) = types {
            if types == record.ty() {
                if let Some(key) = rec.get(key) {
                    if key.eq(&key_wrap) {
                        let reg = Region::new(
                            record.reference_sequence_name(),
                            Interval::from(record.start()..=record.end()),
                        );
                        let rec_new = fa_index_reader.query(&reg)?;
                        trace!(
                            "region info: {}",
                            format!(
                                "{}\t{}\t{}\t{}\t{}\t{}",
                                record.reference_sequence_name(),
                                record.ty(),
                                record.start(),
                                record.end(),
                                record.strand(),
                                key
                            )
                        );
                        let des = record::Definition::new(
                            rec_new.name(),
                            Some(
                                format!("{}:strand{}:{}", record.ty(), record.strand(), key)
                                    .as_bytes()
                                    .to_vec(),
                            ),
                        );
                        let seq = if record.strand().as_ref() == "-" {
                            // complement: report an error when lowercase base in fasta
                            let seqrc = rec_new
                                .sequence()
                                .complement()
                                .rev()
                                .collect::<Result<_, _>>()?;
                            Record::new(des, seqrc)
                        } else {
                            Record::new(des, rec_new.sequence().clone())
                        };
                        fo.write_record(&seq)?;
                    }
                }
            }
        } else if let Some(key) = rec.get(key) {
            if key.eq(&key_wrap) {
                let reg = Region::new(
                    record.reference_sequence_name(),
                    Interval::from(record.start()..=record.end()),
                );
                let rec_new = fa_index_reader.query(&reg)?;
                trace!(
                    "region info: {}",
                    format!(
                        "{}\t{}\t{}\t{}\t{}\t{}",
                        record.reference_sequence_name(),
                        record.ty(),
                        record.start(),
                        record.end(),
                        record.strand(),
                        key
                    )
                );
                let des = record::Definition::new(
                    rec_new.name(),
                    Some(
                        format!("{}:strand{}:{}", record.ty(), record.strand(), key)
                            .as_bytes()
                            .to_vec(),
                    ),
                );
                let seq = if record.strand().as_ref() == "-" {
                    let seqrc = rec_new
                        .sequence()
                        .complement()
                        .rev()
                        .collect::<Result<_, _>>()?;
                    Record::new(des, seqrc)
                } else {
                    Record::new(des, rec_new.sequence().clone())
                };
                fo.write_record(&seq)?;
            }
        }
    }

    info!("all done");
    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
