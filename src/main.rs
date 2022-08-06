use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use serenity::model::id::{ChannelId};
use serenity::cache::{Cache};



struct Handler;

fn set_channel_id(id: u64) -> serenity::model::id::ChannelId{
    return serenity::model::id::ChannelId(id);
}
#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        
        println!("{} is connected!", ready.user.name);
    }

    #[cfg(feature = "cache")]
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("cache ready");
    }

    async fn voice_state_update(&self, ctx: Context , _old: Option<VoiceState>, _new: VoiceState){

        println!("{:?}",_new);
        
        if let Some(joined_member) = _new.member {
           
            println!("{}が通話を始めたよ",  joined_member.user.name);
            let _channel_id: u64 = 1000298417001595010;

            let _text_channel :ChannelId = set_channel_id(_channel_id);
            let _message: String = joined_member.user.name + "が通話を始めたよ";
            if let Err(why) =  _text_channel.say(&ctx.http, _message).await {
                println!("Client error: {:?}", why);
            }
        }
        
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_VOICE_STATES;
 
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}