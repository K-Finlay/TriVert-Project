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

/// The Vec2f struct
///
/// It is used mainly for 2D releated mathematics (e.g. texture and UV coordinates).
/// 32-bit floats are used for the values.
pub struct Vec2f {

    x : f32,
    y : f32
}

/*================================================================================================*/
/*------PUBLIC FUNCTIONS--------------------------------------------------------------------------*/
/*================================================================================================*/

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

/*================================================================================================*/
/*------PUBLIC STATIC FUNCTIONS-------------------------------------------------------------------*/
/*================================================================================================*/

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

/*================================================================================================*/
/*------OPERATOR OVERLOADS------------------------------------------------------------------------*/
/*================================================================================================*/

impl Add for Vec2f {

    type Output = Vec2f;

    // Addition operator (vector)
    fn add (self, rhs : Vec2f) -> Vec2f {

        Vec2f {x : self.x + rhs.x,
               y : self.y + rhs.y}
    }
}

/*================================================================================================*/

impl Add <f32> for Vec2f {

    type Output = Vec2f;

    // Addition operator (f32)
    fn add (self, rhs : f32) -> Vec2f {

        Vec2f {x : self.x + rhs,
               y : self.y + rhs}
    }
}

/*================================================================================================*/

impl AddAssign for Vec2f {

    // Addition assignment operator (vector)
    fn add_assign (&mut self, rhs : Vec2f) {

        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/*================================================================================================*/

impl AddAssign <f32> for Vec2f {

    // Addition assignment operator (f32)
    fn add_assign (&mut self, rhs : f32) {

        self.x += rhs;
        self.y += rhs;
    }
}
