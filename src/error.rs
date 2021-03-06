// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use std::io;
use std::convert::From;
use cbor::CborError;
use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::error;
use std::fmt;

//------------------------------------------------------------------------------
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ResponseError {
    NoData,
    InvalidRequest,
    FailedToStoreData(Vec<u8>)
}

impl error::Error for ResponseError {
    fn description(&self) -> &str {
        match *self {
            ResponseError::NoData => "No Data",
            ResponseError::InvalidRequest => "Invalid request",
            ResponseError::FailedToStoreData(_) => "Failed to store data",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResponseError::NoData => fmt::Display::fmt("ResponsError::NoData", f),
            ResponseError::InvalidRequest => fmt::Display::fmt("ResponsError::InvalidRequest", f),
            ResponseError::FailedToStoreData(_) =>
                fmt::Display::fmt("ResponseError::FailedToStoreData", f),
        }
    }
}

impl Encodable for ResponseError {
    fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
        let mut type_tag;
        let mut data : Option<Vec<u8>> = None;
        match *self {
            ResponseError::NoData => type_tag = "NoData",
            ResponseError::InvalidRequest => type_tag = "InvalidRequest",
            ResponseError::FailedToStoreData(ref err_data) => {
                type_tag = "FailedToStoreData";
                data = Some(err_data.clone());
            }
        };
        CborTagEncode::new(5483_100, &(&type_tag, &data)).encode(e)
    }
}

impl Decodable for ResponseError {
    fn decode<D: Decoder>(d: &mut D)->Result<ResponseError, D::Error> {
        try!(d.read_u64());
        // let mut type_tag : String;
        // // let mut data : Option<Vec<u8>>;
        let (type_tag, data) : (String, Option<Vec<u8>>)
            = try!(Decodable::decode(d));
        match &type_tag[..] {
            "NoData" => Ok(ResponseError::NoData),
            "InvalidRequest" => Ok(ResponseError::InvalidRequest),
            "FailedToStoreData" => {
                match data {
                    Some(err_data) => Ok(ResponseError::FailedToStoreData(err_data)),
                    None => Err(d.error("No data in FailedToStoreData"))
                }
            },
            _ => Err(d.error("Unrecognised ResponseError"))
        }
    }
}

//------------------------------------------------------------------------------
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum InterfaceError {
    Abort,
    Response(ResponseError),
}

impl From<ResponseError> for InterfaceError {
    fn from(e: ResponseError) -> InterfaceError {
        InterfaceError::Response(e)
    }
}

impl error::Error for InterfaceError {
    fn description(&self) -> &str {
        match *self {
            InterfaceError::Abort => "Aborted",
            InterfaceError::Response(ref err) => "Invalid response",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            InterfaceError::Response(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterfaceError::Abort => fmt::Display::fmt("InterfaceError::Abort", f),
            InterfaceError::Response(ref err) => fmt::Display::fmt(err, f)
        }
    }
}



//------------------------------------------------------------------------------
#[derive(Debug)]
pub enum RoutingError {
    BadAuthority,
    AlreadyConnected,
    UnknownMessageType,
    FilterCheckFailed,
    FailedToBootstrap,
    RoutingTableEmpty,
    Interface(InterfaceError),
    Io(io::Error),
    Cbor(CborError),
    Response(ResponseError),
}

impl From<ResponseError> for RoutingError {
    fn from(e: ResponseError) -> RoutingError { RoutingError::Response(e) }
}

impl From<CborError> for RoutingError {
    fn from(e: CborError) -> RoutingError { RoutingError::Cbor(e) }
}

impl From<io::Error> for RoutingError {
    fn from(e: io::Error) -> RoutingError { RoutingError::Io(e) }
}

impl From<InterfaceError> for RoutingError {
    fn from(e: InterfaceError) -> RoutingError { RoutingError::Interface(e) }
}

impl error::Error for RoutingError {
    fn description(&self) -> &str {
        match *self {
            RoutingError::BadAuthority => "Invalid authority",
            RoutingError::AlreadyConnected => "Already connected",
            RoutingError::UnknownMessageType => "Invalid message type",
            RoutingError::FilterCheckFailed => "Filter check failure",
            RoutingError::FailedToBootstrap => "Could not bootstrap",
            RoutingError::RoutingTableEmpty => "Routing table empty",
            RoutingError::Interface(ref e) => "Interface error",
            RoutingError::Io(ref err) => "I/O error",
            RoutingError::Cbor(ref err) => "Serialisation error",
            RoutingError::Response(ref err) => "Response error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RoutingError::Interface(ref err) => Some(err as &error::Error),
            RoutingError::Io(ref err) => Some(err as &error::Error),
            // RoutingError::Cbor(ref err) => Some(err as &error::Error),
            RoutingError::Response(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoutingError::BadAuthority => fmt::Display::fmt("Bad authority", f),
            RoutingError::AlreadyConnected => fmt::Display::fmt("already connected", f),
            RoutingError::UnknownMessageType => fmt::Display::fmt("Unknown message", f),
            RoutingError::FilterCheckFailed => fmt::Display::fmt("filter check failed", f),
            RoutingError::FailedToBootstrap => fmt::Display::fmt("could not bootstrap", f),
            RoutingError::RoutingTableEmpty => fmt::Display::fmt("routing table empty", f),
            RoutingError::Interface(ref err) => fmt::Display::fmt(err, f),
            RoutingError::Io(ref err) => fmt::Display::fmt(err, f),
            RoutingError::Cbor(ref err) => fmt::Display::fmt(err, f),
            RoutingError::Response(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
