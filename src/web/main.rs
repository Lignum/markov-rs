#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markov_core;
extern crate rand;

use rocket::State;

use markov_core::chain;
use markov_core::graph::Graph;

#[get("/")]
fn index(chain: State<Graph<chain::ChainNode, f64>>) -> String {
    let mut rng = rand::thread_rng();
    chain::generate_sentence(&mut rng, &chain)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let prog_name = &args[0];

    if args.len() < 2 {
        eprintln!("Usage: {} [.mkv file]", prog_name);
        return;
    }

    let filename = &args[1];
    let chain = markov_core::chain::deserialise_chain(filename).expect("Failed to deserialise markov chain!");

    rocket::ignite()
        .manage(chain)
        .mount("/", routes![index]).launch();
}