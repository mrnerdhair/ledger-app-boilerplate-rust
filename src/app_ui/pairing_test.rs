/*****************************************************************************
 *   Ledger App Boilerplate Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/
use crate::{debug_print, AppSW};

use alloc::format;
use include_gif::include_gif;
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview};

use alloc::vec::Vec;
use alloc::string::String;

use ark_groth16::Groth16;
use ark_bn254::Bn254;

pub mod proof;
pub mod public;
pub mod vk;

fn verify() {
    let vk = vk::vk();

    // let foo = Bn254::pairing(vk.alpha_g1.into(), vk.gamma_abc_g1[0].into);

    // debug_print(&format!("floop: {}", vk.gamma_abc_g1.len()));

    let proof = proof::proof();
    let public = public::public();

    Groth16::<Bn254>::verify_proof(&vk, &proof, &public).unwrap();
}

pub fn ui_display_pairing_test(data: Vec<u8>) -> Result<(), AppSW> {
    let data_str = String::from_utf8(data).map_err(|_| AppSW::TxDisplayFail)?;
    
    verify();

    // Define transaction review fields
    let my_fields = [
        Field {
            name: "Amount",
            value: "Florp",
        },
        Field {
            name: "Destination",
            value: "Norp",
        },
        Field {
            name: "Memo",
            value: data_str.as_ref(),
        },
    ];

    // Create transaction review

    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("crab_64x64.gif", NBGL));
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("crab_16x16.gif", NBGL));

    // Create NBGL review. Maximum number of fields and string buffer length can be customised
    // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
    let review: NbglReview = NbglReview::new()
        .titles(
            "Pairing test",
            "",
            "Fiddlesticks",
        )
        .glyph(&FERRIS);

    review.show(&my_fields);
    Ok(())
}
