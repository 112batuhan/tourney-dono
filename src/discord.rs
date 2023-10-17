use std::env;
use std::sync::Arc;

use anyhow::Result;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler, GatewayIntents, TypeMapKey};
use serenity::{async_trait, Client};
use tokio::sync::broadcast::Sender;

use crate::db::DB;
use crate::DonationData;

pub struct DbKey;
impl TypeMapKey for DbKey {
    type Value = Arc<DB>;
}
pub struct AllowedUsersKey;
impl TypeMapKey for AllowedUsersKey {
    type Value = Vec<u64>;
}
pub struct BroadcastSenderKey;
impl TypeMapKey for BroadcastSenderKey {
    type Value = Sender<Option<i64>>;
}

#[hook]
async fn before(ctx: &Context, msg: &Message, _: &str) -> bool {
    let data = ctx.data.read().await;
    let allowed_users = data.get::<AllowedUsersKey>().unwrap();
    allowed_users.contains(&msg.author.id.0)
}

#[command]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let donor = args.single::<String>()?;
    let amount = args.single::<f32>()?;

    {
        let data = ctx.data.read().await;
        let db = data.get::<DbKey>().unwrap();
        let added_donation = db.add_donation(&donor, &amount).await?;
        let broadcast_sender = data.get::<BroadcastSenderKey>().unwrap();
        broadcast_sender.send(Some(added_donation.id)).ok();
    }

    let response_message = format!("Donation added: {} - {} units!", donor, amount);
    msg.channel_id.say(&ctx.http, response_message).await?;

    Ok(())
}
#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let donor = args.single::<i64>()?;

    {
        let data = ctx.data.read().await;
        let db = data.get::<DbKey>().unwrap();
        db.delete_donation(donor).await?
    }

    let response_message = format!("Donation removed: {}", donor);
    msg.channel_id.say(&ctx.http, response_message).await?;

    Ok(())
}
#[command]
pub async fn all(ctx: &Context, msg: &Message) -> CommandResult {
    let donations = {
        let data = ctx.data.read().await;
        let db = data.get::<DbKey>().unwrap();
        db.get_donations().await?
    };

    let data = DonationData::new(&donations, None);

    let display_msg = data
        .individual_donations
        .iter()
        .map(|donation| {
            format!(
                "**ID**:`{}` - **Donor**:`{}` - **Amount**:`{}` - **Date**:`{}`",
                donation.id, donation.donor, donation.amount, donation.donated_at
            )
        })
        .fold(
            format!("Total donation amount (doubled) : {} \n", data.pricepool),
            |msg_string, line| msg_string + &line + "\n",
        );

    msg.channel_id.say(&ctx.http, display_msg).await?;

    Ok(())
}

#[command]
pub async fn celebrate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let donor = args.single::<i64>()?;

    {
        let data = ctx.data.read().await;
        let db = data.get::<DbKey>().unwrap();
        let donation_from_db = db.get_donation_by_id(donor).await?;
        let broadcast_sender = data.get::<BroadcastSenderKey>().unwrap();
        broadcast_sender.send(Some(donation_from_db.id)).ok();
    }

    let response_message = format!("Donation set to be celebrated again: {}", donor);
    msg.channel_id.say(&ctx.http, response_message).await?;

    Ok(())
}

#[group]
#[commands(add, remove, all, celebrate)]
struct Command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn initiate_dc_bot(
    db_instance: Arc<DB>,
    allowed_users: Vec<u64>,
    donation_broadcast_sender: Sender<Option<i64>>,
) -> Result<()> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .before(before)
        .group(&COMMAND_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<DbKey>(db_instance);
        data.insert::<AllowedUsersKey>(allowed_users);
        data.insert::<BroadcastSenderKey>(donation_broadcast_sender);
    }

    client
        .start()
        .await
        .expect("Error while starting the discord bot client.");

    Ok(())
}
