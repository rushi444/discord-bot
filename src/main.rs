use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
	fn reaction_add(&self, ctx: Context, reaction: Reaction) {
		if let Err(why) = reaction.channel_id.say(
			&ctx.http,
			format!("{} left a reaction", reaction.user(&ctx).unwrap().name),
		) {
			println!("Error reacting to a reaction: {:?}", why)
		}
	}

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
	let token = include_str!("../token");

	let mut client = Client::new(&token, Handler).expect("Error creating client");

	if let Err(msg) = client.start() {
		println!("Error {:?}", msg)
	}
}
