use std::slice;

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    RX,
    RY,
}

impl Axis {
    pub(super) fn to_evdev_axis(&self) -> input_linux::AbsoluteAxis {
        use Axis::*;

        match &self {
            X => input_linux::AbsoluteAxis::X,
            Y => input_linux::AbsoluteAxis::Y,
            RX => input_linux::AbsoluteAxis::RX,
            RY => input_linux::AbsoluteAxis::RY,
        }
    }

    pub(super) fn all_axes() -> slice::Iter<'static, Self> {
        use Axis::*;
        [X, Y, RX, RY].iter()
    }
}
