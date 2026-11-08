#![cfg_attr(not(feature = "std"), no_std)]
#![feature(portable_simd)]
#![feature(stdarch_riscv_feature_detection)]
#![feature(riscv_target_feature)]

pub mod gemm;
pub mod microkernel;

#[macro_use]
extern crate gemm_common;
