#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ROMSWMAP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SW0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW0ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW0ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW0ENR::NOTVIS => 0,
            SW0ENR::CORE => 1,
            SW0ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW0ENR {
        match value {
            0 => SW0ENR::NOTVIS,
            1 => SW0ENR::CORE,
            i => SW0ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW0ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW0ENR::CORE
    }
}
#[doc = "Possible values of the field `SW1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW1ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW1ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW1ENR::NOTVIS => 0,
            SW1ENR::CORE => 1,
            SW1ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW1ENR {
        match value {
            0 => SW1ENR::NOTVIS,
            1 => SW1ENR::CORE,
            i => SW1ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW1ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW1ENR::CORE
    }
}
#[doc = "Possible values of the field `SW2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW2ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW2ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW2ENR::NOTVIS => 0,
            SW2ENR::CORE => 1,
            SW2ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW2ENR {
        match value {
            0 => SW2ENR::NOTVIS,
            1 => SW2ENR::CORE,
            i => SW2ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW2ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW2ENR::CORE
    }
}
#[doc = "Possible values of the field `SW3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW3ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW3ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW3ENR::NOTVIS => 0,
            SW3ENR::CORE => 1,
            SW3ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW3ENR {
        match value {
            0 => SW3ENR::NOTVIS,
            1 => SW3ENR::CORE,
            i => SW3ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW3ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW3ENR::CORE
    }
}
#[doc = "Possible values of the field `SW4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW4ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW4ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW4ENR::NOTVIS => 0,
            SW4ENR::CORE => 1,
            SW4ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW4ENR {
        match value {
            0 => SW4ENR::NOTVIS,
            1 => SW4ENR::CORE,
            i => SW4ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW4ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW4ENR::CORE
    }
}
#[doc = "Possible values of the field `SW5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW5ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW5ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW5ENR::NOTVIS => 0,
            SW5ENR::CORE => 1,
            SW5ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW5ENR {
        match value {
            0 => SW5ENR::NOTVIS,
            1 => SW5ENR::CORE,
            i => SW5ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW5ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW5ENR::CORE
    }
}
#[doc = "Possible values of the field `SW6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW6ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW6ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW6ENR::NOTVIS => 0,
            SW6ENR::CORE => 1,
            SW6ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW6ENR {
        match value {
            0 => SW6ENR::NOTVIS,
            1 => SW6ENR::CORE,
            i => SW6ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW6ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW6ENR::CORE
    }
}
#[doc = "Possible values of the field `SW7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW7ENR {
    #[doc = "Software region not available to the core"]
    NOTVIS,
    #[doc = "Region available to core"]
    CORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SW7ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SW7ENR::NOTVIS => 0,
            SW7ENR::CORE => 1,
            SW7ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SW7ENR {
        match value {
            0 => SW7ENR::NOTVIS,
            1 => SW7ENR::CORE,
            i => SW7ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline]
    pub fn is_notvis(&self) -> bool {
        *self == SW7ENR::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == SW7ENR::CORE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline]
    pub fn sw0en(&self) -> SW0ENR {
        SW0ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline]
    pub fn sw1en(&self) -> SW1ENR {
        SW1ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline]
    pub fn sw2en(&self) -> SW2ENR {
        SW2ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline]
    pub fn sw3en(&self) -> SW3ENR {
        SW3ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline]
    pub fn sw4en(&self) -> SW4ENR {
        SW4ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline]
    pub fn sw5en(&self) -> SW5ENR {
        SW5ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline]
    pub fn sw6en(&self) -> SW6ENR {
        SW6ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline]
    pub fn sw7en(&self) -> SW7ENR {
        SW7ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
