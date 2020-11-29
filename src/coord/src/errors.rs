// Copyright Materialize, Inc. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! Per-connection configuration parameters and state.

#![forbid(missing_docs)]

use thiserror::Error;

use expr::GlobalId;

/// Errors the coordinator can throw that might be of interest to an end user
#[derive(Error, Debug)]
pub enum CoordinatorError {
    /// The coordinator tried an operation on a source with an input lacking a complete time stamp
    #[error("At least one input has no complete timestamps yet: {0:?}")]
    NoCompleteTimestamps(Vec<GlobalId>)
}