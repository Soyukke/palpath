use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, serde::Deserialize, Clone)]
struct Record {
    name: String,
    value: i32,
}

#[derive(Debug, Clone)]
pub struct Data {
    values: HashMap<String, i32>,
    //data: HashMap<(&'a str, &'a str), &'a str>,
}

impl Data {
    pub fn new(values: HashMap<String, i32>) -> Self {
        //Self { values, data }
        Self { values }
    }

    pub fn from_csv() -> Result<Data> {
        let mut rdr = csv::Reader::from_path("./assets/data.csv")?;
        let mut records: Vec<Record> = vec![];
        let mut data_value: HashMap<String, i32> = HashMap::new();
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Record = result?;
            records.push(record.clone());
            data_value.insert(record.name, record.value);
        }

        let d = Data::new(data_value);
        Ok(d)
    }

    pub fn kv(&self, a: &str) -> (String, i32) {
        let v_a = self.values.get(a).unwrap();
        (a.to_string(), *v_a)
    }

    pub fn combine(&self, a: &str, b: &str) -> (String, i32) {
        let v_a = self.values.get(a).unwrap();
        let v_b = self.values.get(b).unwrap();
        let v_c = ((v_a + v_b) as f64) / 2.0;

        let mut closest_distance = f64::INFINITY;
        let mut closest_key: Option<&str> = None;
        let mut closest_value: Option<i32> = None;
        for key in self.values.keys() {
            let x = self.values.get(key).unwrap();
            let distance = (*x as f64 - v_c).abs();
            if distance < closest_distance {
                closest_distance = distance;
                closest_key = Some(key);
                closest_value = Some(*x);
            }
        }
        (closest_key.unwrap().to_string(), closest_value.unwrap())
    }
}

pub fn example(parent1: &str, parent2: &str) -> Result<()> {
    println!("example!");
    let mut rdr = csv::Reader::from_path("./assets/data.csv")?;
    let mut records: Vec<Record> = vec![];
    let mut data_value: HashMap<String, i32> = HashMap::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
        records.push(record.clone());
        data_value.insert(record.name, record.value);
    }

    let mut s: HashSet<String> = HashSet::new();
    s.insert(parent1.to_string());
    s.insert(parent2.to_string());

    let d = Data::new(data_value);
    for g in 0..10 {
        println!("Generation {g}: {:?}", s);
        let s_before = s.clone();
        let sub_s: Vec<String> = s.clone().into_iter().collect();
        for i in 0..sub_s.len() {
            for j in i + 1..sub_s.len() {
                let a = &sub_s[i];
                let b = &sub_s[j];
                let (k, _) = d.combine(a, b);
                s.insert(k);
            }
        }
        if s_before == s {
            println!("Generation {g} is end.");
            break;
        }
    }

    Ok(())
}

pub fn example2() -> Result<Data> {
    println!("example!");
    let mut rdr = csv::Reader::from_path("./assets/data.csv")?;
    let mut records: Vec<Record> = vec![];
    let mut data_value: HashMap<String, i32> = HashMap::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
        records.push(record.clone());
        data_value.insert(record.name, record.value);
    }

    let d = Data::new(data_value);
    Ok(d)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn a00() {
        let names: Vec<String> = (b'a'..=b'z').map(|c| (c as char).to_string()).collect();
        let mut data_value: HashMap<String, i32> = HashMap::new();
        let mut rng = rand::thread_rng();
        for name in names.iter() {
            let random_number = rng.gen_range(0..1500);
            data_value.insert(name.to_string(), random_number);
        }
        let d = Data::new(data_value);
        println!("{:?}", d);
        let (k, v) = d.kv("a");
        println!("(key, value) = ({:?}, {})", k, v);
        let (k, v) = d.kv("b");
        println!("(key, value) = ({:?}, {})", k, v);
        let (k, v) = d.combine("a", "b");
        println!("(key, value) = ({:?}, {})", k, v);
        //let mut data: HashMap<(&str, &str), &str> = HashMap::new();
        //data.insert(("a", "b"), "c");
        //data.insert(("a", "c"), "d");
        //data.insert(("a", "e"), "d");
    }

    //cargo test b00 -- --nocapture
    #[test]
    fn b00() {
        let mut names: Vec<String> = (b'a'..=b'z').map(|c| (c as char).to_string()).collect();
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
        s.insert("a".to_string());
        s.insert("b".to_string());

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

        //let mut data: HashMap<(&str, &str), &str> = HashMap::new();
        //data.insert(("a", "b"), "c");
        //data.insert(("a", "c"), "d");
        //data.insert(("a", "e"), "d");
    }

    #[test]
    fn cs00() {
        //example("シルキーヌ", "ヘルカルダ");
        let d = example2().unwrap();
        let parent = "ヘルカルダ";
        let mut parent2 = "ホウロック".to_string();
        for i in 0..10 {
            let (child, v) = d.combine(parent, &parent2);
            println!("{parent} x {parent2} = {child}");
            parent2 = child;
        }
    }
}
