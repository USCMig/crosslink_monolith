//! Vote participation includes both NIL and value votes.

/// Convert `[prevote, precommit][nil, value]` counts into observed-vote flags.
pub(crate) fn observed_votes(no_yes_counts: [[u64; 2]; 2]) -> [bool; 2] {
    no_yes_counts.map(|[nil, value]| nil > 0 || value > 0)
}

#[cfg(test)]
mod tests {
    use super::observed_votes;

    #[test]
    fn no_votes_are_not_participation() {
        assert_eq!(observed_votes([[0, 0], [0, 0]]), [false, false]);
    }

    #[test]
    fn nil_votes_count_in_their_own_phase() {
        assert_eq!(observed_votes([[1, 0], [0, 0]]), [true, false]);
        assert_eq!(observed_votes([[0, 0], [1, 0]]), [false, true]);
        assert_eq!(observed_votes([[1, 0], [1, 0]]), [true, true]);
    }

    #[test]
    fn value_votes_still_count_in_their_own_phase() {
        assert_eq!(observed_votes([[0, 1], [0, 0]]), [true, false]);
        assert_eq!(observed_votes([[0, 0], [0, 1]]), [false, true]);
        assert_eq!(observed_votes([[0, 1], [0, 1]]), [true, true]);
    }

    #[test]
    fn value_prevote_and_nil_precommit_are_both_participation() {
        assert_eq!(observed_votes([[0, 1], [1, 0]]), [true, true]);
    }

    #[test]
    fn large_counts_do_not_overflow() {
        assert_eq!(observed_votes([[u64::MAX; 2]; 2]), [true, true]);
    }
}
