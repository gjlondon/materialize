// Copyright Materialize, Inc. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! Errors relevant to the coordinator.

#![forbid(missing_docs)]

use std::fmt;

use expr::GlobalId;

/// Errors the coordinator can throw that might be of interest to an end user
pub enum CoordinatorError {
    /// The tried to establish the latest timestamp and discovered that no timestamps are complete yet
    NoCompleteTimestamps(Vec<GlobalId>),
}

impl std::error::Error for CoordinatorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CoordinatorError::NoCompleteTimestamps(_) => None,
        }
    }
}

impl fmt::Display for CoordinatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Coordinator Error: ")?;
        match &self {
            CoordinatorError::NoCompleteTimestamps(tried_ids) => write!(f, "(Error Code: 1) At least one input has no complete timestamps yet: {:?}:", tried_ids),
        }
    }
}

impl fmt::Debug for CoordinatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
