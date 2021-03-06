/*================================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*================================================================================================*/

extern crate time;

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

/*================================================================================================*/
/*------ENUMS-------------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The log level enum
///
/// It is used with the logger struct, and is used to determine the severity of the message.
#[derive (Copy, Clone)]
pub enum LogLevel {

    /// Used for logging debug messages (stripped out of release builds)
    Debug (&'static str),
    /// Used for normal logging messages
    Info (&'static str),
    /// Used for warnings (non critical errors)
    Warning (&'static str),
    /// Used for critical errors
    Error (&'static str)
}

/*================================================================================================*/
/*------STRUCTS-----------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The logger struct.
///
/// It allows for basic logging capabilities.
/// It can currently log strings, but others will be added in the future.
pub struct Logger {

    // Private
    log_file    : Option <BufWriter <File>>,
    is_enabled  : bool,
    is_logging  : bool
}

/*================================================================================================*/
/*------PUBLIC STATIC FUNCTIONS-------------------------------------------------------------------*/
/*================================================================================================*/

impl Logger {

    /// Creates a new logger instance
    ///
    /// Logging is disabled on faliure.
    ///
    /// # Examples
    /// ```
    /// let logger = Logger::new ("log.txt");
    pub fn new (log_file_path : &str) -> Logger {

        match File::create (&log_file_path) {

            Ok (file) => Logger {log_file   : Some (BufWriter::new (file)),
                                 is_enabled : true,
                                 is_logging : false},

            Err (_) => {

                println! ("{}\n{}",
                          "WARNING: Log file could not be created",
                          "         Logging has been disabled\n");

                Logger {log_file   : None,
                        is_enabled : false,
                        is_logging : false}
            }
        }
    }

/*================================================================================================*/
/*------PUBLIC FUNCTIONS--------------------------------------------------------------------------*/
/*================================================================================================*/

    /// Starts file logging
    pub fn begin_log (&mut self) {

        // Check if enabled
        if self.is_enabled {

            self.is_logging = true;
            self.log_file.as_mut ().unwrap ().write ("---BEGIN LOG---\n\n".as_bytes ()).unwrap ();
        }
    }

/*================================================================================================*/

    /// Logs a message
    ///
    /// # Examples
    /// ```
    /// let mut logger = Logger::new ("log.txt");
    ///
    /// logger.log (LogLevel::Debug   ("This is a log entry")) // Debug
    /// logger.log (LogLevel::Message ("This is a log entry")) // Message
    /// logger.log (LogLevel::Warning ("This is a log entry")) // Warning
    /// logger.log (LogLevel::Error   ("This is a log entry")) // Error
    pub fn log (&mut self, log_level : LogLevel) {

        // Check if logger is enabled and logging
        if self.is_enabled && self.is_logging {

            // Get the buffer from the logger struct
            let buffer = self.log_file.as_mut ().unwrap ();

            match log_level {

                // Debug
                LogLevel::Debug (m) => {

                    // Only call if run in debug
                    if cfg! (debug_assertions) {

                        let log_string = format! ("DEBUG   ({}) {}\n", time::now ().asctime (), m);
                        buffer.write (log_string.as_bytes ()).unwrap ();
                    }
                },

                // Message
                LogLevel::Info (m) => {

                    let log_string = format! ("INFO    ({}) {}\n", time::now ().asctime (), m);
                    buffer.write (log_string.as_bytes ()).unwrap ();
                },

                // Warning
                LogLevel::Warning (m) => {

                    let log_string = format! ("WARNING ({}) {}\n", time::now ().asctime (), m);
                    buffer.write (log_string.as_bytes ()).unwrap ();
                },

                // Error
                LogLevel::Error (m) => {

                    let log_string = format! ("ERROR   ({}) {}\n", time::now ().asctime (), m);
                    buffer.write (log_string.as_bytes ()).unwrap ();
                }
            }
        }
    }

/*================================================================================================*/

    /// Stops file logging
    pub fn end_log (&mut self) {

        // Check if enabled
        if self.is_enabled {

            self.is_logging = false;
            self.log_file.as_mut ().unwrap ().write ("\n---END LOG---\n".as_bytes ()).unwrap ();
        }
    }
}
