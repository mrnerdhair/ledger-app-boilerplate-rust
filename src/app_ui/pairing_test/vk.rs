use once_cell::unsync::OnceCell;

use alloc::vec;
use ark_bn254::Bn254;
use ark_ec::{bn::BnConfig, pairing::Pairing};
use ark_ff::biginteger::BigInt;
use ark_groth16::data_structures::{PreparedVerifyingKey, VerifyingKey};

type BaseField = <Bn254 as Pairing>::BaseField;
type G1Affine = <Bn254 as Pairing>::G1Affine;
type Fp2 = ark_ff::Fp2<<ark_bn254::Config as BnConfig>::Fp2Config>;
type G2Affine = <Bn254 as Pairing>::G2Affine;

pub fn vk() -> VerifyingKey<Bn254> {
    // static INSTANCE: OnceCell<PreparedVerifyingKey<Bn254>> = OnceCell::new();
    // INSTANCE.get_or_init(|| -> PreparedVerifyingKey::<Bn254> {
    VerifyingKey::<Bn254> {
        alpha_g1: G1Affine::new(
            BaseField::new(BigInt::new([
                0x22f599a2be6df2e2,
                0xdbea33fbb16c643b,
                0x41749d5507949d05,
                0x2d4d9aa7e302d9df,
            ])),
            BaseField::new(BigInt::new([
                0xf076caff004d1926,
                0xce89830a19230301,
                0x61d8ec60209fe345,
                0x14bedd503c37ceb0,
            ])),
        ),
        beta_g2: G2Affine::new(
            Fp2::new(
                BaseField::new(BigInt::new([
                    0x0213bc7fc13db7ab,
                    0x7dd68bc0e071241e,
                    0x74d0d6732bf50184,
                    0x0e187847ad4c7983,
                ])),
                BaseField::new(BigInt::new([
                    0xa9794cbc3bf3060c,
                    0xd38480a653f2deca,
                    0xafc985f88877f182,
                    0x0967032fcbf776d1,
                ])),
            ),
            Fp2::new(
                BaseField::new(BigInt::new([
                    0xd57f06547ad0cec8,
                    0xf896b7c63eea05a9,
                    0x313123d24d2f9192,
                    0x1739c1b1a457a8c7,
                ])),
                BaseField::new(BigInt::new([
                    0x9da69a4d112346a7,
                    0xaafddec46b7a0d37,
                    0x99f5e847d93f8c3c,
                    0x304cfbd1e08a704a,
                ])),
            ),
        ),
        gamma_g2: G2Affine::new(
            Fp2::new(
                BaseField::new(BigInt::new([
                    0x46debd5cd992f6ed,
                    0x674322d4f75edadd,
                    0x426a00665e5c4479,
                    0x1800deef121f1e76,
                ])),
                BaseField::new(BigInt::new([
                    0x97e485b7aef312c2,
                    0xf1aa493335a9e712,
                    0x7260bfb731fb5d25,
                    0x198e9393920d483a,
                ])),
            ),
            Fp2::new(
                BaseField::new(BigInt::new([
                    0x4ce6cc0166fa7daa,
                    0xe3d1e7690c43d37b,
                    0x4aab71808dcb408f,
                    0x12c85ea5db8c6deb,
                ])),
                BaseField::new(BigInt::new([
                    0x55acdadcd122975b,
                    0xbc4b313370b38ef3,
                    0xec9e99ad690c3395,
                    0x090689d0585ff075,
                ])),
            ),
        ),
        delta_g2: G2Affine::new(
            Fp2::new(
                BaseField::new(BigInt::new([
                    0xf4f55dc742e05837,
                    0x6ab0f319f8c354e0,
                    0xaf7ebd804b8bd6c4,
                    0x01975aa2eb19e86d,
                ])),
                BaseField::new(BigInt::new([
                    0x1cbb26ea60773789,
                    0x7f5acddfb6ef767c,
                    0x3ce1be71145b4ea6,
                    0x2dd5ffc26f0188d5,
                ])),
            ),
            Fp2::new(
                BaseField::new(BigInt::new([
                    0x0b90a4355a6f1335,
                    0x180dab61aa6cf09e,
                    0xf7bc80b3e5817328,
                    0x202106289262db8c,
                ])),
                BaseField::new(BigInt::new([
                    0xf315fa935a65982b,
                    0x32795fb0a8383cf3,
                    0x86ed793645e73525,
                    0x0139267c5edd5c68,
                ])),
            ),
        ),
        gamma_abc_g1: vec![
            G1Affine::new(
                BaseField::new(BigInt::new([
                    0x1ea235c128ec8b0d,
                    0xf7a7d268c07b6be7,
                    0xa72138125f42fb6c,
                    0x0f13deecd2bb97a4,
                ])),
                BaseField::new(BigInt::new([
                    0x53c88cc5ecd13afc,
                    0x50108204685faf29,
                    0xd848da5554d22ffe,
                    0x141a084ea96151ed,
                ])),
            ),
            G1Affine::new(
                BaseField::new(BigInt::new([
                    0x7f158ef8019bdffb,
                    0xbff334e68cc531b5,
                    0x205d0f7173927374,
                    0x27890d5f3cdd70f4,
                ])),
                BaseField::new(BigInt::new([
                    0x3bfc87163cab5315,
                    0xec0eab67ecb7ca64,
                    0x35715c99a3e1f741,
                    0x29cd386360b0c1e9,
                ])),
            ),
        ],
    }
    .into()
    // })
}
