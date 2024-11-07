/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 */

use log::LevelFilter;

use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn get_log_level(log_level: &String) -> LevelFilter {
    match log_level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}

pub fn init_file_logger(log_folder: &String, log_level: &String) {
    let file_path = format!("{}/turtle_run.log", log_folder);
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)} {l} - {f}:{L} {m}{n})}",
        )))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("file")
                .build(get_log_level(log_level)),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

pub fn init_file_and_console_logger(log_folder: &String, log_level: &String) {
    let file_path = format!("{}/turtle_run.log", log_folder);
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
                .build(get_log_level(log_level)),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
