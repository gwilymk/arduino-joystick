# Arduino joystick

This repo allows you to emulate a joystick via an arduino connected via a serial port (on Linux only).
I put this together for my raspberry pi based games console, in order to avoid having to attach a usb joystick.

This repo comes with an accompanying blog post [here](https://gwilym.dev/2021/02/virtual-joystick-on-linux/).

# Building

You will need the arduino IDE to compile and upload the firmware and the rust toolchain in order to build the driver.

The `joystick-firmware` directory contains the required code which you need to flash to an arduino of your choice.
You can change the supported buttons, but there should be no more than 15 of them.

You will also need to update the corresponding buttons in buttons.rs if you change them in the firmware.

The driver can be compiled by doing `cargo build --release` in the `joystick-daemon` directory.
The resulting binary in `target/release` is what you need to run in order to connect this joystick.

# Building for a raspberry pi

Since this was intended for a raspberry pi build, you could either build this directly on a pi, or do a cross compile.

## Cross compiling

Go to the `joystick-daemon/build-images` directory and build the crossbuild image.

```sh
$ docker build . -t crossbuild:local
```

The image _must_ be called `crossbuild:local`.

Once you've done that, go back to the `joystick-daemon` directory and using the cargo cross tool, run

```sh
$ cross build --target armv7-unknown-linux-gnueabihf --release
```

You can find the `cross` command here: https://github.com/rust-embedded/cross

The resulting executable can be found in `target/armv7-unknown-linux-gnueabihf/release/joystick-daemon`