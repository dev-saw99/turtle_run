use log::LevelFilter;

use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn get_log_file_path() -> String {
    "/tmp/turtle_run/turtle_run.log".to_string()
}

pub fn init_file_logger() {
    let file_path = get_log_file_path();
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)} {l} - {f}:{L} {m}{n})}",
        )))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

pub fn init_file_and_console_logger() {
    let file_path = get_log_file_path();
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)} {l} - {f}:{L} {m}{n})}",
        )))
        .build(file_path)
        .unwrap();

    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)} {l} - {f}:{L} {m}{n})}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(
            Root::builder()
                .appender("file")
                .appender("console")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
