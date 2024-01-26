use anyhow::{Context, Result};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, serde::Deserialize, Clone)]
struct Record {
    id: i32,
    name: String,
    value: i32,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Special {
    parent1: String,
    parent2: String,
    child: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    values: HashMap<String, i32>,
    ids: HashMap<String, i32>,
    specials: HashMap<(String, String), String>,
}

impl Data {
    pub fn new(
        values: HashMap<String, i32>,
        ids: HashMap<String, i32>,
        specials: HashMap<(String, String), String>,
    ) -> Self {
        //Self { values, data }
        Self {
            values,
            ids,
            specials,
        }
    }

    pub fn from_csv() -> Result<Data> {
        let mut rdr = csv::Reader::from_path("./assets/data.csv")?;
        let mut records: Vec<Record> = vec![];
        let mut data_value: HashMap<String, i32> = HashMap::new();
        let mut ids: HashMap<String, i32> = HashMap::new();
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Record = result?;
            records.push(record.clone());
            data_value.insert(record.name.clone(), record.value);
            ids.insert(record.name, record.id);
        }

        let mut rdr = csv::Reader::from_path("./assets/special.csv")?;
        let mut records: Vec<Special> = vec![];
        let mut specials: HashMap<(String, String), String> = HashMap::new();
        for result in rdr.deserialize() {
            let record: Special = result?;
            records.push(record.clone());
            specials.insert((record.parent1, record.parent2), record.child);
        }

        let d = Data::new(data_value, ids, specials);
        Ok(d)
    }

    pub fn kv(&self, a: &str) -> (String, i32) {
        let v_a = self.values.get(a).unwrap();
        (a.to_string(), *v_a)
    }

    pub fn combine(&self, a: &str, b: &str) -> Result<(String, i32)> {
        let v_a = self
            .values
            .get(a)
            .context(format!("パルが見つからない!: {a}"))?;
        let v_b = self
            .values
            .get(b)
            .context(format!("パルが見つからない!: {b}"))?;

        if self.specials.get(&(a.to_string(), b.to_string())).is_some() {
            let child = self.specials.get(&(a.to_string(), b.to_string())).unwrap();
            let value = self.values.get(child).unwrap();
            return Ok((child.to_string(), *value));
        }
        if self.specials.get(&(b.to_string(), a.to_string())).is_some() {
            let child = self.specials.get(&(b.to_string(), a.to_string())).unwrap();
            let value = self.values.get(child).unwrap();
            return Ok((child.to_string(), *value));
        }

        // 1桁目は切り捨てする
        let v_c = ((v_a + v_b) as f64) / 2.0;

        // distanceが近くてidが小さい方を優先する
        let mut closest_distance = f64::INFINITY;
        let mut closest_key: Option<&str> = None;
        let mut closest_value: Option<i32> = None;
        let mut closest_id: i32 = i32::MAX;

        let reigai: Vec<String> = self.specials.values().cloned().collect();

        for key in self.values.keys() {
            if reigai.contains(key) {
                continue;
            }
            let x = self.values.get(key).unwrap();
            let id = self.ids.get(key).unwrap();
            let distance = (*x as f64 - v_c).abs();
            if distance < closest_distance {
                closest_id = *id;
                closest_distance = distance;
                closest_key = Some(key);
                closest_value = Some(*x);
            } else if distance == closest_distance && *id < closest_id {
                closest_id = *id;
                closest_key = Some(key);
                closest_value = Some(*x);
            }
        }
        Ok((closest_key.unwrap().to_string(), closest_value.unwrap()))
    }

    pub fn find_compact(&self) -> Result<()> {
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
                let (k, _) = self.combine(p, q)?;
                s2.insert(k.to_string());

                loop {
                    let mut s_new: HashSet<String> = HashSet::new();
                    for p in s.iter() {
                        for q in s2.iter() {
                            let (k, _) = self.combine(p, q)?;
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
        Ok(())
    }
}
