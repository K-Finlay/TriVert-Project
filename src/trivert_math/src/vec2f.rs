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
#[derive (Copy, Clone, Default)]
pub struct Vec2f {

    // Public
    /// X-axis coordinate
    pub x : f32,
    /// Y-axis coordinate
    pub y : f32
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

/*================================================================================================*/

    /// Creates a vector with a value of (0, 1).
    pub fn up () -> Vec2f {

        Vec2f {x : 0.0,
               y : 1.0}
    }

/*================================================================================================*/

    /// Creates a vector with a value of (0, -1).
    pub fn down () -> Vec2f {

        Vec2f {x : 0.0,
               y : -1.0}
    }

/*================================================================================================*/

    /// Creates a vector with a value of (-1, 0).
    pub fn left () -> Vec2f {

        Vec2f {x : -1.0,
               y : 0.0}
    }

/*================================================================================================*/

    /// Creates a vector with a value of (1, 0)
    pub fn right () -> Vec2f {

        Vec2f {x : 1.0,
               y : 0.0}
    }

/*================================================================================================*/

    /// Returns the dot product of two vectors.
    pub fn dot (lhs : &Vec2f, rhs : &Vec2f) -> f32 {

        (lhs.x * rhs.x) +
        (lhs.y * rhs.y)
    }

/*================================================================================================*/

    /// Returns the distance between two vectors
    pub fn distance (start : &Vec2f, end : &Vec2f) -> f32 {

        Vec2f::length (& (*start - *end))
    }

/*================================================================================================*/

    /// Returns the length of a vector
    pub fn length (vector : &Vec2f) -> f32 {

        (vector.x * vector.x +
         vector.y * vector.y).sqrt ()
    }

/*================================================================================================*/

    /// Linearly interpolates between two vectors.
    pub fn lerp (start : &Vec2f, end : &Vec2f, percentage : f32) -> Vec2f {

        Vec2f {x : Mathf::lerp (start.x, end.x, percentage),
               y : Mathf::lerp (start.y, end.y, percentage)}
    }

/*================================================================================================*/

    /// Linearly interpolates between two vectors without clamping
    pub fn lerp_unclamped (start : &Vec2f, end : &Vec2f, percentage : f32) -> Vec2f {

        Vec2f {x : Mathf::lerp_unclamped (start.x, end.x, percentage),
               y : Mathf::lerp_unclamped (start.y, end.y, percentage)}
    }

/*================================================================================================*/

    /// Get a normalized vector.
    pub fn normalize (vector : &Vec2f) -> Vec2f {

        let length = Vec2f::length (vector);

        if length != 0.0 {

            return Vec2f {x : vector.x / length,
                          y : vector.y / length}
        }

        Vec2f::new ()
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

/*================================================================================================*/

impl Sub for Vec2f {

    type Output = Vec2f;

    // Subtraction operator (vector)
    fn sub (self, rhs : Vec2f) -> Vec2f {

        Vec2f {x : self.x - rhs.x,
               y : self.y - rhs.y}
    }
}

/*================================================================================================*/

impl Sub <f32> for Vec2f {

    type Output = Vec2f;

    // Subtraction operator (f32)
    fn sub (self, rhs : f32) -> Vec2f {

        Vec2f {x : self.x - rhs,
               y : self.y - rhs}
    }
}

/*================================================================================================*/

impl SubAssign for Vec2f {

    // Subtraction assignment operator (vector)
    fn sub_assign (&mut self, rhs : Vec2f) {

        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/*================================================================================================*/

impl SubAssign <f32> for Vec2f {

    // Subtraction assignment operator (f32)
    fn sub_assign (&mut self, rhs : f32) {

        self.x -= rhs;
        self.y -= rhs;
    }
}

/*================================================================================================*/

impl Neg for Vec2f {

    type Output = Vec2f;

    // Unary minus operator
    fn neg (self) -> Vec2f {

        Vec2f {x : -self.x,
               y : -self.y}
    }
}

/*================================================================================================*/

impl Mul for Vec2f {

    type Output = Vec2f;

    // Multiplication operator (vector)
    fn mul (self, rhs : Vec2f) -> Vec2f {

        Vec2f {x : self.x * rhs.x,
               y : self.y * rhs.y}
    }
}

/*================================================================================================*/

impl Mul <f32> for Vec2f {

    type Output = Vec2f;

    // Multiplication operator (f32)
    fn mul (self, rhs : f32) -> Vec2f {

        Vec2f {x : self.x * rhs,
               y : self.y * rhs}
    }
}

/*================================================================================================*/

impl MulAssign for Vec2f {

    // Multiplication assignment operator (vector)
    fn mul_assign (&mut self, rhs : Vec2f) {

        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/*================================================================================================*/

impl MulAssign <f32> for Vec2f {

    // Multiplication assignment operator (f32)
    fn mul_assign (&mut self, rhs : f32) {

        self.x *= rhs;
        self.y *= rhs;
    }
}

/*================================================================================================*/

impl Div for Vec2f {

    type Output = Vec2f;

    // Division operator (vector)
    fn div (self, rhs : Vec2f) -> Vec2f {

        Vec2f {x : self.x / rhs.x,
               y : self.y / rhs.y}
    }
}

/*================================================================================================*/

impl Div <f32> for Vec2f {

    type Output = Vec2f;

    // Division operator (f32)
    fn div (self, rhs : f32) -> Vec2f {

        Vec2f {x : self.x / rhs,
               y : self.y / rhs}
    }
}

/*================================================================================================*/

impl DivAssign for Vec2f {

    // Division assignment operator (vector)
    fn div_assign (&mut self, rhs : Vec2f) {

        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

/*================================================================================================*/

impl DivAssign <f32> for Vec2f {

    // Division assignment operator (f32)
    fn div_assign (&mut self, rhs : f32) {

        self.x /= rhs;
        self.y /= rhs;
    }
}

/*================================================================================================*/

impl PartialEq for Vec2f {

    // Equal to operator
    fn eq (&self, rhs : &Vec2f) -> bool {

        self.x == rhs.x &&
        self.y == rhs.y
    }

/*================================================================================================*/

    // Not equal to operator
    fn ne (&self, rhs : &Vec2f) -> bool {

        self.x != rhs.x ||
        self.y != rhs.y
    }
}

/*================================================================================================*/

impl Index <u8> for Vec2f {

    type Output = f32;

    // Index operator (immutable)
    fn index (&self, index : u8) -> &f32 {

        match index {

            0 => &self.x,
            1 => &self.y,
            _ => unreachable! ("Index out of range for Vec2f")
        }
    }
}

/*================================================================================================*/

impl IndexMut <u8> for Vec2f {

    // Index operator (mutable)
    fn index_mut (&mut self, index : u8) -> &mut f32 {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable! ("Index out of range for Vec2f")
        }
    }
}
