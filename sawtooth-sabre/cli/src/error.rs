// Copyright 2018 Cargill Incorporated
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

use std::borrow::Borrow;
use std::error::Error as StdError;

use sabre_sdk::protocol::payload::{ActionBuildError, SabrePayloadBuildError};
use sawtooth_sdk::signing;
use transact::{
    protocol::{batch::BatchBuildError, transaction::TransactionBuildError},
    protos::ProtoConversionError,
};

#[derive(Debug)]
pub enum CliError {
    /// The user has provided invalid inputs; the string by this error
    /// is appropriate for display to the user without additional context
    UserError(String),
    IoError(std::io::Error),
    SigningError(signing::Error),
    HyperError(hyper::Error),
    ProtocolBuildError(Box<dyn StdError>),
    ProtoConversionError(ProtoConversionError),
}

impl StdError for CliError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CliError::UserError(_) => None,
            CliError::IoError(err) => Some(err),
            CliError::SigningError(err) => Some(err),
            CliError::HyperError(err) => Some(err),
            CliError::ProtocolBuildError(ref err) => Some(err.borrow()),
            CliError::ProtoConversionError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::UserError(ref s) => write!(f, "Error: {}", s),
            CliError::IoError(ref err) => write!(f, "IoError: {}", err),
            CliError::SigningError(ref err) => write!(f, "SigningError: {}", err),
            CliError::HyperError(ref err) => write!(f, "HyperError: {}", err),
            CliError::ProtocolBuildError(ref err) => write!(f, "Protocol Error: {}", err),
            CliError::ProtoConversionError(ref err) => write!(f, "Proto Conversion Error: {}", err),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<signing::Error> for CliError {
    fn from(e: signing::Error) -> Self {
        CliError::SigningError(e)
    }
}

impl From<hyper::Error> for CliError {
    fn from(e: hyper::Error) -> Self {
        CliError::HyperError(e)
    }
}

impl From<ProtoConversionError> for CliError {
    fn from(e: ProtoConversionError) -> Self {
        CliError::ProtoConversionError(e)
    }
}

// used to convert BuildErrors into a CliError.
macro_rules! impl_builder_errors {
    ($($x:ty),*) => {
        $(
            impl From<$x> for CliError {
                fn from(e: $x) -> Self {
                    CliError::ProtocolBuildError(Box::new(e))
                }
            }
        )*
    };
}

impl_builder_errors!(
    ActionBuildError,
    BatchBuildError,
    TransactionBuildError,
    SabrePayloadBuildError
);
