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

use Mathf;

use std::ops::*;
use std::cmp::PartialEq;

/*================================================================================================*/
/*------STRUCTS-----------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The generic Vec2 struct
///
/// It is used mainly for 2D releated mathematics (e.g. texture and UV coordinates)
pub struct Vec2 <T> {

    x : T,
    y : T
}

/*================================================================================================*/

/// The f32 Vec2 struct
pub type Vec2f = Vec2 <f32>;

/// The i32 Vec2 struct
pub type Vec2i = Vec2 <i32>;

/*============================================================================================================*/
/*------PUBLIC FUNCTIONS--------------------------------------------------------------------------------------*/
/*============================================================================================================*/

impl Vec2f {

    /// Formats the vector as a string.
    ///
    /// # Examples
    /// ```
    /// let vec = Vec2f {x : 10.0, y : 5.0};
    /// println! ("Vector = {}", vec.to_string ());
    /// ```
    /// ```c
    /// Output : Vector = 10.0, 5.0
    pub fn to_string (&self) -> String {

        format! ("{}, {}", self.x, self.y)
    }

/*============================================================================================================*/
/*------PUBLIC STATIC FUNCTIONS-------------------------------------------------------------------------------*/
/*============================================================================================================*/

    /// Creates a vector with a default value of zero.
    ///
    /// # Examples
    /// ```
    /// let vec = Vec2f::new ();
    pub fn new () -> Vec2f {

        Vec2f {x : 0.0,
               y : 0.0}
    }
}
