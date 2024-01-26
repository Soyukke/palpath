use anyhow::Result;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::data::Data;

#[derive(Debug, Clone)]
pub struct PalNode {
    pub name: String,
    pub parents: Option<(Box<PalNode>, Box<PalNode>)>,
}

impl PalNode {
    pub fn new(name: &str, parents: (&PalNode, &PalNode)) -> Self {
        Self {
            name: name.to_string(),
            parents: Some((Box::new(parents.0.clone()), Box::new(parents.1.clone()))),
        }
    }

    pub fn terminal(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parents: None,
        }
    }
}

use std::io::Write;

pub fn write_dot<W: Write>(node: &PalNode, writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "digraph PalNode {{")?;
    writeln!(writer, "  node [shape=box];")?;
    write_node(node, writer)?;
    writeln!(writer, "}}")?;
    Ok(())
}

fn write_node<W: Write>(node: &PalNode, writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "  {} [label=\"{}\"];", node.name, node.name)?;
    if let Some((left, right)) = &node.parents {
        write_node(left, writer)?;
        write_node(right, writer)?;
        writeln!(writer, "  {} -> {};", left.name, node.name)?;
        writeln!(writer, "  {} -> {};", right.name, node.name)?;
    }
    Ok(())
}

pub fn find_target_path(males: Vec<&str>, females: Vec<&str>, target: &str) -> Result<()> {
    let d = Data::from_csv()?;
    let male_nodes: Vec<PalNode> = males.iter().map(|name| PalNode::terminal(name)).collect();
    let female_nodes: Vec<PalNode> = females.iter().map(|name| PalNode::terminal(name)).collect();
    let mut s: Vec<PalNode> = vec![];
    // NOTE: 次の世代を計算するときは、オスメスどちらかを必ず用意できるから、malesもfemalesも1つのセットに寄せる
    s.append(&mut male_nodes.clone());
    s.append(&mut female_nodes.clone());
    let mut s_next: Vec<PalNode> = vec![];

    // Generation 1.
    for p in male_nodes.iter() {
        for q in female_nodes.iter() {
            let (k, _) = d.combine(&p.name, &q.name)?;
            let node = PalNode::new(&k, (p, q));
            println!("{:?}", node);
            s_next.push(node);
        }
    }

    let mut target_node: Option<PalNode> = None;
    let mut s_fix = s.clone();
    let mut s_next_fix = s_next.clone();
    let mut targets: Vec<PalNode> = vec![];

    // Generation 2以降
    for _ in 0..100 {
        let mut is_found = false;
        for p in s_fix.iter() {
            //if is_found {
            //    break;
            //}
            for q in s_next_fix.iter() {
                let (k, _) = d.combine(&p.name, &q.name)?;
                // ここでtargetが出てこないか探す
                let node = PalNode::new(&k, (p, q));
                //println!("node: {:?}", node);
                if &node.name == target {
                    println!("target was found.");
                    targets.push(node.clone());
                    target_node = Some(node.clone());
                    is_found = true;
                }
                // sにnameが含まれている場合は追加しない。
                let names: Vec<String> = s_fix.iter().map(|n| n.name.clone()).collect();
                let names2: Vec<String> = s_next_fix.iter().map(|n| n.name.clone()).collect();
                let names3: Vec<String> = s_next.iter().map(|n| n.name.clone()).collect();
                if !names.contains(&node.name)
                    && !names2.contains(&node.name)
                    && !names3.contains(&node.name)
                {
                    //println!("if {}", &node.name);
                    s_next.push(node);
                } else {
                    //println!("else {}", &node.name);
                }
            }
        }
        //if s_next.len() == 0 || is_found {
        if s_next.len() == 0 {
            break;
        }

        s.append(&mut s_next_fix.clone());
        s_fix = s.clone();
        s_next_fix = s_next.clone();
        s_next = vec![];
    }

    if targets.len() == 0 {
        println!("ターゲットへの交配パスが見つかりません！");
    }

    for (i, t) in targets.iter().enumerate() {
        println!("====Path {i}===");
        let mut writer = std::io::stdout();
        write_dot(&t, &mut writer).unwrap();
        println!("===============");
    }

    //if let Some(node) = target_node {
    //    let mut writer = std::io::stdout();
    //    write_dot(&node, &mut writer).unwrap();
    //} else {
    //    println!("ターゲットへの交配パスが見つかりません！");
    //}

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::find_target_path;
    //cargo test path00 -- --nocapture
    #[test]
    fn path00() -> Result<()> {
        find_target_path(
            vec!["ヘルガルダ"],
            vec!["タマコッコ", "ミルカルビ"],
            "フェスキー",
        )?;
        Ok(())
    }

    #[test]
    fn path01() -> Result<()> {
        find_target_path(vec!["エレパンダ"], vec!["ペコドン"], "ボルゼクス")?;
        Ok(())
    }
}
