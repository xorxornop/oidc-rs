/// Configuration for events, e.g. which may be raised.
#[derive(Debug)]
pub(crate) struct EventsConfig {
    /// Whether to raise success events.
    raise_success_events: bool,

    /// Whether to raise failure events.
    raise_failure_events: bool,

    /// Whether to raise information events.
    raise_information_events: bool,

    /// Whether to raise error events.
    raise_error_events: bool,
}