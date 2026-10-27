# 🦀 rustyqc
A tool for calculating minimal QC metrics for assembled genomes, written in Rust

## 🛠️ Installation

To install `rustyqc`, run the following command in your terminal:
```
cargo install --git https://github.com/lmc297/rustyqc
```

## 🧰 Usage
```
Usage: rustyqc [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Path to FASTA
  -o, --output <OUTPUT>  Desired path to output file
      --outfmt <OUTFMT>  Output file format; one of "tsv" or "text" [default: tsv]
  -h, --help             Print help
  -V, --version          Print version
```

## 📥 Input
`rustyqc` takes an assembled (meta)genome, in FASTA format, as input.

## 📤 Output
`rustyqc` outputs basic QUAST-like QC metrics (e.g., genome length, number of contigs, N50) in TSV format (default).

| Column Name  | Description |
| ------------- | ------------- |
| #filename | File supplied as input |
| prefix | File basename  |
| N_contigs | Number of contigs |
| Max_contig_length_bp | Longest contig (bp) |
| Genome_length_bp | Total genome length (bp) |
| GC_percent | GC content % (i.e., # of GC / genome length * 100; note that currently, rustyqc includes ambiguous/N characters in the genome length calculation for GC content, while QUAST does not) |
| N50 | Genome N50 (bp) |
| N75 | Genome N75 (bp) |
| L50 | Genome L50 (contigs) | 
| L75 | Genome L75 (contigs) | 
| Ns_per_100Kb | Number of ambiguous bases per 100 Kbp (i.e., # of non-ACGT / genome length * 100,000) |

## ✅ Validation

QC metrics for 41,777 prokaryotic genomes (i.e., species representative genomes from the [proGenomes3 database](https://academic.oup.com/nar/article/51/D1/D760/6835361)) were calculated using `rustyqc` v0.1.1 and [QUAST](https://github.com/ablab/quast) v5.2.0. All `rustyqc` QC metrics were identical to their QUAST counterparts, except for GC content; currently, `rustyqc` includes ambiguous/N characters in the genome length calculation for GC content, while QUAST does not (this may be changed in the future).
