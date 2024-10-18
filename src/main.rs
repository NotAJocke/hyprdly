use std::{
    env,
    process::exit,
    time::{Duration, Instant},
};

use download_builder::*;
use indicatif::DecimalBytes;

mod download;
mod download_builder;
mod utils;

fn main() {
    if !utils::ytdlp_installed() {
        eprintln!("\nYou need to install yt-dlp (https://github.com/yt-dlp/yt-dlp)\n");
        exit(1);
    }
    if !utils::ffmpeg_installed() {
        eprintln!("\nYou need to install ffmpeg (https://www.ffmpeg.org)\n");
        exit(1);
    }

    let args = env::args();
    let args: Vec<String> = args.collect();

    let mut dl = DownloadBuilder::new();

    match args.get(1) {
        Some(arg) if arg == "-s" || arg == "--simulate" => dl.simulate(true),
        _ => dl.simulate(false),
    };

    prompt_informations(&mut dl);

    let dl_args = dl.build();

    if cfg!(debug_assertions) {
        dbg!(&dl_args);
    }

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    let start = Instant::now();
    let dl_result = download::make(dl_args);
    let end = start.elapsed();

    match dl_result {
        Ok(s) => {
            spinner.finish();
            let outputs = download::parse_output(s);
            let total_bytes = utils::total_download_bytes(&outputs);
            let human_bytes = DecimalBytes(total_bytes);
            let human_time = utils::human_duration(end);

            println!("Total size: {human_bytes}");
            println!("Time spent: {human_time}");
            println!("Downloaded {} videos", outputs.len());
        }
        Err(s) => {
            spinner.finish_with_message(format!("Error while download: {s}"));
            exit(1);
        }
    }
}

fn prompt_informations(dl: &mut DownloadBuilder) {
    let dl_type = inquire::Select::new("What do you want to download ?", DownloadType::options())
        .prompt()
        .ok()
        .unwrap_or_default();
    let dl_type = DownloadType::from_option(&dl_type);

    if !matches!(&dl_type, DownloadType::Audio) {
        let dl_quality = inquire::Select::new("In which quality ?", DownloadQuality::options())
            .prompt()
            .ok()
            .unwrap_or_default();
        let dl_quality = DownloadQuality::from_option(&dl_quality);
        dl.quality(dl_quality);
    };

    dl.download_type(dl_type);

    let dl_ext = inquire::Text::new("In which extension ? (enter for default)")
        .prompt()
        .ok()
        .unwrap_or_default();
    if !dl_ext.is_empty() {
        dl.extension(dl_ext);
    }

    let urls = inquire::Editor::new("Input your url(s)")
        .with_predefined_text("# Each url separated by a new line.")
        .prompt();

    let Ok(urls) = urls else {
        eprintln!("\nNo urls provided\n");
        exit(1);
    };

    dl.urls(&urls);
}
