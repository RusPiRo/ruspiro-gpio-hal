

/// The different GPIO events, a handler (function or closure) can be registered for. See also
/// [Gpio::register_event_handler_always] and [Gpio::register_event_handler_onetime].
pub enum GpioEvent {
    /// Event triggered when the level changes from low to high
    RisingEdge,
    /// Event triggered when the level changes from high to low
    FallingEdge,
    /// Event triggerd when the level changes from low to high or high to low
    BothEdges,
    /// Event riggered as long as the pin level is high
    High,
    /// Event riggered as long as the pin level is low
    Low,
    /// Event triggered when the level changes from low to high, but the detection is not bound
    /// to the GPIO clock rate and allows for faster detections
    AsyncRisingEdge,
    /// Event triggered when the level changes from high to low, but the detection is not bound
    /// to the GPIO clock rate and allows for faster detections
    AsyncFallingEdge,
    /// Event triggered when the level changes from high to low or low to high, but the detection is
    /// not bound to the GPIO clock rate and allows for faster detections
    AsyncBothEdges,
}
