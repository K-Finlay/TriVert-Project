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

/*================================================================================================*/
//! This crate contains all mathematical functionality for use in TriVert.
//!
//! While a part of TriVert, has no dependencies, and can be used independently
//! to the rest of the project.
/*================================================================================================*/

// Crate attributes
#![allow   (dead_code)]
#![deny    (missing_docs)]
#![feature (augmented_assignments)]
#![feature (op_assign_traits)]

// Static variables
/// The value of PI.
pub static PI       : f32 = 3.141592;
/// Half of PI.
pub static HALF_PI  : f32 = 1.570796;

// Private modules
mod mathf;
mod vec2f;

// Public module exports
pub use self::mathf::Mathf;
pub use self::vec2f::Vec2f;
