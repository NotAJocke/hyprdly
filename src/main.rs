use std::{
    env,
    process::{exit, Command},
    time::{Duration, Instant},
};

mod download_builder;
use download_builder::*;
use indicatif::DecimalBytes;

fn main() {
    let args = env::args();
    let args: Vec<String> = args.collect();

    let mut dl = DownloadBuilder::new();

    match args.get(1) {
        Some(arg) if arg == "-s" || arg == "--simulate" => dl.simulate(true),
        _ => dl.simulate(false),
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
        .prompt();

    let Ok(urls) = urls else {
        eprintln!("\nNo urls provided\n");
        exit(1);
    };

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
            let parsed = parse_output(s);
            let total_bytes = total_download_bytes(&parsed);
            let human_bytes = DecimalBytes(total_bytes);
            let human_time = human_duration(end);

            println!("Total size: {human_bytes}");
            println!("Time spent: {human_time}");
            println!("Downloaded {} videos", parsed.len());
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

    let out = unsafe { String::from_utf8_unchecked(bytes) };
    let out = out.to_string();

    if cmd_output.status.success() {
        Ok(out)
    } else {
        Err(out)
    }
}

pub fn parse_output(out: String) -> Vec<Output> {
    let data: Vec<String> = out.lines().map(str::trim).map(String::from).collect();
    let amount = data.len() / 4;

    let mut outputs: Vec<Output> = Vec::with_capacity(amount);

    for i in 0..amount {
        outputs.push(Output {
            id: data[i * 4].to_owned(),
            title: data[i * 4 + 1].to_owned(),
            bytes: data[i * 4 + 2].to_owned(),
            duration: data[i * 4 + 3].to_owned(),
        });
    }

    outputs
}

pub fn total_download_bytes(outputs: &[Output]) -> u64 {
    outputs
        .iter()
        .map(|e| &e.bytes)
        .map(|e| e.parse::<u64>().unwrap())
        .sum()
}

pub fn human_duration(duration: Duration) -> String {
    if duration.as_micros() < 1000 {
        format!("{}μs", duration.as_micros())
    } else if duration.as_millis() < 1000 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}s", duration.as_secs())
    }
}

#[derive(Debug)]
pub struct Output {
    pub id: String,
    pub title: String,
    pub bytes: String,
    pub duration: String,
}
