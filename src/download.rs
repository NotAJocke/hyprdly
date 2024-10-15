use std::process::Command;

pub fn make(args: Vec<String>) -> Result<String, String> {
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

#[allow(unused)]
#[derive(Debug)]
pub struct Output {
    pub id: String,
    pub title: String,
    pub bytes: String,
    pub duration: String,
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
