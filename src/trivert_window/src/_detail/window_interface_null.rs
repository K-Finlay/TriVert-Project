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

// External crates
extern crate trivert_math;

// Private module exports
use self::trivert_math as math;

// Private modules
use super::super::WindowInterface;

/*================================================================================================*/
/*------STRUCTS-----------------------------------------------------------------------------------*/
/*================================================================================================*/

/// This is the null window interface.
///
/// It is used as a failsafe if other backends fail.
/// It can also be considered a reference implimentation of the trait.
#[derive (Default)]
pub struct WindowInterfaceNull {

    // Private
    position    : math::Vec2f,
    size        : math::Vec2f,
    title       : String
}

/*================================================================================================*/
/*------GETTERS / SETTERS-------------------------------------------------------------------------*/
/*================================================================================================*/

impl WindowInterface for WindowInterfaceNull {

    // Get the window position
    fn get_position (&self) -> math::Vec2f {

        self.position
    }

/*================================================================================================*/

    // Get the window size
    fn get_size (&self) -> math::Vec2f {

        self.size
    }

/*================================================================================================*/

    // Get the window title
    fn get_title (&self) -> String {

        self.title.clone ()
    }

/*================================================================================================*/

    // Set the window position
    fn set_position (&mut self, position : math::Vec2f) {

        self.position = position;
    }

/*================================================================================================*/

    // Set the window size
    fn set_size (&mut self, size : math::Vec2f) {

        self.size = size;
    }

/*================================================================================================*/

    // Set the window title
    fn set_title (&mut self, title : String) {

        self.title = title;
    }

/*================================================================================================*/
/*------FUNCTIONS---------------------------------------------------------------------------------*/
/*================================================================================================*/

    // Initialize the window
    fn initialize (&self) -> Result <(), &'static str> {

        Result::Ok (())
    }

/*================================================================================================*/

    // Release the window
    fn release (&self) {

    }
}
