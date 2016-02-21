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
//! This crate contains all window functionality for use in TriVert.
//!
//! It allows you to create a cross platform window (through the use of multiple backends),
//! and provide a rendering surface for the renderer.
/*================================================================================================*/

// Crate attributes
#![allow (dead_code)]
#![deny  (missing_docs)]

// Private modules
mod window;

// Public module exports
pub use self::window::Window;
