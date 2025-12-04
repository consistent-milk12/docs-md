*[rustix](../index.md) / [termios](index.md)*

---

# Module `termios`

Terminal I/O stream operations.

This API automatically supports setting arbitrary I/O speeds, on any
platform that supports them, including Linux and the BSDs.

The [`speed`](#speed) module contains various predefined speed constants which are
more likely to be portable, however any `u32` value can be passed to
[`Termios::set_speed`](#set-speed), [`Termios::set_input_speed`](#set-input-speed), and
[`Termios::set_output_speed`](#set-output-speed), and they will simply fail if the speed is
not supported by the platform or the device.

