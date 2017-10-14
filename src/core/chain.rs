use graph::Graph;

use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

use std::fmt::{self, Display};

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