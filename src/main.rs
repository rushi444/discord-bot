use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

const TOKEN: &str = "NzU3NDQ2Nzc3MTk1NzI0OTMx.X2ghSA.yVIMJr3EzsZLq6IZAiAwWETwsDo";

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
                println!("Error giving message: {:?}", why)
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

fn main() {
    let mut client = Client::new(&TOKEN, Handler).expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error {:?}", msg)
    }
}
