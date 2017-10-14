#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markov_core;
extern crate rand;

use std::collections::HashMap;
use std::path::Path;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use rocket::State;
use rocket::response::{Response, Result};
use rocket::http::Status;

use markov_core::chain;
use markov_core::graph::Graph;

struct Context {
    chain_dir: String,
    chains: Arc<Mutex<HashMap<String, Graph<chain::ChainNode, f64>>>>
}

#[get("/<name>")]
fn index(ctx: State<Context>, name: String) -> Result {
    let mut rng = rand::thread_rng();

    let mut chains = ctx.chains.lock().expect("Failed to lock chains!");
    {
        let ch = chains.get(&name);

        if let Some(chain) = ch {
            let sentence = chain::generate_sentence(&mut rng, &chain);
            return Response::build().sized_body(Cursor::new(sentence)).ok();
        }
    }

    let chain_path = Path::new(&ctx.chain_dir).join(format!("messages-{}.mkv", name));
    let chain_path_str = chain_path.to_str().expect("Invalid chain path!");

    match chain::deserialise_chain(chain_path_str) {
        Ok(chain) => {
            let sentence = chain::generate_sentence(&mut rng, &chain);
            chains.insert(name, chain);
            Response::build().sized_body(Cursor::new(sentence)).ok()
        },
        Err(err) => {
            eprintln!("Could not load chain {}: {} ({})", name, err.description(), err);
            Response::build().status(Status::InternalServerError).ok()
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let prog_name = &args[0];

    if args.len() < 2 {
        eprintln!("Usage: {} [chain file directory]", prog_name);
        eprintln!("Directory must be a directory containing files named messages-[username].mkv!");
        return;
    }

    let dir_name = &args[1];
    {
        let dir_path = Path::new(dir_name);

        if !dir_path.exists() || !dir_path.is_dir() {
            eprintln!("\"{}\" is not a directory!", dir_name);
            return;
        }
    }

    let ctx = Context {
        chain_dir: dir_name.to_string(),
        chains: Arc::new(Mutex::new(HashMap::new()))
    };

    rocket::ignite()
        .manage(ctx)
        .mount("/", routes![index]).launch();
}