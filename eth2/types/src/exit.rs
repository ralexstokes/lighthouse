use super::ssz::{hash, Decodable, DecodeError, Encodable, SszStream, TreeHash};
use crate::test_utils::TestRandom;
use bls::Signature;
use rand::RngCore;

#[derive(Debug, PartialEq, Clone)]
pub struct Exit {
    pub slot: u64,
    pub validator_index: u32,
    pub signature: Signature,
}

impl Encodable for Exit {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.slot);
        s.append(&self.validator_index);
        s.append(&self.signature);
    }
}

impl Decodable for Exit {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (slot, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_index, i) = <_>::ssz_decode(bytes, i)?;
        let (signature, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            Self {
                slot,
                validator_index,
                signature,
            },
            i,
        ))
    }
}

impl TreeHash for Exit {
    fn hash_tree_root(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.slot.hash_tree_root());
        result.append(&mut self.validator_index.hash_tree_root());
        result.append(&mut self.signature.hash_tree_root());
        hash(&result)
    }
}

impl<T: RngCore> TestRandom<T> for Exit {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            slot: <_>::random_for_test(rng),
            validator_index: <_>::random_for_test(rng),
            signature: <_>::random_for_test(rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ssz::ssz_encode;
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Exit::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Exit::random_for_test(&mut rng);

        let result = original.hash_tree_root();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }
}
