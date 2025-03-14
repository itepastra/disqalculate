use poise::serenity_prelude::{
    self as serenity,
    colours::roles::{DARK_PURPLE, DARK_RED},
};
use std::{env, sync::Arc};
use tokio::sync::Semaphore;

struct Data {
    calculator: Arc<Calculator>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Discord command for calculations
///
/// calculate a passed parameter by first checking if it contains illegal instructions and then
/// passing it into [libqalculate](https://github.com/Qalculate/libqalculate)
///
/// # example
///
/// in discord with the bot in the server:
/// ```
/// /calc query:hex(55) + bin(1001010) to hex
/// ```
/// returns the value in hex: `9F`
#[poise::command(slash_command, prefix_command)]
async fn calc(
    ctx: Context<'_>,
    #[description = "Calculation query"] query: String,
) -> Result<(), Error> {
    println!("got a query: {}", query);
    // load may leak sensitive information, so we disallow it's use
    // if query.to_lowercase().contains("load") {
    //     ctx.send(poise::CreateReply::default().embed(
    //         serenity::CreateEmbed::new().colour(DARK_RED).fields(vec![
    //             ("Query", format!("```{}```", query), false),
    //             ("Error", "`load` is not allowed".to_string(), false),
    //         ]),
    //     ))
    //     .await?;
    //     return Ok(());
    // }
    // if query.to_lowercase().contains("function") {
    //     ctx.send(poise::CreateReply::default().embed(
    //         serenity::CreateEmbed::new().colour(DARK_RED).fields(vec![
    //             ("Query", format!("```{}```", query), false),
    //             ("Error", "`function` is not allowed".to_string(), false),
    //         ]),
    //     ))
    //     .await?;
    //     return Ok(());
    // }
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

/// Thread safe version of the libqalculate calculator
///
/// Uses a semaphore to make sure only a single thread can ever execute on the global CALCULATOR
/// object in libqalculate
struct Calculator {
    // libqcalc isn't thread safe as far as I know
    semaphore: Semaphore,
}

impl Calculator {
    /// Initialises the calculator object
    ///
    /// there should only ever be one at a time, otherwise I cannot guarantee it'll keep working
    pub(crate) fn create_calculator() -> Self {
        ffi::init_calculator();
        Calculator {
            semaphore: Semaphore::new(1),
        }
    }

    /// Calculate the result from an input string
    ///
    /// Uses the semaphore in [`Calculator`] to make sure only one calculation happens at once.
    pub(crate) async fn calculate(&self, input: String) -> String {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .expect("semaphore was poisoned");
        ffi::do_calculation(input)
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("disqalculate/include/disqalc.h");

        fn init_calculator();

        fn do_calculation(input: String) -> String;
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::Semaphore;

    use crate::Calculator;
    static PERMIT: Semaphore = Semaphore::const_new(1);

    async fn get_calculator() -> Calculator {
        let _permit = PERMIT.acquire().await;
        Calculator::create_calculator()
    }

    async fn test_calculation(calc: &Calculator, input: &str, correct: &str) {
        let result = calc.calculate(input.to_string()).await;
        assert_eq!(result, correct)
    }

    macro_rules! make_calculation_tests {
        ( $( ($name:ident, $query:literal, $correct:literal), )+ ) => {
            $(
                paste::item! {
                    #[tokio::test]
                    async fn [< test_calculate_ $name >] () {
                        let calc = get_calculator().await;
                        test_calculation(&calc, $query, $correct).await;
                    }
                }
            )+
        };
    }

    make_calculation_tests! {
        (add, "1+1", "2"),
        (sub, "1-1", "0"),
        (mul, "7*8", "56"),
        (div_int, "8/2", "4"),
        (div_floor, "7//2", "3"),
        (convert, "183 cm to m", "1.83 m"),
    }
}
