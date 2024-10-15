use std::process::Command;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_installed() {
        assert!(ffmpeg_installed());
    }
}
