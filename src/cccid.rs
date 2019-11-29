//! Cardholder Capability Container (CCC) ID Support

// Adapted from yubico-piv-tool:
// <https://github.com/Yubico/yubico-piv-tool/>
//
// Copyright (c) 2014-2016 Yubico AB
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//   * Redistributions of source code must retain the above copyright
//     notice, this list of conditions and the following disclaimer.
//
//   * Redistributions in binary form must reproduce the above
//     copyright notice, this list of conditions and the following
//     disclaimer in the documentation and/or other materials provided
//     with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{consts::*, error::Error, yubikey::YubiKey};
use getrandom::getrandom;

/// Cardholder Capability Container (CCC) Template
///
/// f0: Card Identifier
///
///  - 0xa000000116 == GSC-IS RID
///  - 0xff == Manufacturer ID (dummy)
///  - 0x02 == Card type (javaCard)
///  - next 14 bytes: card ID
const CCC_TMPL: &[u8] = &[
    0xf0, 0x15, 0xa0, 0x00, 0x00, 0x01, 0x16, 0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0x01, 0x21, 0xf2, 0x01, 0x21, 0xf3, 0x00, 0xf4,
    0x01, 0x00, 0xf5, 0x01, 0x10, 0xf6, 0x00, 0xf7, 0x00, 0xfa, 0x00, 0xfb, 0x00, 0xfc, 0x00, 0xfd,
    0x00, 0xfe, 0x00,
];

/// Cardholder Capability Container (CCC) Identifier
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CCCID(pub [u8; YKPIV_CCCID_SIZE]);

impl CCCID {
    /// Generate a random CCCID
    pub fn generate() -> Result<Self, Error> {
        let mut id = [0u8; YKPIV_CCCID_SIZE];
        getrandom(&mut id).map_err(|_| Error::RandomnessError)?;
        Ok(CCCID(id))
    }

    /// Get Cardholder Capability Container (CCC) ID
    pub fn get(yubikey: &mut YubiKey) -> Result<Self, Error> {
        let txn = yubikey.begin_transaction()?;
        let response = txn.fetch_object(YKPIV_OBJ_CAPABILITY)?;

        if response.len() != CCC_TMPL.len() {
            return Err(Error::GenericError);
        }

        let mut cccid = [0u8; YKPIV_CCCID_SIZE];
        cccid.copy_from_slice(&response[CCC_ID_OFFS..(CCC_ID_OFFS + YKPIV_CCCID_SIZE)]);
        Ok(CCCID(cccid))
    }

    /// Get Cardholder Capability Container (CCC) ID
    pub fn set(&self, yubikey: &mut YubiKey) -> Result<(), Error> {
        let mut buf = CCC_TMPL.to_vec();
        buf[CCC_ID_OFFS..(CCC_ID_OFFS + self.0.len())].copy_from_slice(&self.0);

        let txn = yubikey.begin_transaction()?;
        txn.save_object(YKPIV_OBJ_CAPABILITY, &buf)
    }
}
