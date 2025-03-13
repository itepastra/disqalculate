use poise::serenity_prelude::{self as serenity, colours::roles::DARK_PURPLE, CreateEmbed};
use std::sync::Arc;
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
    let result = ctx.data().calculator.calculate(query.clone()).await;
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
    let calculator = Calculator::create_calculator();
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

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client
        .expect("client couldn't be built")
        .start()
        .await
        .unwrap();
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
