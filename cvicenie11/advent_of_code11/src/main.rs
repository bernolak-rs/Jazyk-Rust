use std::{
    fs::{self, File},
    io::BufReader,
};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    //println!("{}", content);
    let v: Vec<&str> = content.split('\n').collect();

    let mut number = 50;
    let mut answer = 0;
    for line in v {
        let f = format!("{line}");
        if line.contains('L') {
            let mut subtrahend = line.strip_prefix('L').expect(&f);
            let mut subtrahend: i32 = subtrahend.parse().unwrap();
            number -= subtrahend;
        }
        if line.contains('R') {
            let mut addends = line.strip_prefix('R').expect(&f);
            let mut addends: i32 = addends.parse().unwrap();
            number += addends;
        }

        if number > 99 {
            number = number % 100;
        }
        while number < 0 {
            number = number + 100;
        }
        if number == 0 {
            answer += 1;
        }
    }

    println!("{answer}");
}
