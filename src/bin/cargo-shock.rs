use fastrand::Rng;
use rzap::{api::OpenShockAPI, api_builder::OpenShockAPIBuilder, data_type::ControlType};
use std::{ffi::OsStr, io::IsTerminal, time::Duration, u16};
use tokio::{self, time::sleep};

#[tokio::main]
async fn main() {
    let code = real_main().await.unwrap_or_else(|e| {
        pretty_print(Err(e.to_string()));
        -1
    });
    std::process::exit(code)
}

fn pretty_print(response: Result<String, String>) {
    let stylize = std::io::stderr().is_terminal();
    let unicode = supports_unicode::on(supports_unicode::Stream::Stderr);
    match (response, stylize, unicode) {
        (Ok(resp), true, true) => eprintln!("\x1b[1m ⚡ {resp}\x1b[0m"),
        (Ok(resp), true, false) => eprintln!("\x1b[1mcargo-shock: {resp}\x1b[0m"),
        (Err(resp), true, true) => eprintln!("\x1b[31m ⚡ {resp}\x1b[0m"),
        (Err(resp), true, false) => eprintln!("\x1b[31mcargo-shock: {resp}\x1b[0m"),
        (Ok(resp) | Err(resp), false, _) => eprintln!("cargo-shock: {resp}"),
    }
}

fn parse_pattern(
    pattern: &str,
) -> Result<Vec<(u8, Duration, Duration)>, Box<dyn std::error::Error>> {
    pattern
        .split('/')
        .map(|x| {
            let parts: Vec<&str> = x.split_whitespace().take(3).collect();
            let (intensity, duration, delay) = (parts.get(0), parts.get(1), parts.get(2));
            let intensity = intensity.unwrap_or(&"").parse()?;
            let duration = duration
                .unwrap_or(&"0.3s")
                .trim_end_matches('s')
                .parse()
                .map(Duration::from_secs_f64)?;
            let delay = delay
                .unwrap_or(&"0s")
                .trim_end_matches('s')
                .parse()
                .map(Duration::from_secs_f64)?;
            Ok((intensity, duration, delay))
        })
        .collect()
}

async fn build_openshock_api(token: String, api_url: Option<String>) -> OpenShockAPI {
    let mut api_builder = OpenShockAPIBuilder::new()
        .with_app("cargo-shock".to_string(), None)
        .with_default_api_token(token);
    if let Some(api_url) = api_url {
        api_builder = api_builder.with_base_url(api_url);
    }
    return api_builder.build().unwrap();
}

async fn trigger_random_shock(token: String, id: String, pattern: String, api_url: Option<String>) {
    let api = build_openshock_api(token, api_url).await;
    let patterns = parse_pattern(&pattern);
    if let Err(_) = patterns {
        pretty_print(Err("Error: Provided pattern is not valid".to_string()));
        return;
    }
    let patterns = patterns.unwrap();
    let rng = Rng::new();
    let (intensity, duration, delay) = &patterns[rng.usize(..patterns.len())];

    sleep(delay.to_owned()).await;

    let _ = api
        .post_control(
            id,
            ControlType::Shock,
            intensity.to_owned(),
            u16::try_from(duration.as_millis()).unwrap(),
            None,
        )
        .await;

    pretty_print(Ok(format!(
        "Shocked you with intensity {} for {}ms :P",
        intensity,
        duration.as_millis()
    )));
}

// code stolen from cargo-mommy, thanks Gankra
async fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    let token = std::env::var("CARGO_SHOCK_TOKEN").ok();
    let shocker_id = std::env::var("CARGO_SHOCK_ID").ok();
    let pattern = std::env::var("CARGO_SHOCK_PATTERN").ok();
    let address = std::env::var("CARGO_SHOCK_ADDR").ok();

    if token.is_none() {
        pretty_print(Err(
            "Warning: The required environment variable \"CARGO_SHOCK_TOKEN\" is not set. cargo-shock will do nothing.".to_string()
        ))
    }
    if shocker_id.is_none() {
        pretty_print(Err(
            "Warning: The required environment variable \"CARGO_SHOCK_ID\" is not set. cargo-shock will do nothing.".to_string()
        ))
    }
    if pattern.is_none() {
        pretty_print(Err(
            "Warning: The required environment variable \"CARGO_SHOCK_PATTERN\" is not set. cargo-shock will do nothing.".to_string()
        ))
    }

    let cargo_var = std::env::var_os("CARGO");
    let cargo = cargo_var.as_deref().unwrap_or(OsStr::new("cargo"));
    let mut arg_iter = std::env::args_os();
    let _cargo = arg_iter.next();
    let _cmd = arg_iter.next();

    let status = std::process::Command::new(cargo).args(arg_iter).status()?;
    if !status.success() {
        match (token, shocker_id, pattern) {
            (Some(token), Some(shocker_id), Some(pattern)) => {
                trigger_random_shock(token, shocker_id, pattern, address).await;
            }
            (_, _, _) => {}
        }
    }
    Ok(status.code().unwrap_or(-1))
}
