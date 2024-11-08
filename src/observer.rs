/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 *
 * Copyright (c) 2024
 * All rights reserved.
 */

use crate::configuration;
use crate::utils;
use crossbeam::channel;
use crossbeam::select;
use crossbeam::sync::WaitGroup;
use std::fs;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

/// ### Observer
///
/// The observer is responsible for observing the tasks and pass it on to the producer thread.
#[derive(Debug, Clone)]
pub struct Observer {
    configuration: configuration::Configuration,
}

impl Observer {
    /// ### New
    ///
    /// Create a new instance of Observer
    pub fn new(configuration: configuration::Configuration) -> Self {
        Self { configuration }
    }

    /// ### Spawn the observer thread
    ///
    /// This function spawns the observer thread, which is responsible for observing the tasks and pass it on to the producer thread.
    pub fn spawn_observer(self: Arc<Self>) {
        log::info!("{:?}", self.configuration);
        let wg = WaitGroup::new();
        // Spawn the observer thread
        {
            log::info!("Spawning task observer thread");
            let wg = wg.clone();
            let self_clone = Arc::clone(&self);
            thread::spawn(move || {
                self_clone.monitor_file_updates();
                drop(wg);
            });
        }
        wg.wait();
    }

    fn monitor_file_updates(&self) {
        let mut last_modified_time = SystemTime::UNIX_EPOCH;
        let ticker = channel::tick(std::time::Duration::from_secs(5));

        loop {
            select! {
                recv(ticker) -> _ => {
                    if let Some(new_modified_time) = self.is_file_updated(last_modified_time) {
                        last_modified_time = new_modified_time;
                        log::info!("Task file updated, reloading tasks");
                        match utils::read_yaml_file(&utils::get_task_yaml_file_path()) {
                            Ok(content) => {
                                log::info!("Task file content: {}", content);
                            }
                            Err(e) => {
                                log::error!("{}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_file_updated(&self, last_modified_time: SystemTime) -> Option<SystemTime> {
        let file_path = utils::get_task_yaml_file_path();

        // check last time this file was modified
        let modified_time = match fs::metadata(&file_path) {
            Ok(metadata) => match metadata.modified() {
                Ok(time) => time,
                Err(e) => {
                    log::warn!(
                        "Unable to get modified time for the file:{}, error:{}",
                        file_path,
                        e
                    );
                    return None;
                }
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    log::error!("Task file not found, creating new file: {}", file_path);
                    utils::create_task_yaml_file(file_path);
                    return None;
                }
                log::warn!(
                    "Unable to read metadata for the file:{}, error:{}",
                    file_path,
                    e
                );
                return None;
            }
        };

        if modified_time > last_modified_time {
            Some(modified_time)
        } else {
            None
        }
    }
}
