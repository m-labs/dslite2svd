#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `INTID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTIDR {
    #[doc = "No interrupt pending"]
    NONE,
    #[doc = "Status Interrupt"]
    STATUS,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl INTIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            INTIDR::NONE => 0,
            INTIDR::STATUS => 32768,
            INTIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> INTIDR {
        match value {
            0 => INTIDR::NONE,
            32768 => INTIDR::STATUS,
            i => INTIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == INTIDR::NONE
    }
    #[doc = "Checks if the value of the field is `STATUS`"]
    #[inline]
    pub fn is_status(&self) -> bool {
        *self == INTIDR::STATUS
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline]
    pub fn intid(&self) -> INTIDR {
        INTIDR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
