use crate::{error, AoCSolution};
use pest::{self, Parser};
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "aoc2017/day07.pest"]
struct Day07Parser;

#[derive(Debug)]
struct Node {
    id: String,
    weight: u32,
    children: Option<Vec<String>>,
}

/// Compute the solution to day 7 of AoC 2017
pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let nodes = parser(&input);

    println!("{}", find_root_node(&nodes));

    Err(error::AoCError::from(error::AoCErrorKind::InvalidDay))
}

fn find_root_node(nodes: &HashMap<String, Node>) -> String {
    let mut names: HashSet<&str> = nodes.keys().map(|key| key.as_str()).collect();

    for node in nodes.values() {
        if let Some(children) = &node.children {
            for child in children.iter() {
                names.remove(child.as_str());
            }
        }
    }

    let root = names.drain().nth(0).unwrap().to_string();

    root
}
fn parser(input: &str) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    let file = Day07Parser::parse(Rule::file, input)
        .unwrap()
        .next()
        .unwrap();

    for record in file.into_inner() {
        if let Rule::node = record.as_rule() {
            let mut id = "".to_string();
            let mut weight = 0;
            let mut children = None;
            for value in record.into_inner() {
                match value.as_rule() {
                    Rule::id => {
                        id = value.as_str().to_string();
                    }
                    Rule::weight => {
                        weight = value.as_str().parse().unwrap();
                    }
                    Rule::children => {
                        children = Some(
                            value
                                .into_inner()
                                .map(|child| child.as_str().to_string())
                                .collect(),
                        )
                    }
                    _ => {}
                }
            }
            nodes.insert(
                id.clone(),
                Node {
                    id: id,
                    weight: weight,
                    children: children,
                },
            );
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    // TODO: Solve test and put the offical answer here
    // use super::*;

    // #[test]
    // fn matches_offical_result() {
    //     let input = include_str!("./input/day07");

    //     let config = Config {
    //         day: 1,
    //         input: input.to_string(),
    //     };

    //     let _result = run(&config.input).unwrap();
    // }

}
