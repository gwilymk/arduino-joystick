mod axis;
mod button;
mod error;

pub use axis::Axis;
pub use button::Button;
pub use error::Error;

use input_linux::sys;

use std::{fs, path};

pub struct Joystick {
    device: input_linux::UInputHandle<fs::File>,
}

impl Joystick {
    pub fn new() -> Result<Self, Error> {
        let device = create_joystick_device()?;

        Ok(Joystick { device })
    }

    pub fn device_path(&self) -> Result<path::PathBuf, Error> {
        Ok(self.device.evdev_path()?)
    }

    pub fn move_axis(&self, axis: Axis, position: i32) -> Result<(), Error> {
        if position > 512 || position < -512 {
            return Err(Error::OutOfRangeError {
                min: -512,
                max: 512,
                actual: position,
            });
        }

        self.write_event(input_linux::AbsoluteEvent::new(
            empty_event_time(),
            axis.to_evdev_axis(),
            position,
        ))
    }

    pub fn button_press(&self, button: Button, is_pressed: bool) -> Result<(), Error> {
        let value = if is_pressed {
            input_linux::KeyState::PRESSED
        } else {
            input_linux::KeyState::RELEASED
        };

        self.write_event(input_linux::KeyEvent::new(
            empty_event_time(),
            button.to_evdev_button(),
            value,
        ))
    }

    pub fn synchronise(&self) -> Result<(), Error> {
        self.write_event(input_linux::SynchronizeEvent::report(empty_event_time()))
    }

    fn write_event(&self, event: impl std::convert::AsRef<sys::input_event>) -> Result<(), Error> {
        self.device.write(&[*event.as_ref()])?;

        Ok(())
    }
}

fn empty_event_time() -> input_linux::EventTime {
    input_linux::EventTime::new(0, 0)
}

fn create_joystick_device() -> Result<input_linux::UInputHandle<fs::File>, Error> {
    let uinput_file = fs::File::create("/dev/uinput")?;
    let device = input_linux::UInputHandle::new(uinput_file);

    let input_id = input_linux::InputId {
        bustype: sys::BUS_VIRTUAL,
        vendor: 34,
        product: 10,
        version: 1,
    };

    let standard_info = input_linux::AbsoluteInfo {
        value: 0,
        minimum: -512,
        maximum: 512,
        fuzz: 0,
        flat: 0,
        resolution: 50,
    };

    device.set_evbit(input_linux::EventKind::Absolute)?;
    device.set_evbit(input_linux::EventKind::Key)?;
    device.set_keybit(input_linux::Key::ButtonTrigger)?; // informs linux that this is a joystick

    for button in Button::all_buttons() {
        device.set_keybit(button.to_evdev_button())?;
    }

    device.create(
        &input_id,
        b"arduino-virtual-joystick",
        0,
        &Axis::all_axes()
            .map(|axis| input_linux::AbsoluteInfoSetup {
                axis: axis.to_evdev_axis(),
                info: standard_info,
            })
            .collect::<Vec<_>>(),
    )?;

    Ok(device)
}
