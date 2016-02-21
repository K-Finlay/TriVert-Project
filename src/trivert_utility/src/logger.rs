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

/*================================================================================================*/
/*------STRUCTS-----------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The logger struct.
///
/// It allows for basic logging capabilities.
/// It can currently log strings, but other will be added in the future.
pub struct Logger {

    log_file    : Option <File>,
    is_enabled  : bool
}

/*================================================================================================*/
/*------FUNCTIONS---------------------------------------------------------------------------------*/
/*================================================================================================*/

impl Logger {

    /// Creates a new logger instance
    ///
    /// Logging is disabled on faliure.
    pub fn new (log_file_path : String) -> Logger {

        match File::create (&log_file_path) {

            Ok (file) => Logger {log_file   : Some (file),
                                 is_enabled : true},

            Err (_) => {

                println! ("WARNING: Log file could not be created\n
                           Logging has been disabled");

                Logger {log_file   : None,
                        is_enabled : false}
            }
        }
    }
}
