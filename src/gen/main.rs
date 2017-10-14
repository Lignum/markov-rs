extern crate markov_core;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

use markov_core::graph::Graph;
use markov_core::chain::{serialise_chain, ChainNode};

lazy_static! {
    static ref SENTENCE_PATTERN: Regex = Regex::new(r"\s*?(?P<content>[A-Z].*?)[!\?\.]+\s*?").expect("Failed to compile regex!!");
    static ref WORD_PATTERN: Regex = Regex::new(r"[\wz']+").expect("Failed to compile regex!");
}

fn normalise_chain(chain: Graph<ChainNode, u64>, start_node: usize) -> Graph<ChainNode, f64> {
    fn normalise(chain: &Graph<ChainNode, u64>, nodes: &mut Vec<ChainNode>, edges: &mut HashMap<(usize, usize), f64>, start: usize) {
        let nodes_from = chain.nodes_from(start);
        if nodes_from.is_empty() {
            return;
        }

        let sum = nodes_from.iter().fold(0, |i, &(j, _)| i + chain.weight(start, j).unwrap()) as f64;

        nodes_from.iter()
            .for_each(|&(i, v)| {
                if !nodes.contains(v) {
                    *nodes.get_mut(i).unwrap() = v.clone();
                    normalise(chain, nodes, edges, i);
                }
            });

        nodes_from.iter()
            .map(|&(i, _)| (i, *chain.weight(start, i).unwrap() as f64 / sum))
            .for_each(|(i, w)| { edges.insert((start, i), w); });
    }

    let mut nodes: Vec<ChainNode> = vec![ChainNode::End; chain.len()];
    let mut edges: HashMap<(usize, usize), f64> = HashMap::new();

    *nodes.get_mut(start_node).unwrap() = ChainNode::Start;

    normalise(&chain, &mut nodes, &mut edges, start_node);

    Graph::from_nodes_and_edges(nodes, edges)
}

fn generate_chain(sentences: Vec<String>) -> Graph<ChainNode, f64> {
    let mut chain: Graph<ChainNode, u64> = Graph::new();
    let start_node = chain.insert_node(ChainNode::Start);
    let end_node = chain.insert_node(ChainNode::End);

    for sentence in sentences.into_iter() {
        let mut previous_node = start_node;

        for word in words(sentence.as_str()).into_iter() {
            let node = if !chain.contains(&ChainNode::Word(word.clone())) {
                chain.insert_node(ChainNode::Word(word.clone()))
            } else {
                // This will definitely be Some(word) because we checked with contains.
                chain.find_node(&ChainNode::Word(word.clone())).unwrap()
            };

            let has_edge = chain.has_edge(previous_node, node);
            if has_edge {
                let weight = chain.weight(previous_node, node).expect("wtf") + 1;
                chain.set_weight(previous_node, node, weight);
            } else {
                chain.insert_edge(previous_node, node, 1);
            }

            previous_node = node;
        }

        let has_edge = chain.has_edge(previous_node, end_node);
        if has_edge {
            let weight = chain.weight(previous_node, end_node).expect("wtf") + 1;
            chain.set_weight(previous_node, end_node, weight);
        } else {
            chain.insert_edge(previous_node, end_node, 1);
        }
    }

    normalise_chain(chain, start_node)
}

fn words(sentence: &str) -> Vec<String> {
    WORD_PATTERN.captures_iter(sentence)
        .map(|caps| caps[0].to_string())
        .collect()
}

fn sentences(text: &str) -> Vec<String> {
    SENTENCE_PATTERN.captures_iter(text)
        .map(|caps| (&caps["content"]).to_string())
        .collect()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let prog_name = &args[0];

    if args.len() < 2 {
        eprintln!("Usage: {} [file] <output file>", prog_name);
        return;
    }

    let input_file = &args[1];
    let input_root = match Path::new(input_file).file_stem().and_then(|s| s.to_str()) {
        Some(root) => root,
        None => "out"
    };

    let default_output_file = format!("{}.mkv", input_root);
    let output_file = args.get(2).unwrap_or(&default_output_file);

    let text = {
        let mut file = File::open(input_file).expect("Couldn't open file test.txt!!");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read test.txt!!");
        content
    };

    let chain = generate_chain(sentences(text.as_str()));

    match serialise_chain(chain, output_file) {
        Ok(()) => println!("Successfully generated {}", output_file),
        Err(err) => eprintln!("Error occurred: {}", err)
    }
}