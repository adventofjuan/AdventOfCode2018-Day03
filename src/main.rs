use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_file(file: BufReader<&File>) -> HashMap<(i32, i32), i32> {
    let mut res = HashMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        let tokens:Vec<&str> = l.split(" ").collect();

        //let id = &tokens[0][1..].parse::<i32>().unwrap();
        let xy: Vec<&str> = tokens[2].split(",").collect();
        let x = xy[0].parse::<i32>().unwrap();
        let y = &xy[1][..xy[1].len()-1].parse::<i32>().unwrap();

        let size:Vec<&str> = tokens[3].split("x").collect();
        let sizex = size[0].parse::<i32>().unwrap();
        let sizey = size[1].parse::<i32>().unwrap();

        for sx in 0..sizex {
            for sy in 0..sizey {
                let entry = res.entry((x + sx, y + sy)).or_insert(0);
                *entry += 1;
            }
        }
    }
    res
}

fn part01(values: &HashMap<(i32, i32), i32>) {
    let mut sum = 0;
    for (_,v) in values.iter() {
        if *v > 1 {
            sum += 1;
        }
    }
    println!("Part01 result = {}", sum);
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let values = parse_file(file);
    part01(&values);
}
