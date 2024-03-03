use minesweeper_lib::*;
use serde::Deserialize;
use serenity::{
    all::{
        CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, Guild, GuildId, Http,
        Interaction, Ready, ResolvedValue, UnavailableGuild,
    },
    async_trait, Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.data.name.as_str() != "mosquitoswatter" {
                return;
            }

            let mut width = 11;
            let mut height = 9;
            let mut mosquitos = 10;
            let mut private = true;
            let mut seed: Option<String> = None;

            for option in command.data.options() {
                let name = option.name;
                match option.value {
                    ResolvedValue::Integer(value) => {
                        match name {
                            "width" => width = value,
                            "height" => height = value,
                            "mosquitos" => mosquitos = value,
                            _ => (),
                        };
                    }
                    ResolvedValue::Boolean(value) => {
                        if name == "private" {
                            private = value
                        }
                    }
                    ResolvedValue::String(value) => {
                        if name == "seed" {
                            seed = Some(value.to_string())
                        }
                    }
                    _ => (),
                }
            }

            let width = width.min(11).max(1);
            let height = height.min(9).max(1);
            let mosquitos = mosquitos.min((width * height) - 1).max(1);
            let seed = seed.unwrap_or(get_quote().await);

            let minesweeper = Minesweeper::random_seed(
                width as usize,
                height as usize,
                mosquitos as usize,
                seed.clone(),
            );

            let board = minesweeper.get_board();
            let mut as_str = String::new();

            for row in board.get_rows() {
                for tile in row.get_tiles() {
                    as_str.push_str(&format!("||{}||", &from_tile(tile)));
                }
                as_str.push('\n');
            }

            let data = CreateInteractionResponseMessage::new()
                .content(format!("{}\n{}", seed, as_str))
                .ephemeral(private);

            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _: Option<bool>) {
        println!("Added to {}", guild.name);

        if let Err(e) = load(&ctx.http, guild.id).await {
            eprintln!("{:#?}", e);
        };
    }

    async fn guild_delete(&self, _ctx: Context, guild: UnavailableGuild, _: Option<Guild>) {
        println!("Removed from {:?}", guild.id);
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn load(http: impl AsRef<Http>, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error>> {
    guild_id
        .set_commands(
            http,
            vec![CreateCommand::new("mosquitoswatter")
                .description("Don't get bit by a mosquito!")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "width",
                        "Width of the board, between 3 and 11 (default: 11)",
                    )
                    .min_int_value(3)
                    .max_int_value(11)
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "height",
                        "Height of the board, between 3 and 9 (default: 9)",
                    )
                    .min_int_value(3)
                    .max_int_value(9)
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "mosquitos",
                        "Amount of mosquitos, between 1 and (width*height-1) (default: 10)",
                    )
                    .min_int_value(1)
                    .max_int_value(99)
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "private",
                        "Reply with a message only you can see (default: true)",
                    )
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "seed",
                        "Random seed used to generate the board (default: None)",
                    )
                    .required(false),
                )],
        )
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // fail silently

    println!("https://discord.com/oauth2/authorize?client_id=1212506405240836199&scope=bot");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::GUILDS)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub author: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author_slug: String,
    pub length: usize,
    pub date_added: String,
    pub date_modified: String,
}

async fn get_quote() -> String {
    match inner_get_quote().await {
        Err(_) => String::from("Good Luck!"),
        Ok(q) => q,
    }
}

async fn inner_get_quote() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.quotable.io/quotes/random";
    let res: Vec<Quote> = reqwest::get(url).await?.json().await?;

    let Some(first_result) = res.first() else {
        return Ok(String::from("Good luck!"));
    };

    Ok(first_result.clone().content)
}

fn from_tile(tile: &Tile) -> String {
    match tile {
        Tile::Mine => String::from(":mosquito:"),
        Tile::One => String::from(":one:"),
        Tile::Two => String::from(":two:"),
        Tile::Three => String::from(":three:"),
        Tile::Four => String::from(":four:"),
        Tile::Five => String::from(":five:"),
        Tile::Six => String::from(":six:"),
        Tile::Seven => String::from(":seven:"),
        Tile::Eight => String::from(":eight:"),
        Tile::Empty => String::from(":zero:"),
    }
}
