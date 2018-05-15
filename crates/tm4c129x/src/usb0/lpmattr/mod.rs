#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::LPMATTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSR {
    #[doc = "Sleep State (L1)"]
    L1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LSR::L1 => 1,
            LSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LSR {
        match value {
            1 => LSR::L1,
            i => LSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline]
    pub fn is_l1(&self) -> bool {
        *self == LSR::L1
    }
}
#[doc = r" Value of the field"]
pub struct HIRDR {
    bits: u8,
}
impl HIRDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RMTWAKR {
    bits: bool,
}
impl RMTWAKR {
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
pub struct ENDPTR {
    bits: u8,
}
impl ENDPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Link State"]
    #[inline]
    pub fn ls(&self) -> LSR {
        LSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline]
    pub fn hird(&self) -> HIRDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        HIRDR { bits }
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline]
    pub fn rmtwak(&self) -> RMTWAKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RMTWAKR { bits }
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline]
    pub fn endpt(&self) -> ENDPTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        ENDPTR { bits }
    }
}
