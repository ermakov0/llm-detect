use std::env;
use std::error::Error;
use std::process::ExitCode;

use docx_lite::extract_text;
use log::LevelFilter;
use log::{info, warn};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

fn init_log() {
    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(
        LevelFilter::Info,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage:");
        eprintln!("./{}: file.docx", args[0]);
        return Ok(ExitCode::FAILURE);
    }

    init_log();
    let text = extract_text(&args[1])?;

    let mut count_warn = 0;
    let mut count_info = 0;

    for (line_num, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // C#
        // TODO  set { weightKg = value >= 0 ? value : 0; }
        if let Some(idx) = line.trim().find("//")
            && idx > 0
        {
            warn!("{}: {line}", line_num + 1);
        }
        if line.contains("=>") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("??=") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("this(") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("===") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("---") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("tring('='") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("tring('-'") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        // Console.WriteLine($"Vector({X:F1}, {Y:F1}) | Length = {Length():F3}");
        if line.contains(":F3}") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("Console.WriteLine(\"[") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("\\\"") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains(".PadRight(") {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if line.contains("1. ")
            || line.contains("2. ")
            || line.contains("3. ")
            || line.contains("4. ")
            || line.contains("5. ")
            || line.contains("6. ")
        {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }

        if line
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .any(|window| window.iter().all(|&c| c.is_uppercase()))
        {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }

        if line.contains("Console.WriteLine(\"\\n") {
            info!("{}: {line}", line_num + 1);
            count_info += 1;
        }
        // {consumer.TotalPayment()}"
        if line.contains("()}") {
            info!("{}: {line}", line_num + 1);
            count_info += 1;
        }
        if line.contains("new(") {
            info!("{}: {line}", line_num + 1);
            count_info += 1;
        }
        // if line.contains("<summary>") {
        //     info!("{}: {line}", line_num + 1);
        //     count_info += 1;
        // }
    }

    info!("\nwarnings={count_warn} info={count_info}");

    Ok(ExitCode::SUCCESS)
}
