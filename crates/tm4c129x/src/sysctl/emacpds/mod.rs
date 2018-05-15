#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EMACPDS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PWRSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSTATR {
    #[doc = "OFF"]
    OFF,
    #[doc = "ON"]
    ON,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRSTATR::OFF => 0,
            PWRSTATR::ON => 3,
            PWRSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRSTATR {
        match value {
            0 => PWRSTATR::OFF,
            3 => PWRSTATR::ON,
            i => PWRSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PWRSTATR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == PWRSTATR::ON
    }
}
#[doc = "Possible values of the field `MEMSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSTATR {
    #[doc = "Array OFF"]
    OFF,
    #[doc = "Array On"]
    ON,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MEMSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MEMSTATR::OFF => 0,
            MEMSTATR::ON => 3,
            MEMSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MEMSTATR {
        match value {
            0 => MEMSTATR::OFF,
            3 => MEMSTATR::ON,
            i => MEMSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == MEMSTATR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == MEMSTATR::ON
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline]
    pub fn pwrstat(&self) -> PWRSTATR {
        PWRSTATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline]
    pub fn memstat(&self) -> MEMSTATR {
        MEMSTATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
