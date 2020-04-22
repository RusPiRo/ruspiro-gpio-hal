/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Appache License 2.0
 **********************************************************************************************************************/

//! # GPIO Pin
//! 
//! HAL traits representing a single pin of the GPIO peripheral
//! 

use alloc::boxed::Box;
use ruspiro_error::*;

/// The representation of a generic GPIO PIN
pub trait HalGpioPin {
    /// return the identifier of this [HalGpioPin]
    fn id(&self) -> u32;

    /// re-configure the [HalGpioPin] as an Input pin. This is a stateful operation at the hardware layer
    /// so even if the [HalGpioPin] get's out of scope this setting remains valid
    /// TODO: verify if this is a valid/desired appraoch
    fn into_input(self: Box<Self>) -> Box<dyn HalGpioPinInput>;

    /// re-configure the [HalGpioPin] as an Output pin. This is a stateful operation at the hardware layer
    /// so even if the [HalGpioPin] get's out of scope this setting remains valid
    /// TODO: verify if this is a valid/desired appraoch
    fn into_output(self: Box<Self>) -> Box<dyn HalGpioPinOutput>;

    /// re-configure the [HalGpioPin] with an alternative function. This is a stateful operation at the hardware layer
    /// so even if the [HalGpioPin] get's out of scope this setting remains valid.
    /// If a specific hardware dow not support the requested alternative function it shall return an [Err]
    /// TODO: verify if this is a valid/desired appraoch
    fn into_altfunc(self: Box<Self>, function: u8) -> Result<Box<dyn HalGpioPinAltFunc>, BoxError>;

    /// Diable the pull-up/down settings for this [HalGpioPin].
    fn disable_pud(&self);
    
    /// Enable the pull-up settings for this [HalGpioPin].
    fn enable_pud_up(&self);

    /// Enable the pull-down settings for this [HalGpioPin].
    fn enable_pud_down(&self);
}

/// The representation of an input GPIOPin
pub trait HalGpioPinInput: HalGpioPin {
    /// Reads the actual level of the [HalGpioPin] and returns [true] if it is high.
    fn is_high(&self) -> bool;

    /// Reads the actual level of the [HalGpioPin] and returns [true] if it is low.
    fn is_low(&self) -> bool { !self.is_high() }
}

/// The representation of an output GPIOPin
pub trait HalGpioPinOutput: HalGpioPin {
    /// Set the output level of the [HalGpioPin] to high
    fn high(&self);

    /// Set the output level of the [HalGpioPin] to low
    fn low(&self);

    /// Toggle the output level of the [HalGpioPin] either from low -> high or from high -> low
    fn toggle(&self);
}

/// The representation of an GPIOPin with alternative function. The meaning of the function is usually specified within
/// the peripheral documentation of the hardware for which this will be implemented.
pub trait HalGpioPinAltFunc: HalGpioPin {}