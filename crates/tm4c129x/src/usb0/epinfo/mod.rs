#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::EPINFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXEPR {
    bits: u8,
}
impl TXEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXEPR {
    bits: u8,
}
impl RXEPR {
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
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline]
    pub fn txep(&self) -> TXEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        TXEPR { bits }
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline]
    pub fn rxep(&self) -> RXEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RXEPR { bits }
    }
}
