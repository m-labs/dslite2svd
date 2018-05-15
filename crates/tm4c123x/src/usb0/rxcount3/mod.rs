#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::RXCOUNT3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct COUNTR {
    bits: u16,
}
impl COUNTR {
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
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        COUNTR { bits }
    }
}
