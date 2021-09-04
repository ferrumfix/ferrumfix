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
#[derive(Debug, Clone)]
pub struct Config {
    verify_test_indicator: bool,
    max_allowed_latency: Duration,
    begin_string: String,
    environment: Environment,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
    sender_comp_id: String,
    target_comp_id: String,
}

// The reason why we're using setters rather than making `struct` fields `pub`
// is so that we can add more configuration options without breaking backwards
// compatibility.
impl Config {
    /// Changes the value of [`Configure::verify_test_indicator`].
    pub fn set_verify_test_indicator(&mut self, verify: bool) {
        self.verify_test_indicator = verify;
    }

    /// Changes the value of [`Configure::max_allowed_latency`].
    pub fn set_max_allowed_latency(&mut self, max_allowed_latency: Duration) {
        self.max_allowed_latency = max_allowed_latency;
    }

    pub fn set_begin_string<S>(&mut self, begin_string: S)
    where
        S: Into<String>,
    {
        self.begin_string = begin_string.into();
    }

    /// Sets the [`Environment`] associated with this [`FixConnection`].
    pub fn set_environmen(&mut self, env: Environment) {
        self.environment = env;
    }

    /// Provides the initial sequence numbers associated with this
    /// [`FixConnection`]. These are both set to 1 by default. You will need to
    /// change this if you plan on reconnecting to a previous [`FixConnection`].
    ///
    /// # Panics
    ///
    /// Inputs must be strictly positive. Panics if either `inbound` or
    /// `outbound` is equal to 0.
    pub fn set_seq_numbers(&mut self, inbound: u64, outbound: u64) {
        if inbound == 0 || outbound == 0 {
            panic!("FIX sequence numbers must be strictly positive");
        }
        self.seq_numbers = SeqNumbers {
            next_inbound: inbound,
            next_outbound: outbound,
        };
    }

    pub fn set_sender_comp_id<S>(&mut self, sender_comp_id: S)
    where
        S: Into<String>,
    {
        self.sender_comp_id = sender_comp_id.into();
    }

    pub fn set_target_comp_id<S>(&mut self, target_comp_id: S)
    where
        S: Into<String>,
    {
        self.target_comp_id = target_comp_id.into();
    }
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
            msg_seq_num_inbound: MsgSeqNumCounter(1),
            msg_seq_num_outbound: MsgSeqNumCounter(1),
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
        config.set_max_allowed_latency(latency);
        config.max_allowed_latency() == latency
    }

    #[quickcheck]
    fn config_set_verify_test_indicator(verify: bool) -> bool {
        let mut config = Config::default();
        config.set_verify_test_indicator(verify);
        config.verify_test_indicator() == verify
    }
}
