mod Graph;
pub mod graph;

fn main() {
    let data = read_file("2018_worldcup_v3");
    let (homeg,awayg) = split(data);
    let mut g: graph::Graph<String> = graph::Graph::new();
    g.add_edge(homeg.0, homeg.1, homeg.2);
    g.add_edge(awayg.0, awayg.1, awayg.2);
    for e in g.dfs_iter(){
        println!("{}",e)
    }
}

use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Vec<(u32, u32, u32, u32)> {
    let mut result: Vec<(u32, u32, u32, u32)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<u32>().unwrap();
        let y = v[1].parse::<u32>().unwrap();
        let z = v[2].parse::<u32>().unwrap();
        let h = v[3].parse::<u32>().unwrap();
        result.push((x, y, z, h));
    }
    return result;
}

fn split(data:Vec<(u32, u32, u32)>) -> (Vec<(u32, u32, u32)>,Vec<(u32, u32, u32)>) {
    let mut homeg: Vec<(u32, u32, u32)> = Vec::new();
    let mut awayg: Vec<(u32, u32, u32)> = Vec::new();
    for i in 0..66 {
            homeg.push((data[i].0,data[i].3,data[i].1));
            awayg.push((data[i].3,data[i].0,data[i].2));
        }
    return (homeg,awayg);
}