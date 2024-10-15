use std::{process::Command, time::Duration};

use crate::download;

pub fn ffmpeg_installed() -> bool {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Failed to exec process.");

    output.status.success()
}

pub fn ytdlp_installed() -> bool {
    let output = Command::new("yt-dlp")
        .arg("--version")
        .output()
        .expect("Failed to exec process.");

    output.status.success()
}

pub fn total_download_bytes(outputs: &[download::Output]) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_installed() {
        assert!(ffmpeg_installed());
    }
}
