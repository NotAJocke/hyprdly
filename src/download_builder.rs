use std::process::exit;

use regex::Regex;

#[derive(Debug)]
pub struct DownloadBuilder {
    simulate: bool,
    download_type: Option<DownloadType>,
    quality: Option<DownloadQuality>,
    urls: Vec<String>,
}

impl DownloadBuilder {
    pub fn new() -> Self {
        Self {
            simulate: true,
            download_type: None,
            quality: None,
            urls: vec![],
        }
    }

    pub fn simulate(&mut self, simulate: bool) -> &mut Self {
        self.simulate = simulate;
        self
    }

    pub fn download_type(&mut self, download_type: DownloadType) -> &mut Self {
        self.download_type = Some(download_type);
        self
    }

    pub fn quality(&mut self, quality: DownloadQuality) -> &mut Self {
        self.quality = Some(quality);
        self
    }

    pub fn urls(&mut self, urls: &str) -> &mut Self {
        let parsed_urls = urls
            .lines()
            .filter(|e| !e.starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n");

        // https://regexper.com/#%5E%28https%3F%3A%5C%2F%5C%2F%5B%5E%5Cs%2F%24.%3F%23%5D.%5B%5E%5Cs%5D*%5Cn%3F%29%2B%24
        let re = Regex::new(r"^(https?://[^\s/$.?#].[^\s]*\n?)+$").unwrap();
        let validated_urls = re.is_match(&parsed_urls);

        if !validated_urls {
            eprintln!("\nUrls aren't properly formatted\n");
            exit(1);
        }

        self.urls = parsed_urls.lines().map(String::from).collect();

        self
    }

    pub fn build(self) -> Vec<String> {
        if self.download_type.is_none() {
            eprintln!("\nDownload type isn't set\n");
            exit(1);
        }

        if self.quality.is_none() {
            eprintln!("\nQuality isn't set\n");
            exit(1);
        }

        if self.urls.is_empty() {
            eprintln!("\nNo urls provided\n");
            exit(1);
        }

        let format = match self.download_type.unwrap() {
            DownloadType::VideoWithAudio => {
                format!(
                    "bv*[height<={0}]+ba/b[height<={0}]",
                    self.quality.unwrap().to_str()
                )
            }
            DownloadType::VideoOnly => todo!(),
            DownloadType::Audio => todo!(),
        };

        let simulate = if self.simulate {
            "--simulate"
        } else {
            "--no-simulate"
        };

        let mut output: Vec<String> = vec![
            simulate.into(),
            "--print".into(),
            "id,title,filesize_approx,duration_string".into(),
            "--format".into(),
            format,
        ];

        for url in self.urls {
            output.push(url);
        }

        output
    }
}

#[derive(Debug)]
pub enum DownloadType {
    VideoWithAudio,
    VideoOnly,
    Audio,
}

impl DownloadType {
    pub fn options() -> Vec<String> {
        vec![
            "video(s) with audio",
            "video(s) without audio",
            "audio(s) only",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    pub fn from_option(option: &str) -> Self {
        match option {
            "video(s) with audio" => Self::VideoWithAudio,
            "video(s) without audio" => Self::VideoOnly,
            "audio(s) only" => Self::Audio,
            _ => {
                eprintln!("\nPlease select a download type\n");
                exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub enum DownloadQuality {
    P360,
    P480,
    P720,
    P1024,
    P2048,
}

impl DownloadQuality {
    pub fn options() -> Vec<String> {
        vec!["360p", "480p", "720p", "1024p", "2048p"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn from_option(option: &str) -> Self {
        match option {
            "360p" => Self::P360,
            "480p" => Self::P480,
            "720p" => Self::P720,
            "1024p" => Self::P1024,
            "2048p" => Self::P2048,
            _ => {
                eprintln!("\nPlease select a quality\n");
                exit(1);
            }
        }
    }

    pub fn to_str(&self) -> String {
        let output = match self {
            Self::P360 => "360",
            Self::P480 => "480",
            Self::P720 => "720",
            Self::P1024 => "1024",
            Self::P2048 => "2048",
        };

        output.into()
    }
}
