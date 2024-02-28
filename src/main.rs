use rand::Rng;
use serde::Deserialize;
use serenity::{
    all::{
        CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, GuildId, Interaction,
        Ready, ResolvedOption, ResolvedValue,
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

            for option in command.data.options() {
                let ResolvedOption {
                    value: ResolvedValue::Integer(i),
                    name,
                    ..
                } = option
                else {
                    continue;
                };

                match name {
                    "width" => width = i,
                    "height" => height = i,
                    "mosquitos" => mosquitos = i,
                    _ => (),
                };
            }

            let quote = get_quote().await;

            let data = CreateInteractionResponseMessage::new()
                .content(format!(
                    "{}\n{}",
                    quote,
                    generate_board(width as usize, height as usize, mosquitos as usize,)
                ))
                .ephemeral(false);

            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            std::env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        if let Err(e) = guild_id
            .set_commands(
                &ctx.http,
                vec![CreateCommand::new("mosquitoswatter")
                    .description("Don't step on a mosquito!")
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
                    )],
            )
            .await
        {
            eprintln!("{:#?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("error");

    println!("https://discord.com/oauth2/authorize?client_id=1212506405240836199&scope=bot");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
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

fn generate_board(cols: usize, rows: usize, mosquitos: usize) -> String {
    let mut output = String::new();

    let rows = rows.min(9).max(3);
    let cols = cols.min(11).max(3);
    let total = rows * cols;

    let num_mosquitos = mosquitos.max(1).min(total - 1);

    let mut mosquitos = vec![];

    while mosquitos.len() < num_mosquitos {
        let num = rand::thread_rng().gen_range(0..total);

        if mosquitos.contains(&num) {
            continue; // generate a new one
        }

        mosquitos.push(num);
    }

    for y in 0..rows {
        for x in 0..cols {
            if mosquitos.contains(&((y * cols) + x)) {
                output += "||:mosquito:||";
                continue;
            }

            let icon = to_emoji(check_neigbors(&mosquitos, cols, (x, y)));

            output += &format!("||{}||", icon).to_string();
        }
        output += "\n";
    }

    output
}

fn check_neigbors(mosquitos: &[usize], cols: usize, pos: (usize, usize)) -> usize {
    let mut count = 0;
    for y in pos.1.max(1) - 1..=pos.1 + 1 {
        for x in pos.0.max(1) - 1..=(pos.0 + 1).min(cols - 1) {
            let index = &((y * cols) + x);
            if mosquitos.contains(index) {
                count += 1;
            }
        }
    }

    count
}

fn to_emoji(count: usize) -> String {
    match count {
        1 => String::from(":one:"),
        2 => String::from(":two:"),
        3 => String::from(":three:"),
        4 => String::from(":four:"),
        5 => String::from(":five:"),
        6 => String::from(":six:"),
        7 => String::from(":seven:"),
        8 => String::from(":eight:"),
        _ => String::from(":zero:"),
    }
}
