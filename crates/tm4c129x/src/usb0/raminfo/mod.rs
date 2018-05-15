#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::RAMINFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RAMBITSR {
    bits: u8,
}
impl RAMBITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMACHANR {
    bits: u8,
}
impl DMACHANR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline]
    pub fn rambits(&self) -> RAMBITSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RAMBITSR { bits }
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline]
    pub fn dmachan(&self) -> DMACHANR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        DMACHANR { bits }
    }
}
