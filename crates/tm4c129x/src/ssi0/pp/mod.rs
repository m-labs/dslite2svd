#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct HSCLKR {
    bits: bool,
}
impl HSCLKR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Legacy SSI mode"]
    LEGACY,
    #[doc = "Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    ADVBI,
    #[doc = "Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    ADVBIQUAD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::LEGACY => 0,
            MODER::ADVBI => 1,
            MODER::ADVBIQUAD => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::LEGACY,
            1 => MODER::ADVBI,
            2 => MODER::ADVBIQUAD,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline]
    pub fn is_legacy(&self) -> bool {
        *self == MODER::LEGACY
    }
    #[doc = "Checks if the value of the field is `ADVBI`"]
    #[inline]
    pub fn is_advbi(&self) -> bool {
        *self == MODER::ADVBI
    }
    #[doc = "Checks if the value of the field is `ADVBIQUAD`"]
    #[inline]
    pub fn is_advbiquad(&self) -> bool {
        *self == MODER::ADVBIQUAD
    }
}
#[doc = r" Value of the field"]
pub struct FSSHLDFRMR {
    bits: bool,
}
impl FSSHLDFRMR {
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
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline]
    pub fn hsclk(&self) -> HSCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSCLKR { bits }
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline]
    pub fn fsshldfrm(&self) -> FSSHLDFRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSSHLDFRMR { bits }
    }
}
