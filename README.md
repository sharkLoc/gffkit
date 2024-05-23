# gffkit
🦀 A simple program for gff3 file manipulation

## install

```bash
git clone https://github.com/sharkLoc/gffkit.git
cd gffkit
cargo b --release
# mv target/release/gffkit to anywhere you want 
```

## usage

```bash
gffkit --help
gffkit -- gffkit: A simple program for gff3 file manipulation

Version: 0.1.2

Authors: size_t <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/gffkit.git


Usage: gffkit [OPTIONS] <COMMAND>

Commands:
  query  query feature info from GFF3 file
  help   Print this message or the help of the given subcommand(s)

Global Arguments:
      --log <FILE>    if file name specified, write log message to this file, or write to stderr
  -v, --verbosity...  control verbosity of logging, [-v: Error, -vv: Warn, -vvv: Info, -vvvv: Debug, -vvvvv: Trace, defalut: Debug]

Global FLAGS:
  -q, --quiet    be quiet and do not show any extra information
  -h, --help     prints help information
  -V, --version  prints version information

Use "gffkit help [command]" for more information about a command
```
** any bugs please report issues **
