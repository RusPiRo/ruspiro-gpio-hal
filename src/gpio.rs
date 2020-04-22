/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Appache License 2.0
 **********************************************************************************************************************/

//! # GPIO Peripheral
//! 
//! HAL Trait of a GPIO peripheral device for embedded systems.
//! 

use alloc::boxed::Box;
use ruspiro_error::*;
use super::pin::*;
use super::events::*;

/// This trait provides access to the GPIO hardware present in embedded systems.
pub trait HalGpio {
    /// Request a [HalGpioPin] from the GPIO peripheral for further usage. It returns a pin with initially undefined
    /// function and pud settings. If the requested pin is already in use the implementation shall return a respective
    /// [Error].
    fn use_pin(&mut self, id: u32) -> Result<Box<dyn HalGpioPin>, BoxError>;

    /// Release a [GpioPin] that has been in use previously. If the pin to be released has not been in use the
    /// implementation shall return an [Error]. Even though it might not be a real error as such it allows the caller to
    /// properly handle this state.
    fn release_pin(&mut self, id: u32) -> Result<(), BoxError>;

    /// Register an event handler that will be called whenever the given event is detected for the [GpioPin]. The
    /// [HalGpioPin] need to be configured as [HalGpioPinInput]. As the specified events quite likely are triggerd from an
    /// interrupt the implememter of this function need to ensure that the corresponding interrupts are enabled and
    /// activated.
    fn register_event_handler_always(
        &mut self,
        gpio_pin: &dyn HalGpioPinInput,
        event: GpioEvent,
        handler: Box<dyn FnMut() + 'static + Send>,
    );

    /// Register an event handler that will be called only once for the next occurance of the given event is detected
    /// for the [HalGpioPin]. The [HalGpioPin] need to be configured as [HalGpioPinInput]. As the specified events quite
    /// likely are triggerd from an interrupt the implememter of this function need to ensure that the corresponding
    /// interrupts are enabled and activated.
    fn register_event_handler_onetime(
        &mut self,
        gpio_pin: &dyn HalGpioPinInput,
        event: GpioEvent,
        handler: Box<dyn FnOnce() + 'static + Send>,
    );

    /// Unregister an event handler of the given type for the [HalGpioPin].
    fn unregister_event_handler(&mut self, gpio_pin: &dyn HalGpioPin, event: GpioEvent);
}