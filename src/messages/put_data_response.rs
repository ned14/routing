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

#![allow(unused_assignments)]

use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use NameType;
use error::ResponseError;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PutDataResponse {
    pub name : NameType,
    pub data : Result<Vec<u8>, ResponseError>,
}

impl Encodable for PutDataResponse {
    fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
        let error : Option<&ResponseError> = match &self.data {
            &Ok(_) => None,
            &Err(ref e) => Some(e),
        };

        let dummy = Vec::new();

        let data : &Vec<u8> = match self.data {
            Ok(ref data) => data,
            Err(_) => &dummy,
        };

        CborTagEncode::new(5483_001, &(&self.name, &data, &error)).encode(e)
    }
}

impl Decodable for PutDataResponse {
  fn decode<D: Decoder>(d: &mut D)->Result<PutDataResponse, D::Error> {
    try!(d.read_u64());
    let (name, data, error) = try!(Decodable::decode(d));

    match error {
        None        => Ok(PutDataResponse { name: name, data: Ok(data) }),
        Some(error) => Ok(PutDataResponse { name: name, data: Err(error)})
    }
  }
}

#[cfg(test)]
mod test {
    use cbor;
    use super::*;
    use test_utils::Random;

    #[test]
    fn put_data_response_serialisation() {
        let obj_before : PutDataResponse = Random::generate_random();

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: PutDataResponse = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
