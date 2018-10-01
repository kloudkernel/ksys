use core::marker::PhantomData;
use super::io::Io;

#[derive(Copy, Clone)]
pub struct PortIO<T> {
    port: u16, //IO Port number
    value: PhantomData<T>, // Do not hold this value yourself, only hint for the compiler
}


impl<T> PortIO<T> {
    pub const fn new(port: u16) -> Self {
        PortIO::<T> {
            port: port,
            value: PhantomData,
        }
    }
}

impl Io for PortIO<u8> {
    type Value = u8;

    // reads a single byte from IO port
    #[inline(always)]
    fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }


    // write a single byte to IO port
    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

