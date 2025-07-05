use once_cell::unsync::OnceCell;

use ark_bn254::Bn254;
use ark_ec::{bn::BnConfig, pairing::Pairing};
use ark_ff::biginteger::BigInt;
use ark_groth16::data_structures::Proof;

type BaseField = <Bn254 as Pairing>::BaseField;
type G1Affine = <Bn254 as Pairing>::G1Affine;
type Fp2 = ark_ff::Fp2::<<ark_bn254::Config as BnConfig>::Fp2Config>;
type G2Affine = <Bn254 as Pairing>::G2Affine;

pub fn proof() -> Proof<Bn254> {
    // static INSTANCE: OnceCell<Proof<Bn254>> = OnceCell::new();
    // INSTANCE.get_or_init(|| -> Proof::<Bn254> {
        Proof::<Bn254> {
            a: G1Affine::new(
                BaseField::new(BigInt::new([
                    0xa13d79fae2bcb272,
                    0x10b9466446ad99a6,
                    0x4b7491a5e17bbbf6,
                    0x204b73baad824d54,
                ])),
                BaseField::new(BigInt::new([
                    0x7433fd51bad7a84a,
                    0xb8719ea7970a11a7,
                    0xe5cb35ad913a7ea5,
                    0x223d3bfefb641b48,
                ])),
            ),
            b: G2Affine::new(
                Fp2::new(
                    BaseField::new(BigInt::new([
                        0xbba94bc73a55eb79,
                        0xd7894e9946bbbbf7,
                        0x23a8e16179759db2,
                        0x1b99891f4785506a,
                    ])),
                    BaseField::new(BigInt::new([
                        0xc5e52e21439fc805,
                        0x8694e9bc39c4f078,
                        0xfc300c3e164ea4fc,
                        0x15ff0905b08c3de1,
                    ])),
                ),
                Fp2::new(
                    BaseField::new(BigInt::new([
                        0x462b93ea90994c82,
                        0x6d2d3ed3b88d40d9,
                        0x837e01dfd8598651,
                        0x3051cb7b87440cdd,
                    ])),
                    BaseField::new(BigInt::new([
                        0x9e474efeb221cabd,
                        0x72b264c958e8022a,
                        0xe1d9931c46fe4786,
                        0x0aaec41ebfaafeb4,
                    ])),
                ),
            ),
            c: G1Affine::new(
                BaseField::new(BigInt::new([
                    0x76794efd32f22f31,
                    0x3eb4c3ebe189a18f,
                    0xda22eb1ae3783931,
                    0x1f7ea1477e29a75b,
                ])),
                BaseField::new(BigInt::new([
                    0x57bbbd499b5dd0f4,
                    0xed48a4ca08cd5c71,
                    0x58cc14815289f9a9,
                    0x1b6f691a4d6bf18a,
                ])),
            ),
        }
    // })
}
