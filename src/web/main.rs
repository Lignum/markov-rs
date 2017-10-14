#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markov_core;
extern crate rand;

use rocket::State;

use markov_core::chain;
use markov_core::graph::Graph;

struct Context {
    chain: Graph<chain::ChainNode, f64>
}

#[get("/")]
fn index(ctx: State<Context>) -> String {
    let mut rng = rand::thread_rng();
    chain::generate_sentence(&mut rng, &ctx.chain)
}

fn main() {
    let chain = markov_core::chain::deserialise_chain("test.mkv").expect("Failed to deserialise markov chain!");

    let ctx = Context { chain };

    rocket::ignite()
        .manage(ctx)
        .mount("/", routes![index]).launch();
}