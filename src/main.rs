fn main() {
    let count: Vec<_> = (0..1000).map(|_| 1).collect();
    let hist = histgram_viz::Histgram::new(&count);
    eprintln!("Hist of Degree\n{}", hist.format(20, 40));
}
