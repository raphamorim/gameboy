//! Save states: a snapshot of the whole machine, minus the ROM.
//!
//! Each component writes its fields in a fixed order and reads them back in
//! the same order. The format is versioned; bump `VERSION` whenever a field
//! is added, removed or reordered.

pub type Result = std::result::Result<(), &'static str>;

pub const MAGIC: &[u8; 4] = b"GBST";
pub const VERSION: u16 = 1;

const TRUNCATED: &str = "The save state is incomplete or damaged.";
pub const WRONG_GAME: &str = "The save state is for a different game.";

#[derive(Default)]
pub struct Writer(pub Vec<u8>);

impl Writer {
    pub fn bytes(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
}

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    pub fn bytes(&mut self, len: usize) -> std::result::Result<&'a [u8], &'static str> {
        let end = self.pos.checked_add(len).ok_or(TRUNCATED)?;
        let bytes = self.data.get(self.pos..end).ok_or(TRUNCATED)?;
        self.pos = end;
        Ok(bytes)
    }

    pub fn is_empty(&self) -> bool {
        self.pos == self.data.len()
    }
}

pub trait State {
    fn save_state(&self, w: &mut Writer);
    fn load_state(&mut self, r: &mut Reader) -> Result;
}

macro_rules! int_state {
    ($($t:ty),*) => {$(
        impl State for $t {
            fn save_state(&self, w: &mut Writer) {
                w.bytes(&self.to_le_bytes());
            }
            fn load_state(&mut self, r: &mut Reader) -> Result {
                let bytes = r.bytes(std::mem::size_of::<$t>())?;
                *self = <$t>::from_le_bytes(bytes.try_into().unwrap());
                Ok(())
            }
        }
    )*};
}
int_state!(u8, u16, u32, u64, i32);

impl State for bool {
    fn save_state(&self, w: &mut Writer) {
        (*self as u8).save_state(w);
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        let mut v = 0u8;
        v.load_state(r)?;
        *self = v != 0;
        Ok(())
    }
}

impl State for usize {
    fn save_state(&self, w: &mut Writer) {
        (*self as u64).save_state(w);
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        let mut v = 0u64;
        v.load_state(r)?;
        *self = usize::try_from(v).map_err(|_| TRUNCATED)?;
        Ok(())
    }
}

impl<T: State, const N: usize> State for [T; N] {
    fn save_state(&self, w: &mut Writer) {
        self.iter().for_each(|v| v.save_state(w));
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        self.iter_mut().try_for_each(|v| v.load_state(r))
    }
}

impl<T: State> State for Box<T> {
    fn save_state(&self, w: &mut Writer) {
        (**self).save_state(w);
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        (**self).load_state(r)
    }
}

/// Cartridge RAM. Its size comes from the cartridge header, so a state with a
/// different size belongs to another game.
impl State for Vec<u8> {
    fn save_state(&self, w: &mut Writer) {
        (self.len() as u32).save_state(w);
        w.bytes(self);
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        let mut len = 0u32;
        len.load_state(r)?;
        if len as usize != self.len() {
            return Err(WRONG_GAME);
        }
        let len = self.len();
        self.copy_from_slice(r.bytes(len)?);
        Ok(())
    }
}

impl State for Option<u64> {
    fn save_state(&self, w: &mut Writer) {
        self.is_some().save_state(w);
        self.unwrap_or(0).save_state(w);
    }
    fn load_state(&mut self, r: &mut Reader) -> Result {
        let (mut some, mut v) = (false, 0u64);
        some.load_state(r)?;
        v.load_state(r)?;
        *self = some.then_some(v);
        Ok(())
    }
}

/// Implements `State` for a struct by listing the fields to keep, in order.
macro_rules! impl_state {
    ($ty:ty { $($field:ident),* $(,)? }) => {
        impl crate::state::State for $ty {
            fn save_state(&self, w: &mut crate::state::Writer) {
                $(crate::state::State::save_state(&self.$field, w);)*
            }
            fn load_state(&mut self, r: &mut crate::state::Reader) -> crate::state::Result {
                $(crate::state::State::load_state(&mut self.$field, r)?;)*
                Ok(())
            }
        }
    };
}
pub(crate) use impl_state;

/// Implements `State` for a fieldless enum, stored as its index.
macro_rules! enum_state {
    ($ty:ty { $($variant:path),* $(,)? }) => {
        impl crate::state::State for $ty {
            fn save_state(&self, w: &mut crate::state::Writer) {
                let index = [$($variant),*].iter().position(|v| v == self).unwrap() as u8;
                crate::state::State::save_state(&index, w);
            }
            fn load_state(&mut self, r: &mut crate::state::Reader) -> crate::state::Result {
                let mut index = 0u8;
                crate::state::State::load_state(&mut index, r)?;
                *self = *[$($variant),*]
                    .get(index as usize)
                    .ok_or("The save state is incomplete or damaged.")?;
                Ok(())
            }
        }
    };
}
pub(crate) use enum_state;
