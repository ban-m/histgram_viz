//! Histgram Viz.
#[derive(Debug, Clone)]
pub struct Histgram {
    raw_count: Vec<usize>,
}

impl Histgram {
    pub fn new(data: &[usize]) -> Self {
        let raw_count = data.to_vec();
        Self { raw_count }
    }
    // Print raw data in height x row format.
    pub fn format(&self, column: usize, row: usize) -> String {
        if self.raw_count.is_empty() {
            return String::new();
        }
        // Determine the draw range.
        let min = *self.raw_count.iter().min().unwrap();
        let max = *self.raw_count.iter().max().unwrap();
        let tick = ((max - min) / row).max(1);
        let counts: Vec<_> = (0..row)
            .map(|i| {
                self.raw_count
                    .iter()
                    .filter(|&&x| match i {
                        0 => x < min + tick,
                        x if x == row => min + tick * row <= x,
                        _ => min + tick * i <= x && x < min + tick * (i + 1),
                    })
                    .count()
            })
            .collect();
        // Determine the header digit.
        let digit = format!("{}", max).chars().count();
        let digit = digit.max(format!("{}", row).chars().count());
        // Determine column tick.
        let c_tick = counts.iter().max().unwrap() / column;
        use std::iter;
        let rows: Vec<_> = counts
            .iter()
            .enumerate()
            .map(|(idx, count)| {
                let count: String = iter::repeat('*').take(count / c_tick).collect();
                let head = format!("{}", idx * tick + min);
                let len = head.chars().count();
                let head_pad: String = iter::repeat(' ').take(digit - len).collect();
                format!("{}{}|{}", head_pad, head, count)
            })
            .collect();
        let header: String = iter::repeat('-').take(digit + column + 10).collect();
        let extra = format!("*={}", c_tick);
        //eprintln!("{}", rows.len());
        format!("{}\n{}\n{}", extra, header, rows.join("\n"))
    }
}
