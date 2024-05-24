use clap::{ArgAction, Parser};

mod cmd;
mod logger;
use logger::*;
mod error;
mod utils;
use cmd::{extract::extract_seq, query::feature_select};

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "0.1.2",
    disable_help_flag = true,
    disable_version_flag = true,
    propagate_version = true,
    about = "gffkit: A simple program for gff3 file manipulation",
    long_about = "gffkit: A simple program for gff3 file manipulation \nsharkLoc <mmtinfo@163.com>"
)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
    \n\nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/gffkit.git\
    \n\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"gffkit help [command]\" for more information about a command")]
struct Args {
    #[clap(subcommand)]
    commamd: Subcli,

    /// if file name specified, write log message to this file, or write to stderr
    #[arg(long = "log", global = true, help_heading = Some("Global Arguments"), value_name = "FILE")]
    pub logfile: Option<String>,

    /// control verbosity of logging, [-v: Error, -vv: Warn, -vvv: Info, -vvvv: Debug, -vvvvv: Trace, defalut: Debug]
    #[arg(short = 'v', long = "verbosity", action = ArgAction::Count, global = true,
    default_value_t = 4, help_heading = Some("Global Arguments")
    )]
    pub verbose: u8,

    /// be quiet and do not show any extra information
    #[arg(short = 'q', long = "quiet", global= true, help_heading = Some("Global FLAGS"))]
    pub quiet: bool,

    /// prints help information
    #[arg(short = 'h', long, action = ArgAction::Help, global= true, help_heading = Some("Global FLAGS"))]
    pub help: Option<String>,

    /// prints version information
    #[arg(short = 'V', long, action = ArgAction::Version, global= true, help_heading = Some("Global FLAGS"))]
    pub version: Option<String>,
}
#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
enum Subcli {
    /// query feature info from GFF3 file
    query {
        /// read GFF file from given path
        gff: Option<String>,
        /// select feature type in gff file column 3; eg. gene,miRNA,exon
        #[arg(short = 't', long = "type", value_name = "STR")]
        types: Option<String>,
        /// feature name in gff file, eg. Name, ID, gene
        #[arg(short = 'k', long = "key", value_name = "STR")]
        key: String,
        /// value of the feature, eg. TP53, CYP2D6
        #[arg(short = 'n', long = "name", value_name = "STR")]
        name: String,
        /// output file name or write to stdout, files ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short, long, value_name = "FILE")]
        out: Option<String>,
    },
    /// extract feature sequence from fasta file
    #[command(
        visible_alias = "extr",
        before_help = "note: 
    1. feature in strand - chain, output reverse-complement sequence
    2. input indexed fasta, or run command with arg --faidx
    3. show each region info in log when arg -vvvvv used"
    )]
    extract {
        /// read gff file from given path
        #[arg(short, long, value_name = "FILE")]
        gff: String,
        /// input genome.fa faidx needed, or use opt --faidx
        #[arg(short, long, value_name = "FILE")]
        fasta: String,
        /// if set, create index for genome.fa when .fai not exists
        #[arg(long, help_heading = Some("FLAGS"))]
        faidx: bool,
        /// select feature type in gff file column 3; eg. gene,miRNA,exon
        #[arg(short = 't', long = "type", value_name = "STR")]
        types: Option<String>,
        /// feature name in gff file, eg. Name, ID, gene
        #[arg(short = 'k', long = "key", value_name = "STR")]
        key: String,
        /// value of the feature, eg. TP53, CYP2D6
        #[arg(short = 'n', long = "name", value_name = "STR")]
        name: String,
        /// output file name or write to stdout, files ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short, long, value_name = "FILE")]
        out: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    logger(cli.verbose, cli.logfile, cli.quiet)?;

    match cli.commamd {
        Subcli::query {
            gff,
            types,
            key,
            name,
            out,
        } => {
            feature_select(gff, types, &key, &name, out)?;
        }
        Subcli::extract {
            gff,
            fasta,
            faidx,
            types,
            key,
            name,
            out,
        } => {
            extract_seq(&fasta, faidx, &gff, types, &key, &name, out)?;
        }
    }

    Ok(())
}
