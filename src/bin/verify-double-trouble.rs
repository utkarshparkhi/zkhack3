#![allow(unused, unreachable_code)]
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use std::collections::VecDeque;
use std::ops::Sub;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    let c1 = challenge(&ck, &instance1, &proof1.commitment);
    let c2 = challenge(&ck, &instance2, &proof2.commitment);
    let r1 = &proof1
        .response
        .s
        .iter()
        .zip(proof2.response.s.iter())
        .map(|(&s1, &s2)| (s1 - s2) / (c1 - (c2.double())))
        .collect::<Vec<_>>();
    let a = &proof1
        .response
        .s
        .iter()
        .zip(r1)
        .map(|(&s1, &r1)| s1 - (c1 * r1))
        .collect::<Vec<_>>();
    let rho_1 = (proof1.response.u - proof2.response.u) / (c1 - (c2.double()));
    let comm_a_rand = proof1.response.u - (c1 * rho_1);
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}
