use std::{
    env,
    process::{exit, Command},
    time::{Duration, Instant},
};

mod download_builder;
use download_builder::*;

fn main() {
    let args = env::args();
    let args: Vec<String> = args.collect();

    let mut dl = DownloadBuilder::new();

    match args.get(1) {
        Some(arg) if arg == "-ns" || arg == "--no-simulate" => dl.simulate(false),
        _ => dl.simulate(true),
    };

    let dl_type = inquire::Select::new("What do you want to download ?", DownloadType::options())
        .prompt()
        .ok()
        .unwrap_or_default();
    let dl_type = DownloadType::from_option(&dl_type);
    dl.download_type(dl_type);

    let dl_quality = inquire::Select::new("In which quality ?", DownloadQuality::options())
        .prompt()
        .ok()
        .unwrap_or_default();
    let dl_quality = DownloadQuality::from_option(&dl_quality);
    dl.quality(dl_quality);

    let urls = inquire::Editor::new("Input your url(s)")
        .with_predefined_text("# Each url separated by a new line.")
        .prompt()
        .unwrap();
    dl.urls(&urls);

    let dl_args = dl.build();

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    let start = Instant::now();
    let dl_result = download(dl_args);
    let end = start.elapsed();

    match dl_result {
        Ok(s) => {
            spinner.finish();
            let parsed = parse_output(&s);
        }
        Err(s) => {
            spinner.finish_with_message(format!("Error while download: {s}"));
            exit(1);
        }
    }
}

pub fn download(args: Vec<String>) -> Result<String, String> {
    let cmd_output = Command::new("yt-dlp")
        .args(args)
        .output()
        .expect("Failed to exec process.");

    let bytes: Vec<u8> = if cmd_output.status.success() {
        cmd_output.stdout
    } else {
        cmd_output.stderr
    };

    let out = String::from_utf8_lossy(&bytes);
    let out = out.to_string();

    if cmd_output.status.success() {
        Ok(out)
    } else {
        Err(out)
    }
}

pub fn parse_output(out: &str) -> Vec<Output> {
    let data: Vec<&str> = out.lines().map(str::trim).collect();
    let amount = data.len() / 4;

    let mut outputs: Vec<Output> = Vec::with_capacity(amount);

    for i in 0..amount {
        outputs.push(Output {
            id: data[i * 4],
            title: data[i * 4 + 1],
            bytes: data[i * 4 + 2],
            duration: data[i * 4 + 3],
        });
    }

    outputs
}

#[derive(Debug)]
pub struct Output<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub bytes: &'a str,
    pub duration: &'a str,
}
