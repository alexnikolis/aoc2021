use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

fn count_depth_increases(depth_readings: &[usize]) -> u32 {
    depth_readings
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<usize>>()
        .as_slice()
        .windows(2)
        .fold(0, |acc, w| if w[0] < w[1] { acc + 1} else { acc })
}

fn main() {
    let file = File::open("./src/depths.txt").expect("no such file");
    let buf = BufReader::new(file);
    let depths: Vec<usize> = buf.lines()
        .map(|l| l.expect("could not read line").parse::<usize>().unwrap())
        .collect();

    println!("The number of depth increases is {:?}", count_depth_increases(depths.as_slice()));
}


