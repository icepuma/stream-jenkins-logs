use crate::errors::Errors;
use clap::{crate_authors, crate_name, crate_version, App, Arg};
use std::io::{self, Write};
use std::time::Duration;
use url::Url;

fn get_text_size(header: &reqwest::header::HeaderValue) -> Result<u64, Errors> {
  header.to_str()?.parse().map_err(Errors::from)
}

fn more_data_is_available(header: &reqwest::header::HeaderValue) -> bool {
  match header.to_str() {
    Ok(value) => value != "",
    Err(_) => false,
  }
}

fn get_poll_interval(interval: &str) -> Result<u64, Errors> {
  interval.parse().map_err(Errors::from)
}

fn app<'a>() -> App<'a, 'a> {
  App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .arg(
      Arg::with_name("base-url")
        .help("Jenkins URL: e.g. https://jenkins.example.com or https://user:password@jenkins.example.com")
        .env("BASE")
        .required(true),
    )
    .arg(Arg::with_name("job").help("Job").env("JOB").required(true))
    .arg(
      Arg::with_name("build")
        .help("Build")
        .env("BUILD")
        .default_value("lastBuild"),
    )
    .arg(
      Arg::with_name("poll-interval")
        .help("Poll interval: in ms")
        .env("POLL_INTERVAL")
        .default_value("1000"),
    )
}

fn main() -> Result<(), Errors> {
  let matches = app().get_matches();

  let base_url = Url::parse(matches.value_of("base-url").unwrap())?;
  let job = matches.value_of("job").unwrap();
  let build_path = job.replace("/", "/job/");
  let build = matches.value_of("build").unwrap();

  let mut start: u64 = 0;
  let client = reqwest::Client::new();


  let poll_interval = matches.value_of("poll-interval").unwrap();
  let interval_in_ms = get_poll_interval(poll_interval)?;

  loop {
    let url = format!(
      "{}/job/{}/{}/logText/progressiveText?start={}",
      base_url, build_path, build, start
    );

    let mut request_builder = client.get(&url);

    if base_url.username() != "" && base_url.password().unwrap_or_default() != "" {
      request_builder = request_builder.basic_auth(base_url.username(), base_url.password())
    }

    let mut response = request_builder.send()?;
    let body = response.text()?;

    io::stdout().write_all(body.as_bytes())?;

    if let Some(text_size) = response.headers().get("x-text-size") {
      start = get_text_size(text_size)?;
    } else {
      break;
    }

    if let Some(more_data) = response.headers().get("x-more-data") {
      if !more_data_is_available(more_data) {
        break;
      }
    } else {
      break;
    }


    std::thread::sleep(Duration::from_millis(interval_in_ms));
  }

  Ok(())
}

mod errors;
