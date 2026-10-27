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
| GC_percent | GC content % (i.e., # of GC / # of ACGT * 100) |
| N50 | Genome N50 (bp) |
| N90 | Genome N90 (bp) |
| L50 | Genome L50 (contigs) | 
| L90 | Genome L90 (contigs) | 
| Ns_per_100Kb | Number of ambiguous bases per 100 Kbp (i.e., # of non-ACGT / genome length * 100,000) |

## ✅ Validation

QC metrics for 41,777 prokaryotic genomes (i.e., species representative genomes from the [proGenomes3 database](https://academic.oup.com/nar/article/51/D1/D760/6835361)) were calculated using `rustyqc` v0.2.0 and [QUAST](https://github.com/ablab/quast) v5.2.0. All `rustyqc` QC metrics were identical to their QUAST counterparts (you can see for yourself [here](https://doi.org/10.5281/zenodo.23697667)!).

## ⭐ Citation

Enjoying `rustyqc`? Feel free to cite my little side-quest / Rust practice session:

> Laura Carroll. rustyqc: a tool for calculating minimal QC metrics for assembled genomes, written in Rust, version 0.2.0. https://github.com/lmc297/rustyqc. doi: https://doi.org/10.5281/zenodo.23697667. 

