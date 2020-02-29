use std::io::prelude::*;
use hound;

fn main() {
    let stdin = std::io::stdin();
    let file = stdin.lock();

    let (mut x_min, mut x_max) = (std::f64::MAX,std::f64::MIN);
    let (mut y_min, mut y_max) = (std::f64::MAX,std::f64::MIN);

    let pts: Vec<(f64,f64)> = file.lines().map(|line| {
        let line = line.expect("Read Error");
        let mut nums = line.split(' ');
        (nums.next().unwrap().parse::<f64>().unwrap(),
        nums.next().unwrap().parse::<f64>().unwrap())
    }).inspect(|&(x,y)| {
        y_min = y_min.min(y);
        x_min = x_min.min(x);
        y_max = y_max.max(y);
        x_max = x_max.max(x);
    }).collect();

    let x_span = x_max - x_min;
    let y_span = y_max - y_min;
    let span = x_span.max(y_span);
    
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buffer = std::io::Cursor::new(vec![0u8;0]);
    let mut writer = hound::WavWriter::new(&mut buffer, spec).unwrap();

    for (x,y) in pts {
        let x = ((x_span/span) * (std::i32::MAX - 1) as f64 * (2.0*(x - x_min)/x_span - 1.0)) as i32;
        let y = ((y_span/span) * (std::i32::MAX - 1) as f64 * (2.0*(y - y_min)/y_span - 1.0)) as i32;
        writer.write_sample(x).unwrap();
        writer.write_sample(y).unwrap();
    }

    writer.finalize().unwrap();

    std::io::stdout().write_all(buffer.get_ref()).unwrap();
}
