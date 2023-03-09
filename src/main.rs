use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

const BINS_COUNT: usize = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide an input file as an argument.");
    }
    let input_file = &args[1];
    let output_file = input_file.replace(".txt",".svg");

    // note: "\[(\d+\.\d+)\].*seq=(\d+).*time=(\d+\.\d+)" captures timestamp and seq# as well
    let re = Regex::new(r".*time=(\d+\.\d+)").unwrap();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(captures) = re.captures(&line) {
            let rtt: f64 = captures.get(1).unwrap().as_str().parse().unwrap();
            data.push(rtt);
        }
    }
    let data_len = data.len();
    let h = Histogram::from_slice(&data[0..data_len-1], HistogramBins::Count(BINS_COUNT))
        .style(&BoxStyle::new().fill("orange"));

    let v = ContinuousView::new().add(h);
    
    Page::single(&v).save(output_file).expect("can't save output file");
}
