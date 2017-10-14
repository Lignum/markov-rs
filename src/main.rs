extern crate markov_core;
extern crate rand;

use markov_core::chain::{generate_sentence, deserialise_chain};

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

    println!("{}", generate_sentence(&mut rng, &chain));
}
