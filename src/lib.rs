#![feature(asm)] 
#![feature(min_const_fn)] 
#![no_std]
pub use self::io::*;

/// Functions for low level hardware control
pub mod io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}