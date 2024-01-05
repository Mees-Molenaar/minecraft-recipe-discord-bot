use poise::serenity_prelude as serenity;
use std::fs;

use crate::table::print_recipe_table;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod table;

fn load_json(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let recipes: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well formatted.");

    recipes
}

#[poise::command(slash_command)]
async fn recipe(
    ctx: Context<'_>,
    #[description = "Ask for a recipe"] item: String,
) -> Result<(), Error> {
    let recipes = load_json("data/recipes.json");

    let message = match &recipes.get(item.clone().to_lowercase()) {
        Some(value) => {
            let recipe = value.to_string();
            let items: Vec<&str> =
                serde_json::from_str(&recipe).expect("JSON formatting goes wrong.exe");
            print_recipe_table(items)
        }
        None => {
            // TODO: Give suggestions
            format!("Item: {} recept bestaat niet.", { item.clone() })
        }
    };

    ctx.say(message).await?;

    Ok(())
}

// NOTE: Easy to register and deregister commands
// #[poise::command(prefix_command)]
// pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
//     poise::builtins::register_application_commands_buttons(ctx).await?;
//     Ok(())
// }

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .options(poise::FrameworkOptions {
            commands: vec![
                recipe(),
                // register()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .expect("Error creating framework");

    if let Err(why) = framework.start().await {
        println!("Client error: {:?}", why);
    }
}
