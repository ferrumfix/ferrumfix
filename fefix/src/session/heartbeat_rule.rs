use super::errs;
use boolinator::Boolinator;
use std::ops::RangeInclusive;
use std::time::Duration;

/// Rules placed on the expected heartbeat interval via out-of-band rules of
/// engatement.
///
/// Please note that [`HeartbeatRule`] is marked with
/// `#[non_exhaustive]`, which future-proofs the enumeration type in case more
/// variants are added.
///
/// Please refer to specs. §4.3.5 for more information.
#[derive(Debug, Clone, Hash)]
#[non_exhaustive]
pub enum HeartbeatRule {
    /// The acceptor requires a specific heartbeat interval, expressed as a
    /// [`Duration`]. Please refer to specs. §4.3.5.1 for
    /// more information.
    Exact(Duration),
    /// The acceptor requires the initiator to specify a heartbeat value within a
    /// [`RangeInclusive`] of
    /// [`Duration`s](Duration). Please refer to specs. §4.3.5.3 for
    /// more information.
    Range(RangeInclusive<Duration>),
    /// The acceptor poses no restrictions on the heartbeat interval and the
    /// initiator can choose any value. Please refer to specs. §4.3.5.3 for more
    /// information.
    Any,
}

impl HeartbeatRule {
    /// Validates an initiator-provided heartbeat value according to the
    /// heartbeat rule represented by `self`.
    ///
    /// # Examples
    ///
    /// Require exact matching with [`HeartbeatRule::Exact`](HeartbeatRule::Exact):
    ///
    /// ```
    /// use fefix::session::HeartbeatRule;
    /// use std::time::Duration;
    ///
    /// let rule = HeartbeatRule::Exact(Duration::from_secs(30));
    /// assert!(rule.validate(&Duration::from_secs(60)).is_err());
    /// assert!(rule.validate(&Duration::from_secs(20)).is_err());
    /// assert!(rule.validate(&Duration::from_secs(30)).is_ok());
    /// ```
    ///
    /// Accepting any proposed heartbeat value with
    /// [`HeartbeatRule::Any`](HeartbeatRule::Any):
    ///
    /// ```
    /// use fefix::session::HeartbeatRule;
    /// use std::time::Duration;
    ///
    /// let rule = HeartbeatRule::Any;
    /// assert!(rule.validate(&Duration::from_secs(1000)).is_ok());
    /// assert!(rule.validate(&Duration::from_secs(1)).is_ok());
    /// ```
    pub fn validate(&self, proposal: &Duration) -> std::result::Result<(), String> {
        match self {
            HeartbeatRule::Exact(expected) => (proposal == expected)
                .ok_or_else(|| errs::heartbeat_exact(expected.as_secs())),
            HeartbeatRule::Range(range) => range.contains(proposal).ok_or_else(|| {
                errs::heartbeat_range(range.start().as_secs(), range.end().as_secs())
            }),
            HeartbeatRule::Any => (*proposal != Duration::from_secs(0))
                .ok_or_else(|| errs::heartbeat_gt_0()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heartebeat_validation() {
        let rule_exact_1 = HeartbeatRule::Exact(Duration::from_secs(1));
        let rule_range_5_30 =
            HeartbeatRule::Range(Duration::from_secs(5)..=Duration::from_secs(30));
        let rule_any = HeartbeatRule::Any;
        assert!(rule_exact_1.validate(&Duration::from_secs(1)).is_ok());
        assert!(!rule_exact_1.validate(&Duration::from_secs(2)).is_ok());
        assert!(!rule_exact_1.validate(&Duration::from_secs(0)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(5)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(10)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(30)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(0)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(4)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(60)).is_ok());
        assert!(rule_any.validate(&Duration::from_secs(1)).is_ok());
        assert!(!rule_any.validate(&Duration::from_secs(0)).is_ok());
    }
}
