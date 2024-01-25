use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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

    pub fn find_compact(&self) {
        // a, bから生成されるセットをすべて計算する
        // セットが最大数のa, bを決定し、Sから生成されるセットをすべて取り除く
        let mut generated_set: HashMap<(String, String), HashSet<String>> = HashMap::new();
        let n_data = self.values.keys().len();
        let keys: Vec<String> = self.values.keys().cloned().collect();
        for i in 0..n_data {
            for j in (i + 1)..n_data {
                let p = &keys[i];
                let q = &keys[j];
                let mut s: HashSet<String> = HashSet::new();
                s.insert(p.clone().to_string());
                s.insert(q.clone().to_string());
                let mut s2: HashSet<String> = HashSet::new();
                let (k, _) = self.combine(p, q);
                s2.insert(k.to_string());

                loop {
                    let mut s_new: HashSet<String> = HashSet::new();
                    for p in s.iter() {
                        for q in s2.iter() {
                            let (k, _) = self.combine(p, q);
                            s_new.insert(k);
                        }
                    }
                    s = s2;
                    s2 = s_new;
                    let is_a_containing_b = s2.iter().all(|x| s.contains(x));
                    if is_a_containing_b {
                        generated_set.insert((p.clone().to_owned(), q.clone().to_owned()), s);
                        break;
                    }
                }
            }
        }
        //for k in generated_set.keys() {
        //    println!("{:?}", generated_set.get(k).unwrap());
        //}
        // 最大数のセットを持っている(a, b)を取り出す
        // generated_setから(a, b)を消す
        // generated_setのvalueから(a, b)のvalueを取り出す
        let mut s_compact: HashSet<String> = HashSet::new();
        loop {
            let (k_max, n_max) = generated_set.keys().fold(
                (("".to_string(), "".to_string()), usize::MIN),
                |res, key| {
                    let subset = generated_set.get(key).unwrap();
                    let n_data = subset.len();
                    if res.1 < n_data {
                        (key.clone(), n_data)
                    } else {
                        res
                    }
                },
            );
            if n_max == 0 {
                break;
            }
            println!(
                "最大パル種族を生み出せるパルの組み合わせ: {:?}, {}",
                k_max, n_max
            );

            s_compact.insert(k_max.0.clone());
            s_compact.insert(k_max.1.clone());
            let s_max = generated_set.get(&k_max).unwrap().clone();

            generated_set.iter_mut().for_each(|x| {
                let k = x.0;
                let mut v = x.1;
                for e in s_max.iter() {
                    v.remove(e);
                }
            });
        }
        println!("s_compact: {:?}", s_compact);
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

    //cargo test b00 -- --nocapture
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
