// Copyright 2020 The Vega Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The set of common errors that can occur within runtime/service workflow.

use vega_derive::ExecutionFail;

use std::fmt::Display;

use crate::runtime::{ExecutionError, ExecutionFail};

/// List of possible common errors.
///
/// This type is not intended to be exhaustively matched. It can be extended in the future
/// without breaking the semver compatibility.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(ExecutionFail)]
#[execution_fail(crate = "crate", kind = "common")]
pub enum CommonError {
    /// The interface is absent in the service.
    NoSuchInterface = 0,
    /// The method is absent in the service.
    NoSuchMethod = 1,
    /// This caller is not authorized to call this method.
    UnauthorizedCaller = 2,
    /// Malformed arguments for calling a service interface method.
    MalformedArguments = 3,
    /// Method with provided ID existed in the past, but now is removed.
    MethodRemoved = 4,

    /// Never actually generated.
    #[doc(hidden)]
    __NonExhaustive = 255,
}

impl CommonError {
    /// Creates a `MalformedArguments` error with the user-provided error cause.
    /// The cause does not need to include the error location; this information is added
    /// by the framework automatically.
    pub fn malformed_arguments(cause: impl Display) -> ExecutionError {
        let description = format!(
            "Malformed arguments for calling a service interface method: {}",
            cause
        );
        CommonError::MalformedArguments.with_description(description)
    }
}
