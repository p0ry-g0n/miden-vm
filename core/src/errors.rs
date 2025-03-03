use super::Word;
use crate::utils::collections::Vec;

#[derive(Clone, Debug)]
pub enum InputError {
    NotFieldElement(u64, &'static str),
    DuplicateAdviceRoot([u8; 32]),
}

#[derive(Clone, Debug)]
pub enum AdviceSetError {
    DepthTooSmall,
    DepthTooBig(u32),
    NumLeavesNotPowerOfTwo(usize),
    InvalidKey(u64),
    InvalidIndex(u32, u64),
    InvalidDepth(u32, u32),
    InvalidPath(Vec<Word>),
    NodeNotInSet(u64),
}
