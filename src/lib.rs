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
//! This is the main crate of the TriVert project.
//!
//! TriVert is a game framework written in Rust.
//! It aims to be simple to use, while remaining powerful and flexible.
//!
//! At this point in time, unstable rust is required to build TriVert.
//! As Rusts standard library stabalizes, this may change in the future.
/*================================================================================================*/

// Crate attributes
#![allow (dead_code)]
#![deny  (missing_docs)]

// External crates
extern crate trivert_math;

// Public module exports
pub use trivert_math as math;
