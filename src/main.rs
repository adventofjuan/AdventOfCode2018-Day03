use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_file(file: BufReader<&File>) -> HashMap<(i32, i32), (i32, Vec<i32>)> {
    let mut res = HashMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        let tokens:Vec<&str> = l.split(" ").collect();

        let id = &tokens[0][1..].parse::<i32>().unwrap();
        let xy: Vec<&str> = tokens[2].split(",").collect();
        let x = xy[0].parse::<i32>().unwrap();
        let y = &xy[1][..xy[1].len()-1].parse::<i32>().unwrap();

        let size:Vec<&str> = tokens[3].split("x").collect();
        let sizex = size[0].parse::<i32>().unwrap();
        let sizey = size[1].parse::<i32>().unwrap();

        for sx in 0..sizex {
            for sy in 0..sizey {
                let entry = res.entry((x + sx, y + sy)).or_insert((0, Vec::new()));
                entry.0 += 1;
                if !entry.1.contains(id) {
                    entry.1.push(*id);
                }
            }
        }
    }
    res
}

fn part01(values: &HashMap<(i32, i32), (i32, Vec<i32>)>) {
    let mut sum = 0;
    for (_,v) in values.iter() {
        if v.0 > 1 {
            sum += 1;
        }
    }
    println!("Part01 result = {}", sum);
}

fn part02(values: &HashMap<(i32, i32), (i32, Vec<i32>)>) {
    let mut overlaps = [0; 1288];

    for (_,v) in values.iter() {
        if v.1.len() > 1 {
            for ids in &v.1 {
                let uid = *ids as usize;
                overlaps[uid] += 1;
            }
        }
    }

    for (i, elem) in overlaps.iter_mut().enumerate() {
        if *elem == 0 && i != 0{
            println!("Part 02 result = {}", i);
        }
    }
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let values = parse_file(file);
    part01(&values);
    part02(&values);
}
