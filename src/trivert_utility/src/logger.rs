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

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

/*================================================================================================*/
/*------ENUMS-------------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The log level enum
///
/// It is used with the logger struct, and is used to determine the severity of the message.
pub enum LogLevel {

    /// Used for logging debug messages (stripped out of release builds)
    Debug   (String),
    /// Used for normal logging messages
    Message (String),
    /// Used for warnings (non critical errors)
    Warning (String),
    /// Used for critical errors
    Error   (String)
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
    pub fn new (log_file_path : String) -> Logger {

        match File::create (&log_file_path) {

            Ok (file) => Logger {log_file   : Some (BufWriter::new (file)),
                                 is_enabled : true,
                                 is_logging : false},

            Err (_) => {

                println! ("WARNING: Log file could not be created\n
                           Logging has been disabled");

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
    // TODO: Write begin log string
    pub fn begin_log (&mut self) {

        self.is_logging = true;
    }

/*================================================================================================*/

    /// Logs a message
    // TODO : Add log level specific messages (ie: DEBUG: "Insert message here")
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

                        buffer.write (m.as_bytes ()).unwrap ();
                    }
                },

                // Message
                LogLevel::Message (m) => {

                    buffer.write (m.as_bytes ()).unwrap ();
                },

                // Warning
                LogLevel::Warning (m) => {

                    buffer.write (m.as_bytes ()).unwrap ();
                },

                // Error
                LogLevel::Error (m) => {

                    buffer.write (m.as_bytes ()).unwrap ();
                }
            }
        }
    }

/*================================================================================================*/

    /// Stops file logging
    // TODO: Write end log string
    pub fn end_log (&mut self) {

        self.is_logging = false;
    }
}
