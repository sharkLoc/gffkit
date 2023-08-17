use clap::Parser;
use anyhow::{Error, Ok};

mod gff;
use crate::gff::*;

#[derive(Parser,Debug)]
#[command(
    author = "size_t", 
    version = "version 0.1.1", 
    about= "gffkit: A simple program for gff3 file manipulation", 
    long_about = "gffkit: A simple program for gff3 file manipulation \nsharkLoc <mmtinfo@163.com>")]
#[command(next_line_help = true)]
struct Args {
    #[clap(subcommand)]
    commamd: Subcli,
}

#[derive(Parser,Debug)]
#[allow(non_camel_case_types)]
enum Subcli {
    /// query feature info from GFF3 file
    query {
        /// read GFF from given file path
        gff: Option<String>,
        /// select feature type in gff file column 3; eg. gene,miRNA,exon
        #[arg(short='t', long="type")]
        types: Option<String>,
        /// feature name in gff file, eg. Name, ID, gene
        #[arg(short='k', long="key")]
        key: String,
        /// value of the feature, eg. TP53, CYP2D6
        #[arg(short='n', long="name")]
        name: String,
    }
}

fn main() -> Result<(), Error> {
    let cli = Args::parse();
    match cli.commamd {
        Subcli::query { gff, types, key, name} => if let Some(gff) = gff {
            feature_select(&gff, types, &key, &name)?;
        }
    }
    
    Ok(())
}
