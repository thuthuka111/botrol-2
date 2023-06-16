use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::ChannelId},
    prelude::*,
};
use std::env;

const HELP_MESSAGE: &str = "
i am the new BOTROL

new and COMPILED, NOT INTEREPRETED!!

things are only being tested in <#829810811278589973>
";

const SENDING_CONFFESTION_MESSAGE: &str = "Sorry i dont have any quips like my predecessor

find ur appaling! conffession on <#816619394360410143>";

const HELP_COMMAND: &str = "help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.starts_with("confess ") {
            let roc_channel = ChannelId(816619394360410143);

            let confession = &msg.content[8..];

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, SENDING_CONFFESTION_MESSAGE)
                .await
            {
                println!("Error sending message: {:?}", why);
            }

            let confession = format!(
                "A confession has been made:
            ```{}```",
                confession
            );

            if let Err(why) = roc_channel.say(&ctx.http, confession).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("Need a discord token in the environment");

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
