pub type SerialCallback<'a> = Box<dyn FnMut(u8) -> Option<u8> + Send + 'a>;

fn noop(_: u8) -> Option<u8> { None }

pub struct Serial<'a> {
    data: u8,
    control: u8,
    callback: SerialCallback<'a>,
    pub interrupt: u8,
}

impl<'a> Serial<'a>
{
    pub fn new_with_callback(cb: SerialCallback<'a>) -> Serial<'a>
    {
        Serial { data: 0, control: 0, callback: cb, interrupt: 0 }
    }

    pub fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF01 => self.data = v,
            0xFF02 => {
                self.control = v;
                if v & 0x81 == 0x81 {
                    match (self.callback)(self.data) {
                        Some(v) => {
                            self.data = v;
                            self.interrupt = 0x8
                        },
                        None => {},
                    }
                }
            },
            _ => panic!("Serial does not handle address {:4X} (write)", a),
        };
    }

    pub fn rb(&self, a: u16) -> u8 {
        match a {
            0xFF01 => self.data,
            0xFF02 => self.control | 0b01111110,
            _ => panic!("Serial does not handle address {:4X} (read)", a),
        }
    }

    pub fn set_callback(&mut self, cb: SerialCallback<'static>) {
        self.callback = cb;
    }

    pub fn unset_callback(&mut self) {
        self.callback = Box::new(noop);
    }
}

impl Serial<'static> {
    pub fn new() -> Serial<'static> {
        Serial { data: 0, control: 0, callback: Box::new(noop), interrupt: 0 }
    }
}
