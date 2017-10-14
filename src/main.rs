extern crate markov_core;
extern crate rand;

use std::fmt;

use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

use markov_core::graph::Graph;
use markov_core::chain::{deserialise_chain, ChainNode};

fn generate_sentence<R: Rng>(rng: &mut R, chain: Graph<ChainNode, f64>) -> String {
    let start_node = chain.find_node(&ChainNode::Start).expect("Could not find start node!");
    let end_node = chain.find_node(&ChainNode::End).expect("Could not find end node!");

    let mut current_node = start_node;
    let mut sentence = String::new();

    while current_node != end_node {
        let nodes = chain.nodes_from(current_node);

        let mut weighted_nodes: Vec<Weighted<ChainNode>> = nodes.iter()
            .map(|&(i, n)| {
                let w = chain.weight(current_node, i).expect("No edge between current node and next node!");
                Weighted { weight: (w * 1000000f64) as u32, item: n.clone() }
            })
            .collect();

        let wc = WeightedChoice::new(&mut weighted_nodes);
        let node = wc.ind_sample(rng);
        let index = chain.find_node(&node).expect("Couldn't find node again!");

        match node {
            ChainNode::Word(_) => fmt::write(&mut sentence, format_args!("{} ", node)).expect("Failed to write to sentence!"),
            _ => {}
        };

        current_node = index;
    }

    format!("{}.", sentence.trim_right())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let prog_name = &args[0];

    if args.len() < 2 {
        eprintln!("Usage: {} [file]", prog_name);
        return;
    }

    let filename = &args[1];
    let mut rng = rand::thread_rng();

    let chain = match deserialise_chain(filename) {
        Ok(chain) => chain,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    println!("{}", generate_sentence(&mut rng, chain));
}
