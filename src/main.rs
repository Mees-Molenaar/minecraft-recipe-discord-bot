use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

use std::fs;

use crate::table::print_recipe_table;

struct Bot;

mod table;

fn load_json(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let recipes: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well formatted.");

    return recipes
}

#[async_trait]
impl EventHandler for Bot {


    async fn message(&self, ctx: Context, msg: Message) {
       
        let recipes = load_json("data/recipes.json");

        let mut iter = msg.content.split_whitespace();

        let command = iter
        .next()
        .unwrap_or("");

        if command != "!recipe" {
            return;
        }

        let item = iter
        .next()
        .unwrap_or("");

        let recipe_string = &recipes[item].to_string();

        let items: Vec<&str> = serde_json::from_str(&recipe_string).expect("JSON formatting goes wrong.exe");

        println!("{:?}", items);
        println!("{:?}", &items[0..3]);

        // let recipe_message = format!(
        //     "
        //     {:?}  
        //     {:?} 
        //     {:?}", &items[0..3], &items[3..6], &items[6..9]
        // );

        let table = print_recipe_table(items);
        println!("{:?}", table);


        if let Err(e) = msg.channel_id.say(&ctx.http, format!("{}", table)).await {
            error!("Error sending message: {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
