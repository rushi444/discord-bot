extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;

const TOKEN:: &str = "";

struct Handler;

impl EventHandler for Handler {}

fn main(){
    let mut client = Client::new(&TOKEN, Handler)
                            .expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error {:?}", msg)
    }
}
