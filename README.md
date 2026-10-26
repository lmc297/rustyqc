# rustyqc
A tool for calculating minimal QC metrics for assembled genomes, written in Rust

## Usage
```
Usage: rustyqc [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Path to FASTA
  -o, --output <OUTPUT>  Desired path to output file
      --outfmt <OUTFMT>  Output file format; one of "tsv" or "text" [default: tsv]
  -h, --help             Print help
  -V, --version          Print version
```

## Input
`rustyqc` takes an assembled (meta)genome, in FASTA format, as input.

## Output
`rustyqc` outputs basic QUAST-like QC metrics (e.g., genome length, number of contigs, N50) in TSV format (default).

| Column Name  | Description |
| ------------- | ------------- |
| #filename | File supplied as input |
| prefix | File basename  |
| N_contigs | Number of contigs |
| Max_contig_length_bp | Longest contig (bp) |
| Genome_length_bp | Total genome length (bp) |
| GC_percent | GC content (%) |
| N50 | Genome N50 (bp) |
| N75 | Genome N75 (bp) |
| L50 | Genome L50 (contigs) | 
| L75 | Genome L75 (contigs) | 
| Ns_per_100Kb | Number of ambiguous bases per 100 Kbp (i.e., # of non-ACGT / genome length * 100,000) |
