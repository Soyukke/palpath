use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::data::Data;

pub fn b00(a: &str, b: &str) {
    let names: Vec<String> = (b'a'..=b'z').map(|c| (c as char).to_string()).collect();
    // TODO: ここは事前定義かなにか
    let mut data_value: HashMap<String, i32> = HashMap::new();
    let mut rng = rand::thread_rng();
    for name in names.iter() {
        let random_number = rng.gen_range(0..1500);
        data_value.insert(name.to_string(), random_number);
    }

    println!("{:?}", data_value);
    let d = Data::new(data_value);
    println!("{:?}", d);

    let mut s: HashSet<String> = HashSet::new();
    let mut s_new: HashSet<String> = HashSet::new();
    s.insert(a.to_string());
    s.insert(b.to_string());

    for g in 0..10 {
        let s_before = s.clone();
        let sub_s: Vec<String> = s.clone().into_iter().collect();
        for i in 0..sub_s.len() {
            for j in i + 1..sub_s.len() {
                let a = &sub_s[i];
                let b = &sub_s[j];
                let (k, _) = d.combine(a, b);
                s_new.insert(k);
            }
        }
        println!("Generation {g}: {:?}", s);
        if s_before == s {
            println!("Generation {g} is end.");
            break;
        }
    }
}
