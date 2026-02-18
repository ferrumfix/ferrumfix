use super::{Environment, HeartbeatRule, SeqNumbers};
use std::time::Duration;

/// Role assumed by the session engine at connection start.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SessionRole {
    /// Send `Logon <A>` first.
    Initiator,
    /// Wait for an incoming `Logon <A>` before sending one back.
    Acceptor,
}

/// Collection of configuration options related to [`FixConnection`](super::FixConnection).
///
/// Implementors must provide defaults that are coherent with the trait default
/// methods.
pub trait Configure: Clone + Default {
    /// Session role.
    fn role(&self) -> SessionRole {
        SessionRole::Initiator
    }

    /// Asks the session engine to verify `TestMessageIndicator <464>` fields.
    fn verify_test_indicator(&self) -> bool {
        true
    }

    /// Asks the session engine to verify `SendingTime <52>` freshness.
    fn verify_sending_time(&self) -> bool {
        true
    }

    /// Returns the maximum allowed latency based on `SendingTime <52>`.
    fn max_allowed_latency(&self) -> Duration {
        Duration::from_secs(3)
    }

    /// Whether to enforce `BeginString <8>` exact matching.
    fn enforce_begin_string(&self) -> bool {
        true
    }

    /// Whether to enforce `SenderCompID <49>` / `TargetCompID <56>` checks.
    fn enforce_comp_id(&self) -> bool {
        true
    }

    /// Begin string used for outbound session messages.
    fn begin_string(&self) -> &[u8] {
        b"FIX.4.4"
    }

    /// Environment mode.
    fn environment(&self) -> Environment {
        Environment::Production { allow_test: true }
    }

    /// Default heartbeat interval used before negotiation completes.
    fn heartbeat(&self) -> Duration {
        Duration::from_secs(30)
    }

    /// Validation rule applied to peer's negotiated heartbeat (`HeartBtInt <108>`).
    fn heartbeat_rule(&self) -> HeartbeatRule {
        HeartbeatRule::Exact(self.heartbeat())
    }

    /// Initial sequence numbers.
    fn seq_numbers(&self) -> SeqNumbers {
        SeqNumbers::default()
    }
}

/// Canonical implementor of [`Configure`].
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct Config {
    pub role: SessionRole,
    pub verify_test_indicator: bool,
    pub verify_sending_time: bool,
    pub max_allowed_latency: Duration,
    pub enforce_begin_string: bool,
    pub enforce_comp_id: bool,
    pub begin_string: String,
    pub environment: Environment,
    pub heartbeat: Duration,
    pub heartbeat_rule: HeartbeatRule,
    pub seq_numbers: SeqNumbers,
}

impl Configure for Config {
    fn role(&self) -> SessionRole {
        self.role
    }

    fn verify_test_indicator(&self) -> bool {
        self.verify_test_indicator
    }

    fn verify_sending_time(&self) -> bool {
        self.verify_sending_time
    }

    fn max_allowed_latency(&self) -> Duration {
        self.max_allowed_latency
    }

    fn enforce_begin_string(&self) -> bool {
        self.enforce_begin_string
    }

    fn enforce_comp_id(&self) -> bool {
        self.enforce_comp_id
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

    fn heartbeat_rule(&self) -> HeartbeatRule {
        self.heartbeat_rule.clone()
    }

    fn seq_numbers(&self) -> SeqNumbers {
        self.seq_numbers
    }
}

impl Default for Config {
    fn default() -> Self {
        let heartbeat = Duration::from_secs(30);
        Self {
            role: SessionRole::Initiator,
            verify_test_indicator: true,
            verify_sending_time: true,
            max_allowed_latency: Duration::from_secs(3),
            enforce_begin_string: true,
            enforce_comp_id: true,
            begin_string: "FIX.4.4".to_string(),
            environment: Environment::Production { allow_test: true },
            heartbeat,
            heartbeat_rule: HeartbeatRule::Exact(heartbeat),
            seq_numbers: SeqNumbers::default(),
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
    fn config_defaults_match_trait_defaults() {
        let config = Config::default();
        assert_eq!(config.role(), ConfigDefault.role());
        assert_eq!(
            config.max_allowed_latency(),
            ConfigDefault.max_allowed_latency()
        );
        assert_eq!(
            config.verify_test_indicator(),
            ConfigDefault.verify_test_indicator()
        );
        assert_eq!(
            config.verify_sending_time(),
            ConfigDefault.verify_sending_time()
        );
        assert_eq!(
            config.enforce_begin_string(),
            ConfigDefault.enforce_begin_string()
        );
        assert_eq!(config.enforce_comp_id(), ConfigDefault.enforce_comp_id());
        assert_eq!(config.heartbeat(), ConfigDefault.heartbeat());
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
