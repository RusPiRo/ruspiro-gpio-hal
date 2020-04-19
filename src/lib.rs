/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Appache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-gpio-hal/0.1.0")]
#![cfg_attr(not(any(test, doctest)), no_std)]
//! # GPIO Hardware Abstraction Layer
//!
//! This provided traits allowsto abstract the specific implementation of the GPIO peripheral access that
//! will be different on each specific embedded system. The provided function can be implemented individual the target /
//! hardware but keeps the implementation details away from crates that wants to use the GPIO peripheral for the 
//! functionality they provide.
//! 
//! One implementation for the Raspberry Pi could be found [here](https://crates.io/crates/ruspiro-gpio)
//!

extern crate alloc;
use alloc::boxed::Box;
extern crate ruspiro_core;
use ruspiro_core::error::*;

/// The representation of a generic GPIO PIN
pub trait GpioPin {
    /// return the identifier of this [GpioPin]
    fn id(&self) -> u32;

    /// re-configure the [GpioPin] as an Input pin. This is a stateful operation at the hardware layer
    /// so even if the [GpioPin] get's out of scope this setting remains valid
    /// TODO: verify if this is a valid/desired appraoch
    fn into_input(self) -> Box<dyn GpioPinInput>;

    /// re-configure the [GpioPin] as an Output pin. This is a stateful operation at the hardware layer
    /// so even if the [GpioPin] get's out of scope this setting remains valid
    /// TODO: verify if this is a valid/desired appraoch
    fn into_output(self) -> Box<dyn GpioPinOutput>;

    /// re-configure the [GpioPin] with an alternative function. This is a stateful operation at the hardware layer
    /// so even if the [GpioPin] get's out of scope this setting remains valid.
    /// If a specific hardware dow not support the requested alternative function it shall return an [Err]
    /// TODO: verify if this is a valid/desired appraoch
    fn into_altfunc(self, function: u8) -> Result<Box<dyn GpioPinAltFunc>, BoxError>;

    /// Diable the pull-up/down settings for this [GpioPin].
    fn disable_pud(&self);
    
    /// Enable the pull-up settings for this [GpioPin].
    fn enable_pud_up(&self);

    /// Enable the pull-down settings for this [GpioPin].
    fn enable_pud_down(&self);
}

/// The representation of an input GPIOPin
pub trait GpioPinInput: GpioPin {
    /// Reads the actual level of the [GpioPin] and returns [true] if it is high.
    fn is_high(&self) -> bool;

    /// Reads the actual level of the [GpioPin] and returns [true] if it is low.
    fn is_low(&self) -> bool { !self.is_high() }
}

/// The representation of an output GPIOPin
pub trait GpioPinOutput: GpioPin {
    /// Set the output level of the [GpioPin] to high
    fn high(&self);

    /// Set the output level of the [GpioPin] to low
    fn low(&self);

    /// Toggle the output level of the [GpioPin] either from low -> high or from high -> low
    fn toggle(&self);
}

/// The representation of an GPIOPin with alternative function. The meaning of the function is usually specified within
/// the peripheral documentation of the hardware for which this will be implemented.
pub trait GpioPinAltFunc: GpioPin {}

/// This trait provides access to the GPIO hardware present in embedded systems.
pub trait Gpio {
    /// Request a [GpioPin] from the GPIO peripheral for further usage. It returns a pin with initially undefined
    /// function and pud settings. If the requested pin is already in use the implementation shall return a respective
    /// [Error].
    fn use_pin(&mut self, id: u32) -> Result<Box<dyn GpioPin>, BoxError>;

    /// Release a [GpioPin] that has been in use previously. If the pin to be released has not been in use the
    /// implementation shall return an [Error]. Even though it might not be a real error as such it allows the caller to
    /// properly handle this state.
    fn release_pin(&mut self, id: u32) -> Result<(), BoxError>;

    /// Register an event handler that will be called whenever the given event is detected for the [GpioPin]. The
    /// [GpioPin] need to be configured as [GpioPinInput]. As the specified events quite likely are triggerd from an
    /// interrupt the implememter of this function need to ensure that the corresponding interrupts are enabled and
    /// activated.
    fn register_event_handler_always(
        &mut self,
        gpio_pin: &dyn GpioPinInput,
        event: GpioEvent,
        handler: Box<dyn FnMut() + 'static + Send>,
    );

    /// Register an event handler that will be called only once for the next occurance of the given event is detected
    /// for the [GpioPin]. The [GpioPin] need to be configured as [GpioPinInput]. As the specified events quite
    /// likely are triggerd from an interrupt the implememter of this function need to ensure that the corresponding
    /// interrupts are enabled and activated.
    fn register_event_handler_onetime(
        &mut self,
        gpio_pin: &dyn GpioPinInput,
        event: GpioEvent,
        handler: Box<dyn FnOnce() + 'static + Send>,
    );

    /// Unregister an event handler of the given type for the [GpioPin].
    fn unregister_event_handler(&mut self, gpio_pin: &dyn GpioPin, event: GpioEvent);
}

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
