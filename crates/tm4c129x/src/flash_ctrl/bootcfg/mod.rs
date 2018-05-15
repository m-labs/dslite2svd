#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BOOTCFG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DBG0R {
    bits: bool,
}
impl DBG0R {
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
pub struct DBG1R {
    bits: bool,
}
impl DBG1R {
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
pub struct KEYR {
    bits: bool,
}
impl KEYR {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct POLR {
    bits: bool,
}
impl POLR {
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
#[doc = "Possible values of the field `PIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINR {
    #[doc = "Pin 0"]
    _0,
    #[doc = "Pin 1"]
    _1,
    #[doc = "Pin 2"]
    _2,
    #[doc = "Pin 3"]
    _3,
    #[doc = "Pin 4"]
    _4,
    #[doc = "Pin 5"]
    _5,
    #[doc = "Pin 6"]
    _6,
    #[doc = "Pin 7"]
    _7,
}
impl PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINR::_0 => 0,
            PINR::_1 => 1,
            PINR::_2 => 2,
            PINR::_3 => 3,
            PINR::_4 => 4,
            PINR::_5 => 5,
            PINR::_6 => 6,
            PINR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINR {
        match value {
            0 => PINR::_0,
            1 => PINR::_1,
            2 => PINR::_2,
            3 => PINR::_3,
            4 => PINR::_4,
            5 => PINR::_5,
            6 => PINR::_6,
            7 => PINR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PINR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PINR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PINR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == PINR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == PINR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == PINR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == PINR::_7
    }
}
#[doc = "Possible values of the field `PORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTR {
    #[doc = "Port A"]
    A,
    #[doc = "Port B"]
    B,
    #[doc = "Port C"]
    C,
    #[doc = "Port D"]
    D,
    #[doc = "Port E"]
    E,
    #[doc = "Port F"]
    F,
    #[doc = "Port G"]
    G,
    #[doc = "Port H"]
    H,
}
impl PORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTR::A => 0,
            PORTR::B => 1,
            PORTR::C => 2,
            PORTR::D => 3,
            PORTR::E => 4,
            PORTR::F => 5,
            PORTR::G => 6,
            PORTR::H => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTR {
        match value {
            0 => PORTR::A,
            1 => PORTR::B,
            2 => PORTR::C,
            3 => PORTR::D,
            4 => PORTR::E,
            5 => PORTR::F,
            6 => PORTR::G,
            7 => PORTR::H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == PORTR::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == PORTR::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline]
    pub fn is_c(&self) -> bool {
        *self == PORTR::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline]
    pub fn is_d(&self) -> bool {
        *self == PORTR::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline]
    pub fn is_e(&self) -> bool {
        *self == PORTR::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline]
    pub fn is_f(&self) -> bool {
        *self == PORTR::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline]
    pub fn is_g(&self) -> bool {
        *self == PORTR::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline]
    pub fn is_h(&self) -> bool {
        *self == PORTR::H
    }
}
#[doc = r" Value of the field"]
pub struct NWR {
    bits: bool,
}
impl NWR {
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
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline]
    pub fn dbg0(&self) -> DBG0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG0R { bits }
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline]
    pub fn dbg1(&self) -> DBG1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG1R { bits }
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline]
    pub fn key(&self) -> KEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEYR { bits }
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline]
    pub fn pol(&self) -> POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLR { bits }
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline]
    pub fn pin(&self) -> PINR {
        PINR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline]
    pub fn port(&self) -> PORTR {
        PORTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline]
    pub fn nw(&self) -> NWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NWR { bits }
    }
}
