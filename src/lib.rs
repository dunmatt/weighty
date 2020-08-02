//! Weighty is a simple library for reading from USB HID scales, as well as an
//! example utility that consumes the library and performs one reading on all
//! available scales.
//!
//! If you have a little compilation time to spare, I recommend using the `units`
//! feature to enable the `read` method on the drivers, which prevents you from
//! bungling the units... not that you would... but I have.
//!
//! Example:
//! ```
//! use weighty;
//!
//! for scale in weighty::get_all_scales() {
//!     println!("{:?}kgf", scale.read_kilograms());
//! }
//! ```
//!

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use hidapi::{HidApi, HidDevice};
#[cfg(feature = "units")]
use uom::si::f64::Force;

mod dymo;

/// A list of things that can go wrong when reading from a digital scale.
#[derive(Clone, Debug)]
pub enum HidScaleError {
    /// For some reason we can't read from the scale.
    CantReadDueTo(String),
    /// For some reason the scale didn't send the expected number of bytes.  If
    /// you see this error please file a bug report.
    NotEnoughData,
    /// The scale has more weight on it than it can handle / measure.
    Overloaded,
    /// For some reason the scale is reporting units that we don't understand.
    /// If you see this error please file a bug report.
    UnknownUnits,
    /// Some scales, for example Dymo scales, cannot send back weight differences
    /// (as you would compute using the tare feature).  If the scale is reporting
    /// such a condition, this is the error you'll get.
    UnreportableReading,
}

/// Type alias to simplify signatures.
pub type Result<T> = std::result::Result<T, HidScaleError>;

/// A ScaleDriver takes care of decoding and making sense of the raw bytes
/// coming from the HID scale.
pub trait ScaleDriver {
    #[cfg(feature = "units")]
    /// Returns the amount of force (weight) currently bearing on the scale.
    fn read(&self) -> Result<Force>;

    /// Returns the number of kilograms-force currently bearing on the scale.
    fn read_kilograms(&self) -> Result<f64>;

    /// Returns the number of pounds of force currently bearing on the scale.
    fn read_pounds(&self) -> Result<f64>;
}

/// Returns a collection of drivers for all of the currently available scales.
///
/// If this is not returning a scale that you expect to see, double check that
/// your user has permissions to access the device.  You might need to set up a
/// udev rule and then reconnect your device.
pub fn get_all_scales() -> Vec<Box<dyn ScaleDriver>> {
    let api = HidApi::new().expect("Couldn't aquire the HID API???");
    api.device_list().filter_map(|info| {
        let vendor_id = info.vendor_id();
        let product_id = info.product_id();
        info.open_device(&api).ok().and_then(|dev| make_driver(vendor_id, product_id, dev))
    }).collect()
}

/// Returns all of one specific type of scale that are currently connected.
///
/// If this is not returning a scale that you expect to see, double check that
/// your user has permissions to access the device.  You might need to set up a
/// udev rule and then reconnect your device.
pub fn get_scales_by_usb_id(vendor_id: u16, product_id: u16) -> Vec<Box<dyn ScaleDriver>> {
    let api = HidApi::new().expect("Couldn't aquire the HID API???");
    api.device_list().filter_map(|info| {
        let vid = info.vendor_id();
        let pid = info.product_id();
        if vid != vendor_id || pid != product_id {
            return None;
        }
        info.open_device(&api).ok().and_then(|dev| make_driver(vendor_id, product_id, dev))
    }).collect()
}

/// Returns one specific scale, if it is connected and we have access to it.
pub fn get_scale_by_serial_number(serial_number: &str) -> Option<Box<dyn ScaleDriver>> {
    let api = HidApi::new().expect("Couldn't aquire the HID API???");
    let result = api.device_list().filter_map(|info| {
        let vendor_id = info.vendor_id();
        let product_id = info.product_id();

        info.open_device(&api).ok().and_then(|dev|
            match dev.get_serial_number_string() {
                Ok(Some(s)) if s == serial_number => make_driver(vendor_id, product_id, dev),
                _ => None,
            }
        )
    }).next();
    result
}

fn make_driver(vendor_id: u16, product_id: u16, device: HidDevice) -> Option<Box<dyn ScaleDriver>> {
    match vendor_id {
        dymo::VENDOR_ID => dymo::make_driver(product_id, device),
        _ => None,
    }
}
