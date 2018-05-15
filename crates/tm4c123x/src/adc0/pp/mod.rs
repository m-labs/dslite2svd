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
#[doc = "Possible values of the field `MSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSRR {
    #[doc = "125 ksps"]
    _125K,
    #[doc = "250 ksps"]
    _250K,
    #[doc = "500 ksps"]
    _500K,
    #[doc = "1 Msps"]
    _1M,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSRR::_125K => 1,
            MSRR::_250K => 3,
            MSRR::_500K => 5,
            MSRR::_1M => 7,
            MSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSRR {
        match value {
            1 => MSRR::_125K,
            3 => MSRR::_250K,
            5 => MSRR::_500K,
            7 => MSRR::_1M,
            i => MSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline]
    pub fn is_125k(&self) -> bool {
        *self == MSRR::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline]
    pub fn is_250k(&self) -> bool {
        *self == MSRR::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline]
    pub fn is_500k(&self) -> bool {
        *self == MSRR::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline]
    pub fn is_1m(&self) -> bool {
        *self == MSRR::_1M
    }
}
#[doc = r" Value of the field"]
pub struct CHR {
    bits: u8,
}
impl CHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCR {
    bits: u8,
}
impl DCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "SAR"]
    SAR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::SAR => 0,
            TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::SAR,
            i => TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAR`"]
    #[inline]
    pub fn is_sar(&self) -> bool {
        *self == TYPER::SAR
    }
}
#[doc = r" Value of the field"]
pub struct RSLR {
    bits: u8,
}
impl RSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSR {
    bits: bool,
}
impl TSR {
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
    #[doc = "Bits 0:3 - Maximum ADC Sample Rate"]
    #[inline]
    pub fn msr(&self) -> MSRR {
        MSRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline]
    pub fn ch(&self) -> CHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHR { bits }
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline]
    pub fn dc(&self) -> DCR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCR { bits }
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline]
    pub fn rsl(&self) -> RSLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSLR { bits }
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline]
    pub fn ts(&self) -> TSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSR { bits }
    }
}
