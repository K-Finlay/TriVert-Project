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

/*================================================================================================*/
/*------TRAITS------------------------------------------------------------------------------------*/
/*================================================================================================*/

/// This is the window interface trait.
///
/// It is implemented by all window backends.
/// This is to allow for things such as loading a backend dynamically from a shared library.
pub trait WindowInterface {

    // Getters
    /// Get the current position of the window
    fn get_position (&self) -> math::Vec2f;
    /// Get the current size of the window
    fn get_size (&self) -> math::Vec2f;

    // Setters
    /// Set the position of the window
    fn set_position (&self, math::Vec2f);
    /// Set the size of the window
    fn set_size (&self, math::Vec2f);
}
