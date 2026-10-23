use std::path::Path;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;
use std::any::type_name_of_val;
use std::convert::TryInto;
use std::fs;
use std::io::Write;
use clap::Parser;
use seq_io::fasta::{Reader,Record};

// Parse command line args
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to FASTA
    #[arg(short = 'i', long = "input")]
    input: std::path::PathBuf,

    /// output: Desired path to output TSV
    #[arg(short = 'o', long = "output")]
    output: std::path::PathBuf,
}

#[derive(Debug)]
struct QCReport {
    fname: String,
    gname: String,
    ncontigs: u64,
    maxcontig: usize,
    glen: usize,
    gc: f64,
    n50: usize,
    n75: usize,
    l50: usize,
    l75: usize,
    nperkbp: f64,
}

fn get_genome_stats(contiglens: &HashMap<String, usize>, glen: usize) -> HashMap<String, usize> {
    let mut genome_stats = HashMap::new();
    let n50_thresh = (glen as f64) * 0.5;
    let n75_thresh = (glen as f64) * 0.75;
    
    let mut len_vec: Vec<(&String, &usize)> = contiglens.iter().collect();
    len_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut n_counter: usize = 0;
    let mut l_counter: usize = 0;
    for i in len_vec.iter() {
        n_counter += i.1;
        l_counter += 1;
        if (n_counter as f64) >= n50_thresh && !genome_stats.contains_key("N50") {
            genome_stats.insert(String::from("N50"), *i.1);
            genome_stats.insert(String::from("L50"), l_counter);
        }
        if (n_counter as f64) >= n75_thresh && !genome_stats.contains_key("N75") {
            genome_stats.insert(String::from("N75"), *i.1);
            genome_stats.insert(String::from("L75"), l_counter);
        }
    }
    genome_stats
}

fn count_non_acgt(dna: &str) -> usize {
    dna.chars()
    .filter(|&c| !matches!(c, 'A' | 'C' | 'G' | 'T'))
    .count()
}

fn get_qc<R: Read>(fname: String, gname: String, mut contigs: seq_io::fasta::Reader<R>) -> QCReport {

    let mut ncontigs = 0;
    let mut glen = 0;
    let mut contiglens = HashMap::new();
    let mut bpcounts = HashMap::new();

    while let Some(record) = contigs.next(){
        let record = record.expect("Error reading record.");

        for s in record.seq_lines(){   
            let seqid = record.id().expect("Missing sequence ID").to_string();      
            let seqlen = s.len();
            let seqseq = std::str::from_utf8(s).unwrap().to_uppercase();

            let acount = seqseq.chars().filter(|&c| c == 'A').count();
            let ccount = seqseq.chars().filter(|&c| c == 'C').count();
            let gcount = seqseq.chars().filter(|&c| c == 'G').count();
            let tcount = seqseq.chars().filter(|&c| c == 'T').count();
            let ncount = count_non_acgt(&seqseq);

            *bpcounts.entry("A").or_insert(0) += acount;
            *bpcounts.entry("C").or_insert(0) += ccount;
            *bpcounts.entry("G").or_insert(0) += gcount;
            *bpcounts.entry("T").or_insert(0) += tcount;
            *bpcounts.entry("N").or_insert(0) += ncount;

            *contiglens.entry(seqid).or_insert(0) += seqlen;

            glen += seqlen;
            
        }
        ncontigs += 1;
    }

    let gc = bpcounts.get("G").unwrap() + bpcounts.get("C").unwrap();
    let maxcontig = contiglens.values().max().unwrap();

    let genome_stats = get_genome_stats(&contiglens, glen);

    println!("Number of contigs: {}", ncontigs);
    println!("Longest contig: {} bp", maxcontig);
    println!("Total genome length: {} bp", glen);
    println!("GC content: {:.2}%", (gc as f64 / glen as f64) * 100 as f64);
    println!("N50: {} bp", genome_stats.get("N50").unwrap());
    println!("N75: {} bp", genome_stats.get("N75").unwrap());
    println!("L50: {} contigs", genome_stats.get("L50").unwrap());
    println!("L75: {} contigs", genome_stats.get("L75").unwrap());
    println!("Number of Ns: {}", bpcounts.get("N").unwrap());

    let final_stats = QCReport {
        fname: fname,
        gname: gname,
        ncontigs: ncontigs,
        maxcontig: *maxcontig,
        glen: glen,
        gc: (gc as f64 / glen as f64) * 100 as f64,
        n50: *genome_stats.get("N50").unwrap(),
        n75: *genome_stats.get("N75").unwrap(),
        l50: *genome_stats.get("L50").unwrap(),
        l75: *genome_stats.get("L75").unwrap(),
        nperkbp: (*bpcounts.get("N").unwrap() as f64) / (glen as f64) * 100000.0,
    };
    final_stats
}

fn print_final_report(final_stats: QCReport, outpath: &PathBuf) -> std::io::Result<()> {
    let header = vec!["#filename", "prefix",
                                 "N_contigs", "Max_contig_length_bp",
                                 "Genome_length_bp", "GC_percent",
                                 "N50", "N75",
                                 "L50", "L75",
                                 "Ns_per_100Kb"];

    let final_line = vec![final_stats.fname, final_stats.gname,
                                          final_stats.ncontigs.to_string(),
                                          final_stats.maxcontig.to_string(),
                                          final_stats.glen.to_string(),
                                          format!("{:.2}", final_stats.gc),
                                          final_stats.n50.to_string(),
                                          final_stats.n75.to_string(),
                                          final_stats.l50.to_string(),
                                          final_stats.l75.to_string(),
                                          format!("{:.2}", final_stats.nperkbp)];
    
    let mut outfile = fs::OpenOptions::new()
                                .create(true).append(true)
                                .open(&outpath)?;
    outfile.write_all(header.join("\t").as_bytes())?;
    outfile.write_all("\n".as_bytes())?;
    outfile.write_all(final_line.join("\t").as_bytes())?;

    Ok(())
}

fn main() {
    let args = Cli::parse();

    let mut reader = Reader::from_path(&args.input).unwrap();
    let fname = String::from(args.input.file_name().unwrap().to_str().unwrap());
    let gname = String::from(args.input.file_stem().unwrap().to_str().unwrap());
    let outpath = &args.output;
    
    let final_stats = get_qc(fname, gname, reader);
    
    print_final_report(final_stats, outpath);
}
