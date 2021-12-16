
use chrono;
use fern::colors::{Color, ColoredLevelConfig};

//use penguin_config::app_config::LoggerConfig;
use crate::config::LoggerConfig;

/// Initializes the fern logger
pub fn init_logger(config: LoggerConfig) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::Cyan)
        .debug(Color::Magenta)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let time = chrono::Local::now().format("%H:%M:%S").to_string();


    // create logs folder if not already present
    let path = std::path::Path::new(&config.output_path);
    let prefix = path.parent().expect("couldn't get parents");
    std::fs::create_dir_all(prefix).expect("couldn't create parent dirs");


    let line_colors = colors.clone().info(Color::Green);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_arg} {time} | {level} | {color_arg}[{module}] \x1B[0m{message}",
                color_arg = {
                    format_args!(
                        "\x1B[{}m",
                        line_colors.get_color(&record.level()).to_fg_str()
                    )
                },
                time = time,
                level = colors.color(record.level()),
                message = message,
                module = record.target(),
            ))
        })
        .level(log::LevelFilter::from(config.debug_message_severity))
        .chain(std::io::stdout())
        .chain(fern::log_file(config.output_path)?)
        .apply()?;


    log::trace!("Logger initialized");

    Ok(())
}
