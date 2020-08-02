# weighty
Weighty is a Rust library for reading from HID scales.

We expose two APIs, one that depends on `uom` and one that does not.  `uom` is nice
in that it gives better static guarantees that you're not crashing a climate orbiter,
but it carries a _huge_ penalty to your compilation time.  I find the trade worth it,
but you may not, either way I've got you covered.  To build with `uom` support,
use the `units` feature.

In addition to the library, weighty also includes the `weigh` executable that polls
each attached HID scale and prints their readings to standard out.

# Supported Hardware

* Dymo M10
* Others?  (try it and let me know)

# How you can help!

Testing!  I only own a Dymo M10, so it's all I can test against.  If you have access
to another HID scale please help by testing this code against it and submitting
either a PR that updates the above list of supported hardware, or a bug report.

Additionally, `weigh` is pretty bare bones right now, it reads what it can and
prints it.  Feature requests welcome!

There might be a more elegant way to do the udev rules.  Right now I'm granting
0666 to all USB devices with Dymo's vendor ID, but a wizard may be able to express
a rule that applies to all devices that support usage page 0x8D.  OTOH, the wizard may
also be frustrated by vendors that don't correctly report their usage page (needless
to say, this is my suspicion in this case).

# `weigh` Installation

    cargo install weighty --features="units"

Note: in order for this to work your user needs access to the scale device.  In
Linux this is typically done via udev.  Example udev rules files can be found in
the udev directory.
