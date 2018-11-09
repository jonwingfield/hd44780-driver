use embedded_hal::timer::CountDown;

mod eightbit;
mod fourbit;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
use time::Us;

pub trait DataBus {
    fn write<C: CountDown<Time = T>, T: From<Us>>(&mut self, byte: u8, data: bool, timer: &mut C);

    // TODO
    // fn read(...)
}
