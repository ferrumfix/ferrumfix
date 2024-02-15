use super::{Environment, MsgSeqNumCounter, SeqNumbers};
use std::num::NonZeroU64;
use std::time::Duration;

/// Collection of configuration options related to
/// [`FixConnection`](super::FixConnection).
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`. Implementors MUST
/// have [`Default`] behavior that is consistent with the provided default
/// return values of this trait.
pub trait Configure: Clone + Default {
    /// Asks the FIX connector to verify `TestMessageIndicator <464>` fields and
    /// refuse messages if the value is invalid. `true` by default.
    fn verify_test_indicator(&self) -> bool {
        true
    }

    /// Returns the maximum allowed latency based on the sending time. Three
    /// seconds by default.
    fn max_allowed_latency(&self) -> Duration {
        Duration::from_secs(3)
    }

    fn begin_string(&self) -> &[u8] {
        b"FIX.4.4"
    }

    fn sender_comp_id(&self) -> &[u8] {
        b"SENDER_COMP"
    }

    fn target_comp_id(&self) -> &[u8] {
        b"TARGET_COMP"
    }

    fn environment(&self) -> Environment {
        Environment::Production { allow_test: true }
    }

    fn heartbeat(&self) -> Duration {
        Duration::from_secs(30)
    }
}

/// The canonical implementor of [`Configure`]. Every setting can be changed.
/// Most fields simply mirror the methods of the [`Configure`] trait.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
#[non_exhaustive]
pub struct Config {
    pub verify_test_indicator: bool,
    pub max_allowed_latency: Duration,
    /// The FIX version string. Defaults to `FIX.4.4`.
    pub begin_string: String,
    pub environment: Environment,
    pub heartbeat: Duration,
    pub seq_numbers: SeqNumbers,
    pub msg_seq_num_inbound: MsgSeqNumCounter,
    pub msg_seq_num_outbound: MsgSeqNumCounter,
    pub sender_comp_id: String,
    pub target_comp_id: String,
}

impl Configure for Config {
    fn verify_test_indicator(&self) -> bool {
        self.verify_test_indicator
    }

    fn max_allowed_latency(&self) -> Duration {
        self.max_allowed_latency
    }

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_bytes()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_bytes()
    }

    fn begin_string(&self) -> &[u8] {
        self.begin_string.as_bytes()
    }

    fn environment(&self) -> Environment {
        self.environment
    }

    fn heartbeat(&self) -> Duration {
        self.heartbeat
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verify_test_indicator: true,
            max_allowed_latency: Duration::from_secs(3),
            begin_string: "FIX.4.4".to_string(),
            environment: Environment::Production { allow_test: true },
            heartbeat: Duration::from_secs(30),
            seq_numbers: SeqNumbers::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(1).unwrap()),
            msg_seq_num_inbound: MsgSeqNumCounter::START,
            msg_seq_num_outbound: MsgSeqNumCounter::START,
            sender_comp_id: "SENDER_COMP".to_string(),
            target_comp_id: "TARGET_COMP".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[derive(Default, Clone)]
    struct ConfigDefault;

    impl Configure for ConfigDefault {}

    #[test]
    fn config_defaults() {
        let config = Config::default();
        assert_eq!(
            config.max_allowed_latency(),
            ConfigDefault.max_allowed_latency()
        );
        assert_eq!(
            config.verify_test_indicator(),
            ConfigDefault.verify_test_indicator()
        );
    }

    #[quickcheck]
    fn config_set_max_allowed_latency(latency: Duration) -> bool {
        let mut config = Config::default();
        config.max_allowed_latency = latency;
        config.max_allowed_latency() == latency
    }

    #[quickcheck]
    fn config_set_verify_test_indicator(verify: bool) -> bool {
        let mut config = Config::default();
        config.verify_test_indicator = verify;
        config.verify_test_indicator() == verify
    }
}
