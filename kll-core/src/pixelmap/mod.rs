// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// TODO
// - Multiple buffers
//   * Dis-similar buffer sizes
//     * ISSI + misc RGB leds + single color leds
//   * Compile-time allocation
// - Direct-drive interface for HID-IO
// - Fade groups
//   * Should we utilize hardware for this? (less memory, more SPI transfers; will need to work
//   with whitebalance configurations)
//   * Should we utilize the existing memory buffer approach? (more memory, more copies)
// - Pixel mapping
//   * bit width
//   * number of channels
//   * channel lookup index for each channel
// - Pixel display mapping (this is the crude calculation for the x/y grid used for rows and
// columns)
// - Animations
//   * Frame data
//     + Operation address format
//     + Address
//     + List of operations for each pixel channel (e.g. set r,g,b to 12,45,38)
//   * Possible settings profiles
//     + index (animation)
//     + position (frame position of animation)
//     + sub position
//     + loops
//     + frame delay
//     + frame option
//     + ffunc
//     + pfunc
//     + replace
//     + state
// - Scan Code to Pixel
// - Scan Code to Display
// - Pixel physical positions
//
//
// Handled by top-level
// - Rate limiting
//   * Use a timer to handle frame processing
//   * Add control to adjust the timer (maybe?)
//
// Handled by LED driver
// - gamma
//   * compute table at startup?
//   * compile-time lookup?
// - white balance
//   * will require multiple lookup tables
//   * and a lookup table to map each channel to the required lookup table
//   * Compile-time
//     + Specify number of tables needed
//     + Specify channel intensity divider?
//       See https://www.nichia.co.jp/specification/products/led/ApplicationNote_SE-AP00042-E.pdf
//       Maybe use luminous intensity ratios? Maybe it's easier just to use a divider (can be
//       fractional as long as the result is in integer)
//   * Or use the scaling table to just hard code the white balance (probably the easiest)

pub mod pixel {
    #[derive(Copy, Clone, Debug, PartialEq, defmt::Format)]
    #[repr(u8)]
    pub enum Configuration {
        /// No pixel defined
        NoPixel = 0,
        /// 1 channel 1-bit on/off
        Single1Bit = 1,
        /// 1 channel 8-bit brightness
        Single8Bit = 8,
        /// 1 channel 16-bit brightness
        Single16Bit = 16,
        /// 3 channel 8-bit RGB
        Rgb8Bit = 24,
        /// 3 channel 16-bit RGB
        Rgb16Bit = 48,
    }

    pub struct Mapping<const MAX_CHANNELS: usize> {
        config: Configuration,
        indices: [u16; MAX_CHANNELS],
    }
}

pub mod display {
    // TODO Needed?
}

pub mod fade {
}

pub mod animation {
}


