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

use Vec2f;

/*================================================================================================*/
/*------STRUCTS-----------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The rect struct
///
/// This struct represents a rectangle, and contains both size and position.
/// It is commonly used for things such as defining window size, and basic
/// bounding box colliion detection.
#[derive (Copy, Clone, Default)]
pub struct Rect {

    // Public
    /// The rect position
    pub position : Vec2f,
    /// The rect size
    pub size     : Vec2f
}

/*================================================================================================*/
/*------PUBLIC FUNCTIONS--------------------------------------------------------------------------*/
/*================================================================================================*/

impl Rect {

    /// Formats the rect as a string
    ///
    /// # Examples
    /// ```
    /// let rect = Rect {position : Vec2f {x : 10.0, y : 10.0},
    ///                  size     : Vec2f {x : 800.0, y : 600.0}};
    ///
    /// println! {"Rect = {}", rect.to_string ()};
    /// ```
    /// ```c
    /// Output : Rect = 10.0, 10.0, 800.0, 600.0
    pub fn to_string (&self) -> String {

        format! ("{}, {}", self.position.to_string (), self.size.to_string ())
    }

/*================================================================================================*/
/*------PUBLIC STATIC FUNCTIONS-------------------------------------------------------------------*/
/*================================================================================================*/

    /// Creates a new rect with default values
    ///
    /// # Examples
    /// ```
    /// let rect = Rect::new ();
    pub fn new () -> Rect {

        Rect {position : Vec2f::new (),
              size     : Vec2f::new ()}
    }
}

/*================================================================================================*/
/*------OPERATOR OVERLOADS------------------------------------------------------------------------*/
/*================================================================================================*/

impl PartialEq for Rect {

    // Equal to operator
    fn eq (&self, rhs : &Rect) -> bool {

        self.position == rhs.position &&
        self.size     == rhs.size
    }

/*================================================================================================*/

    // Not equal to operator
    fn ne (&self, rhs : &Rect) -> bool {

        self.position != rhs.position ||
        self.size     != rhs.size
    }
}
