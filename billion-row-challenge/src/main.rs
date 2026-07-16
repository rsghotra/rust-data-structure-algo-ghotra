/*
Billion Row Challenges
- Determine min/mean/max value per station
- Need to support UTF-8 encoding
- input needs to be generated from the repo: https://github.com/rsghotra/1brc

- Sample file entries:
    Hamburg;12.0
    Bulawayo;8.9
    Patembang;38.8
    St. John's;15.2
    Cracow;12.6
    ...
    Output = {Abha=5.0/18.9/17.5, Abidjan=15.7/26.0/34.1}

*/

use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::BTreeMap;

#[derive(Debug)]
struct Stats {
    min: f64,
    max: f64,
    sum: f64,
    count: f64, 
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: Default::default(),
            count: Default::default(),
        }
    }
}

fn main() {
    /*
        Incrementally read the data
     */
    let path = "measurements.txt";
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut data = BTreeMap::<String, Stats>::new();



    for line in f.lines().flatten() {
        let (city, temp) = match line.split_once(';') {
            Some((city, temp)) => {
                let temp: f64 = temp.parse().unwrap();
                (city, temp)
            },
            None => continue,
        };
        let city_data = data.entry(city.to_string()).or_default();

        city_data.min = temp.min(city_data.min);
        city_data.max = temp.max(city_data.max);
        city_data.sum += temp;
        city_data.count += 1.0;
    }

    // let mut data = data.into_iter().collect::<Vec<_>>();

    // data.sort_unstable_by(|city_a, city_b| {(city_a.0).cmp(&(city_b.0))});

    for (city, stats) in data.into_iter() {
        let avg = if stats.count == 0.0 {
            0.0
        } else {
            stats.sum / stats.count
        };
        println!("{city}: {}/{}/{avg}", stats.min, stats.max);
    }

}
