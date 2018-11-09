pub struct Ms(pub u16);
pub struct Us(pub u16);

pub trait U16Ext {
    fn ms(self) -> Ms;
    fn us(self) -> Us;
}

impl U16Ext for u16 {
    fn ms(self) -> Ms {
        Ms(self)
    }

    fn us(self) -> Us {
        Us(self)
    }
}

impl Into<Us> for Ms {
    fn into(self) -> Us {
        Us(self.0 * 1_000)
    }
}
