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
    println!(concat!(
        env!("CARGO_PKG_NAME"),
        " ",
        env!("CARGO_PKG_VERSION")
    ));

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
        let is_detect = line
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .any(|window| window.iter().all(|&c| c.is_uppercase()))
            || line.contains("=>")
            || line.contains("??=")
            || line.contains("this(")
            || line.contains("===")
            || line.contains("---")
            || line.contains("tring('='")
            || line.contains("tring('-'")
            || line.contains("Console.WriteLine(\"[")
            || line.contains("\\\"")
            || line.contains(":F1}")
            || line.contains(":F2}")
            || line.contains(":F3}")
            || line.contains(":F4}")
            || line.contains(":F5}")
            || line.contains(":F6}")
            || line.contains("1. ")
            || line.contains("2. ")
            || line.contains("3. ")
            || line.contains("4. ")
            || line.contains("5. ")
            || line.contains("6. ")
            || line.contains("var")
            || line.contains("var")
            || line.contains("select")
            || line.contains("from")
            || line.contains("where")
            || line.contains("orderby")
            || line.contains("System.Linq")
            || line.contains(".Select(")
            || line.contains(".Where(")
            || line.contains(".OrderBy(")
            || line.contains(".OrderByDescending(")
            || line.contains(".ThenBy(")
            || line.contains(".ThenByDescending(")
            || line.contains(".Join(")
            || line.contains(".Aggregate(")
            || line.contains(".ToLookup(")
            || line.contains(".GroupJoin(")
            || line.contains(".Reverse(")
            || line.contains(".All(")
            || line.contains(".Any(")
            || line.contains(".Contains(")
            || line.contains(".Distinct(")
            || line.contains(".Except(")
            || line.contains(".Union(")
            || line.contains(".Intersect(")
            || line.contains(".Count(")
            || line.contains(".Sum(")
            || line.contains(".Average(")
            || line.contains(".Min(")
            || line.contains(".Max(")
            || line.contains(".Take(")
            || line.contains(".Skip(")
            || line.contains(".TakeWhile(")
            || line.contains(".SkipWhile(")
            || line.contains(".Concat(")
            || line.contains(".Zip(")
            || line.contains(".First(")
            || line.contains(".FirstOrDefault(")
            || line.contains(".Single(")
            || line.contains(".SingleOrDefault(")
            || line.contains(".ElementAt(")
            || line.contains(".ElementAtOrDefault(")
            || line.contains(".Last(")
            || line.contains(".LastOrDefault(")
            || line.contains(".PadRight(");

        if is_detect {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }
        if let Some(idx) = line.trim().find("//")
            && idx > 0
        {
            warn!("{}: {line}", line_num + 1);
            count_warn += 1;
        }

        let is_detect = line.contains("Console.WriteLine(\"\\n")
            || line.contains("()}")
            || line.contains("new(")
            || line.contains("<summary>");

        if is_detect {
            info!("{}: {line}", line_num + 1);
            count_info += 1;
        }
    }

    info!("\nwarnings={count_warn} info={count_info}");

    Ok(ExitCode::SUCCESS)
}
