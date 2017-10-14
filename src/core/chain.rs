use graph::Graph;

use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

use std::fmt::{self, Display};

use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

use bincode;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ChainNode {
    Start,
    Word(String),
    End,
    NoData
}

impl Display for ChainNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainNode::Start => write!(f, ""),
            ChainNode::Word(ref str) => write!(f, "{}", str),
            ChainNode::End | ChainNode::NoData => write!(f, ".")
        }
    }
}

pub fn serialise_chain(chain: Graph<ChainNode, f64>, file: &str) -> Result<(), Box<Error>> {
    let serialised = bincode::serialize(&chain, bincode::Infinite)?;
    let mut file = File::create(file)?;
    file.write_all(&serialised)?;
    Ok(())
}

pub fn deserialise_chain(file: &str) -> Result<Graph<ChainNode, f64>, Box<Error>> {
    let buffer: Vec<u8> = {
        let mut buf = Vec::new();
        let mut file = File::open(file)?;
        file.read_to_end(&mut buf)?;
        buf
    };

    let deserialised = bincode::deserialize(buffer.as_ref())?;
    Ok(deserialised)
}

pub fn generate_sentence<R: Rng>(rng: &mut R, chain: &Graph<ChainNode, f64>) -> String {
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