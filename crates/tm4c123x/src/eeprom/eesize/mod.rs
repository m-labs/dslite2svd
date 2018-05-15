#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EESIZE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct WORDCNTR {
    bits: u16,
}
impl WORDCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BLKCNTR {
    bits: u16,
}
impl BLKCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline]
    pub fn wordcnt(&self) -> WORDCNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WORDCNTR { bits }
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline]
    pub fn blkcnt(&self) -> BLKCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BLKCNTR { bits }
    }
}
