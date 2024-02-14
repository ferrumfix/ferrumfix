/// An indicator for the kind of environment relative to a FIX Connection.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Environment {
    /// Test messages will be ignored or refused under this environment setting.
    Production {
        /// Flag that indicates whether or not test messages should be allowed
        /// in this production environment.
        allow_test: bool,
    },
    /// Production messages will be refused under this environment setting.
    Testing,
}

impl Environment {
    /// Returns `true` if an only if `self` allows test messages, depending on
    /// the provided configuration.
    ///
    /// This is used to determine whether to accept or refuse incoming test
    /// messages within a [`FixConnection`](super::FixConnection).
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::session::Environment;
    ///
    /// assert_eq!(Environment::Testing.allows_testing(), true);
    /// assert_eq!(Environment::Production { allow_test: true }.allows_testing(), true);
    /// assert_eq!(Environment::Production { allow_test: false }.allows_testing(), false);
    /// ```
    pub fn allows_testing(&self) -> bool {
        match self {
            Self::Production { allow_test } => *allow_test,
            Self::Testing => true,
        }
    }
}
