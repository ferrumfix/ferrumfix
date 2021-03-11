/// Configuration interface for [`json::Codec`](Codec).
pub trait Configure: Clone + Default {
    /// This setting indicates that all encoded messages should be "prettified",
    /// i.e. the JSON code will not be compressed and instead it will have
    /// indentation and other whitespace that favors human readability. Some
    /// performance loss and increased payload size is expected.
    ///
    /// This is turned off be default.
    #[inline(always)]
    fn pretty_print(&self) -> bool {
        false
    }
}

/// A [`Configure`](Configure) that "pretty-prints", i.e. always returns `true` from
/// [`Configure::pretty_print`](Configure::pretty_print).
///
/// # Output examples
///
/// With [`ConfigPrettyPrint`]:
///
/// ```json
/// {
///     "Header": {
///         "BeginString": "FIX.4.4",
///         "MsgType": "W",
///         "MsgSeqNum": "4567",
///         "SenderCompID": "SENDER",
///         "TargetCompID": "TARGET",
///         "SendingTime": "20160802-21:14:38.717"
///     },
///     "Body": {
///         "SecurityIDSource": "8",
///         "SecurityID": "ESU6",
///         "MDReqID": "789",
///         "NoMDEntries": [
///             { "MDEntryType": "0", "MDEntryPx": "1.50", "MDEntrySize": "75", "MDEntryTime": "21:14:38.688" },
///             { "MDEntryType": "1", "MDEntryPx": "1.75", "MDEntrySize": "25", "MDEntryTime": "21:14:38.688" }
///         ]
///     },
///     "Trailer": {
///     }
/// }
/// ```
///
/// Without [`ConfigPrettyPrint`]:
///
/// ```json
/// {"Header":{"...":"..."},"Body":{"...":"..."},"Trailer":{}}
/// ```
#[derive(Debug, Clone, Default)]
pub struct ConfigPrettyPrint;

impl Configure for ConfigPrettyPrint {
    #[inline(always)]
    fn pretty_print(&self) -> bool {
        true
    }
}

/// A [`Configure`](Configure) that can be read from a file and modified at runtime.
#[derive(Debug, Clone)]
pub struct Config {
    pretty_print: bool,
}

impl Config {
    /// Creates a [`Config`](Config) with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables [`Configure::pretty_print`](Configure::pretty_print) if and only if
    /// `pretty_print` is true; otherwise it disables pretty-printing.
    pub fn set_pretty_print(&mut self, pretty_print: bool) {
        self.pretty_print = pretty_print;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pretty_print: false,
        }
    }
}

impl Configure for Config {
    fn pretty_print(&self) -> bool {
        self.pretty_print
    }
}
