#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::LPMCNTRL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXLPMR {
    bits: bool,
}
impl TXLPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RESR {
    bits: bool,
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    NONE,
    #[doc = "LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    EXT,
    #[doc = "The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    LPMEXT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENR::NONE => 0,
            ENR::EXT => 1,
            ENR::LPMEXT => 3,
            ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENR {
        match value {
            0 => ENR::NONE,
            1 => ENR::EXT,
            3 => ENR::LPMEXT,
            i => ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ENR::NONE
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline]
    pub fn is_ext(&self) -> bool {
        *self == ENR::EXT
    }
    #[doc = "Checks if the value of the field is `LPMEXT`"]
    #[inline]
    pub fn is_lpmext(&self) -> bool {
        *self == ENR::LPMEXT
    }
}
#[doc = r" Value of the field"]
pub struct NAKR {
    bits: bool,
}
impl NAKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline]
    pub fn txlpm(&self) -> TXLPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        TXLPMR { bits }
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline]
    pub fn res(&self) -> RESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RESR { bits }
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline]
    pub fn nak(&self) -> NAKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        NAKR { bits }
    }
}
