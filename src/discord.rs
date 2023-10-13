use anyhow::Result;

use std::{env, sync::Arc};

use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group, hook},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::gateway::Ready,
    model::prelude::Message,
    prelude::{Context, EventHandler, GatewayIntents, TypeMapKey},
    Client,
};

use crate::{db::DB, TemplateData};

pub struct DbKey;
impl TypeMapKey for DbKey {
    type Value = Arc<DB>;
}
pub struct AllowedUsersKey;
impl TypeMapKey for AllowedUsersKey {
    type Value = Vec<u64>;
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
        let db = data.get::<DbKey>().unwrap().clone();
        db.add_donation(&donor, &amount).await?;
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
        let db = data.get::<DbKey>().unwrap().clone();
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

    let data = TemplateData::new(&donations);

    let display_msg = data
        .latest
        .iter()
        .map(|donation| {
            format!(
                "**ID**:`{}` - **Donor**:`{}` - **Amount**:`{}` - **Celebrated**:`{}` - **Date**:`{}`",
                donation.id,
                donation.donor,
                donation.amount,
                donation.celebrated,
                donation.donated_at
            )
        })
        .fold(
            format!("Total donation amount (doubled) : {} \n", data.total),
            |msg_string, line| msg_string + &line + "\n",
        );

    msg.channel_id.say(&ctx.http, display_msg).await?;

    Ok(())
}

#[group]
#[commands(add, remove, all)]
struct Command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn initiate_dc_bot(db_instance: Arc<DB>, allowed_users: Vec<u64>) -> Result<()> {
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
    }

    client
        .start()
        .await
        .expect("Error while starting the discord bot client.");

    Ok(())
}
