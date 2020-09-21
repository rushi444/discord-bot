use serenity::prelude::*;
use serenity::model::gateway::Ready;

const TOKEN: &str = "NzU3NDQ2Nzc3MTk1NzI0OTMx.X2ghSA.yVIMJr3EzsZLq6IZAiAwWETwsDo";

struct Handler;

impl EventHandler for Handler {}

fn main(){
    let mut client = Client::new(&TOKEN, Handler)
                        .expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error {:?}", msg)
    }
}
