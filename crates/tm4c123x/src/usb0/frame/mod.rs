#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::FRAME {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FRAMER {
    bits: u16,
}
impl FRAMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline]
    pub fn frame(&self) -> FRAMER {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        FRAMER { bits }
    }
}
