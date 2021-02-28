use std::{error, fmt, io, path, time};

pub struct SerialConnection {
    port: Box<dyn serialport::SerialPort>,
}

impl SerialConnection {
    pub fn new(port: &path::PathBuf, baud: u32) -> Result<Self, Error> {
        let settings = serialport::SerialPortSettings {
            baud_rate: baud,
            data_bits: serialport::DataBits::Eight,
            flow_control: serialport::FlowControl::None,
            parity: serialport::Parity::None,
            stop_bits: serialport::StopBits::One,
            timeout: time::Duration::from_millis(20),
        };

        let port = serialport::open_with_settings(port, &settings)?;

        Ok(SerialConnection { port })
    }

    pub fn read_button_state(&mut self) -> Result<ButtonState, Error> {
        const PACKET_LENGTH: usize = 12;
        let mut buffer: [u8; PACKET_LENGTH] = [0; PACKET_LENGTH];
        loop {
            match self.read_full_buffer(&mut buffer) {
                Ok(()) => {
                    if buffer[0] == u8::MAX && buffer[1] == u8::MAX {
                        // start of packet marker found
                        return Ok(ButtonState::from_buffer(&buffer[2..]));
                    }

                    let mut skip = None;
                    // need to find the starting marker
                    for i in 1..buffer.len() {
                        if buffer[i] == u8::MAX {
                            if i == buffer.len() - 1 {
                                // starting marker is split across the start and end?
                                skip = Some(PACKET_LENGTH - 1);
                                break;
                            } else if buffer[i + 1] == u8::MAX {
                                // found the starting marker!
                                skip = Some(i);
                                break;
                            }
                        }
                    }

                    let skip = skip.unwrap_or(1);

                    let mut buffer = vec![0; skip];
                    match self.port.read(buffer.as_mut_slice()) {
                        Ok(_) => continue,
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                        Err(e) => return Err(e.into()),
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    println!("Timed out: {}", e);
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn read_full_buffer(&mut self, buf: &mut [u8]) -> Result<(), io::Error> {
        let mut read = 0;
        loop {
            let t = self.port.read(&mut buf[read..])?;

            if t + read == buf.len() {
                return Ok(());
            }

            read += t;
        }
    }
}

pub struct ButtonState {
    pub pressed: [bool; 14],
    pub joysticks: [u16; 4],
}

impl ButtonState {
    fn from_buffer(buffer: &[u8]) -> ButtonState {
        if buffer.len() != 10 {
            panic!("Button state constructor called with incorrectly sized buffer");
        }

        let mut pressed: [bool; 14] = [false; 14];
        let mut joysticks: [u16; 4] = [0; 4];

        // first 12 bits (in 2 bytes) are buttons
        for i in 0..14 {
            pressed[i] = (buffer[i / 8] >> (i % 8)) & 1 == 1;
        }

        // last 8 bytes are joystick positions
        for i in 0..4 {
            joysticks[i] = buffer[2 + i * 2] as u16 + ((buffer[2 + i * 2 + 1] as u16) << 8);
        }

        ButtonState { pressed, joysticks }
    }
}

#[derive(Debug)]
pub enum Error {
    SerialPortError(serialport::Error),
    IoError(io::Error),
}

impl From<serialport::Error> for Error {
    fn from(e: serialport::Error) -> Self {
        Error::SerialPortError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::SerialPortError(serial_port_error) => {
                write!(f, "SerialPortError: {}", serial_port_error)
            }
            Error::IoError(io_error) => write!(f, "IoError: {}", io_error),
        }
    }
}

impl error::Error for Error {}
