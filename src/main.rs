use poise::serenity_prelude::{
    self as serenity,
    colours::roles::{DARK_PURPLE, DARK_RED},
};
use std::{env, fmt::format, sync::Arc};
use tokio::sync::Semaphore;

struct Data {
    calculator: Arc<Calculator>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn calc(
    ctx: Context<'_>,
    #[description = "Calculation query"] query: String,
) -> Result<(), Error> {
    println!("got a query: {}", query);
    if query.contains("load") {
        ctx.send(poise::CreateReply::default().embed(
            serenity::CreateEmbed::new().colour(DARK_RED).fields(vec![
                ("Query", format!("```{}```", query), false),
                ("Error", "`load` is not allowed".to_string(), false),
            ]),
        ))
        .await?;
        return Ok(());
    }
    let result = ctx.data().calculator.calculate(query.clone()).await;
    println!("the result is: {}", result);
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .colour(DARK_PURPLE)
                .fields(vec![
                    ("Query", format!("```{}```", query), false),
                    ("Result", format!("```{}```", result), false),
                ]),
        ),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("starting disqalculate");
    let calculator = Calculator::create_calculator();
    println!("created calculator instance");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![calc()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    calculator: Arc::new(calculator),
                })
            })
        })
        .build();
    println!("created framework");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    println!("Got a client");
    env::set_var("DISCORD_TOKEN", "HaHaNO");
    println!("reset discord token");

    client
        .expect("client couldn't be built")
        .start()
        .await
        .unwrap();
    println!("Client returned, exiting")
}

struct Calculator {
    // libqcalc isn't thread safe as far as I know
    semaphore: Semaphore,
}

impl Calculator {
    pub(crate) fn create_calculator() -> Self {
        ffi::init_calculator();
        Calculator {
            semaphore: Semaphore::new(1),
        }
    }

    pub(crate) async fn calculate(&self, input: String) -> String {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .expect("semaphore was poisoned");
        ffi::do_calculation(input)
    }
}

// the bridge to the C++ world, uses a helper function in [disqalc.cc](./disqalc.cc) to do
// calculations correctly, use `Calculator` instead of these directly
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("disqalculate/include/disqalc.h");

        fn init_calculator();

        fn do_calculation(input: String) -> String;
    }
}
