use demoji_rs::remove_emoji;
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::cmp;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const ANON_USER: &str = "Anonymous User";

#[derive(Deserialize, Debug)]
pub struct TodayEntry {
    pub name: Option<String>,
    pub score: i32,
    pub star1: Option<String>,
    pub star2: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct TotalEntry {
    pub name: Option<String>,
    pub score: i32,
    pub stars: [i32; 25],
}

#[derive(Deserialize, Debug)]
pub struct AochData {
    pub assignment: String,
    pub today: Vec<TodayEntry>,
    pub total: Vec<TotalEntry>,
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

async fn get_leaderboard_data() -> AochData {
    let response_data = reqwest::get("https://aoch.wisv.ch/data")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    response_data
}

/// Displays the overall leaderboard of this year
#[poise::command(slash_command, prefix_command)]
async fn leaderboard_total(ctx: Context<'_>) -> Result<(), Error> {
    let aoch_data = get_leaderboard_data().await;

    let start_width: usize = aoch_data
        .total
        .iter()
        .map(|x| {
            x.name
                .clone()
                .unwrap_or_else(|| ANON_USER.to_string())
                .len()
        })
        .max()
        .unwrap();
    let max_stars: usize = aoch_data
        .total
        .iter()
        .map(|x| x.stars.clone().iter().sum::<i32>())
        .max()
        .unwrap() as usize;
    let row_size: usize = start_width + 11 + max_stars;
    let row_count = 1900 / row_size;

    let mut table_rows = vec![];
    table_rows.push(format!(
        "{1:0$} | {2:5} | {3:5}",
        start_width, "Name", "Score", "Stars"
    ));
    table_rows.push(format!("{:-^row_size$}", ""));

    table_rows.append(
        aoch_data
            .total
            .into_iter()
            .map(|x| {
                let name: String = remove_emoji(&x.name.unwrap_or_else(|| ANON_USER.to_string()));

                let star_count = x.stars.iter().sum();
                let mut stars: String = String::new();
                for _ in 0..star_count {
                    stars += "*";
                }

                format!("{name:0$} | {1:5} | {stars} ", start_width, x.score)
            })
            .collect::<Vec<String>>()
            .as_mut(),
    );

    let mut stylized_answer = table_rows[0..cmp::min(row_count, table_rows.len())].join("\n");
    stylized_answer = format!("```\n{}```", truncate(&*stylized_answer, 1990));

    ctx.say(stylized_answer).await?;
    Ok(())
}

/// Displays the leaderboard of today.
#[poise::command(slash_command, prefix_command)]
async fn leaderboard_today(ctx: Context<'_>) -> Result<(), Error> {
    let aoch_data = get_leaderboard_data().await;

    let start_width = aoch_data
        .today
        .iter()
        .map(|x| {
            x.name
                .clone()
                .unwrap_or_else(|| ANON_USER.to_string())
                .len()
        })
        .max()
        .unwrap();
    let row_size: usize = start_width + 24;
    let row_count = 1900 / row_size;

    let mut table_rows = vec![];
    table_rows.push(format!(
        "{4:5} | {1:0$} | {2:5} | {3:5}",
        start_width, "Name", "Stars", "Score", "Rank"
    ));
    table_rows.push(format!("{:-^row_size$}", ""));

    table_rows.append(
        aoch_data
            .today
            .into_iter()
            .map(|x| {
                let name: String = remove_emoji(&x.name.unwrap_or_else(|| ANON_USER.to_string()));

                let mut stars: String = String::new();
                if x.star1.is_some() {
                    stars += "*";
                }
                if x.star2.is_some() {
                    stars += "*";
                }

                format!(
                    "{2:>4}) | {name:0$} | {stars:<5} | {1:>5}",
                    start_width, x.score, x.rank
                )
            })
            .collect::<Vec<String>>()
            .as_mut(),
    );

    let mut stylized_answer = table_rows[0..cmp::min(row_count, table_rows.len())].join("\n");
    stylized_answer = format!("```\n{}```", truncate(&*stylized_answer, 1990));
    println!("{}", stylized_answer);
    ctx.say(stylized_answer).await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN environment variable");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![leaderboard_today(), leaderboard_total()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
