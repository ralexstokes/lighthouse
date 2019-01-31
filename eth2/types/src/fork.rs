use super::ssz::{hash, Decodable, DecodeError, Encodable, SszStream, TreeHash};
use crate::test_utils::TestRandom;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fork {
    pub previous_version: u64,
    pub current_version: u64,
    pub epoch: u64,
}

impl Fork {
    /// Returns the fork version for the given `epoch`.
    pub fn get_version_for(&self, epoch: u64) -> u64 {
        if epoch < self.epoch {
            self.previous_version
        } else {
            self.current_version
        }
    }
}

impl Encodable for Fork {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.previous_version);
        s.append(&self.current_version);
        s.append(&self.epoch);
    }
}

impl Decodable for Fork {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (previous_version, i) = <_>::ssz_decode(bytes, i)?;
        let (current_version, i) = <_>::ssz_decode(bytes, i)?;
        let (epoch, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            Self {
                previous_version,
                current_version,
                epoch,
            },
            i,
        ))
    }
}

impl TreeHash for Fork {
    fn hash_tree_root(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.previous_version.hash_tree_root());
        result.append(&mut self.current_version.hash_tree_root());
        result.append(&mut self.epoch.hash_tree_root());
        hash(&result)
    }
}

impl<T: RngCore> TestRandom<T> for Fork {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            previous_version: <_>::random_for_test(rng),
            current_version: <_>::random_for_test(rng),
            epoch: <_>::random_for_test(rng),
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
        let original = Fork::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = Fork::random_for_test(&mut rng);

        let result = original.hash_tree_root();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }

    #[test]
    fn test_fork_version() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let fork = Fork::random_for_test(&mut rng);

        let previous = fork.previous_version;
        let current = fork.current_version;
        let epoch = fork.epoch;

        assert_eq!(previous, fork.get_version_for(epoch - 1));
        assert_eq!(current, fork.get_version_for(epoch));
        assert_eq!(current, fork.get_version_for(epoch + 1));
    }
}
