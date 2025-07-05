use core::cell::OnceCell;

use alloc::vec;
use alloc::vec::Vec;
use ark_bn254::Bn254;
use ark_ec::{bn::BnConfig, pairing::Pairing};
use ark_ff::biginteger::BigInt;

type BaseField = <Bn254 as Pairing>::BaseField;
type ScalarField = <Bn254 as Pairing>::ScalarField;
type G1Affine = <Bn254 as Pairing>::G1Affine;
type Fp2 = ark_ff::Fp2::<<ark_bn254::Config as BnConfig>::Fp2Config>;
type G2Affine = <Bn254 as Pairing>::G2Affine;

pub fn public() -> Vec<ScalarField> {
    // static INSTANCE: OnceCell<Vec<ScalarField>> = OnceCell::new();
    // INSTANCE.get_or_init(|| -> Vec<ScalarField> {
        vec![
            ScalarField::new(BigInt::new([6, 0, 0, 0])),
        ]
    // })
}
