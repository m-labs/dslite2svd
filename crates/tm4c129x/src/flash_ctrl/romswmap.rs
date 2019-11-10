#[doc = "Reader of register ROMSWMAP"]
pub type R = crate::R<u32, super::ROMSWMAP>;
#[doc = "ROM SW Region 0 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW0EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW0EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW0EN_A) -> Self {
        match variant {
            SW0EN_A::NOTVIS => 0,
            SW0EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW0EN`"]
pub type SW0EN_R = crate::R<u8, SW0EN_A>;
impl SW0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW0EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW0EN_A::NOTVIS),
            1 => Val(SW0EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW0EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW0EN_A::CORE
    }
}
#[doc = "ROM SW Region 1 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW1EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW1EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW1EN_A) -> Self {
        match variant {
            SW1EN_A::NOTVIS => 0,
            SW1EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW1EN`"]
pub type SW1EN_R = crate::R<u8, SW1EN_A>;
impl SW1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW1EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW1EN_A::NOTVIS),
            1 => Val(SW1EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW1EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW1EN_A::CORE
    }
}
#[doc = "ROM SW Region 2 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW2EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW2EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW2EN_A) -> Self {
        match variant {
            SW2EN_A::NOTVIS => 0,
            SW2EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW2EN`"]
pub type SW2EN_R = crate::R<u8, SW2EN_A>;
impl SW2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW2EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW2EN_A::NOTVIS),
            1 => Val(SW2EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW2EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW2EN_A::CORE
    }
}
#[doc = "ROM SW Region 3 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW3EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW3EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW3EN_A) -> Self {
        match variant {
            SW3EN_A::NOTVIS => 0,
            SW3EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW3EN`"]
pub type SW3EN_R = crate::R<u8, SW3EN_A>;
impl SW3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW3EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW3EN_A::NOTVIS),
            1 => Val(SW3EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW3EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW3EN_A::CORE
    }
}
#[doc = "ROM SW Region 4 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW4EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW4EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW4EN_A) -> Self {
        match variant {
            SW4EN_A::NOTVIS => 0,
            SW4EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW4EN`"]
pub type SW4EN_R = crate::R<u8, SW4EN_A>;
impl SW4EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW4EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW4EN_A::NOTVIS),
            1 => Val(SW4EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW4EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW4EN_A::CORE
    }
}
#[doc = "ROM SW Region 5 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW5EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW5EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW5EN_A) -> Self {
        match variant {
            SW5EN_A::NOTVIS => 0,
            SW5EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW5EN`"]
pub type SW5EN_R = crate::R<u8, SW5EN_A>;
impl SW5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW5EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW5EN_A::NOTVIS),
            1 => Val(SW5EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW5EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW5EN_A::CORE
    }
}
#[doc = "ROM SW Region 6 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW6EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW6EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW6EN_A) -> Self {
        match variant {
            SW6EN_A::NOTVIS => 0,
            SW6EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW6EN`"]
pub type SW6EN_R = crate::R<u8, SW6EN_A>;
impl SW6EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW6EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW6EN_A::NOTVIS),
            1 => Val(SW6EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW6EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW6EN_A::CORE
    }
}
#[doc = "ROM SW Region 7 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW7EN_A {
    #[doc = "0: Software region not available to the core"]
    NOTVIS,
    #[doc = "1: Region available to core"]
    CORE,
}
impl From<SW7EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SW7EN_A) -> Self {
        match variant {
            SW7EN_A::NOTVIS => 0,
            SW7EN_A::CORE => 1,
        }
    }
}
#[doc = "Reader of field `SW7EN`"]
pub type SW7EN_R = crate::R<u8, SW7EN_A>;
impl SW7EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW7EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW7EN_A::NOTVIS),
            1 => Val(SW7EN_A::CORE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTVIS`"]
    #[inline(always)]
    pub fn is_notvis(&self) -> bool {
        *self == SW7EN_A::NOTVIS
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == SW7EN_A::CORE
    }
}
impl R {
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline(always)]
    pub fn sw0en(&self) -> SW0EN_R {
        SW0EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline(always)]
    pub fn sw1en(&self) -> SW1EN_R {
        SW1EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline(always)]
    pub fn sw2en(&self) -> SW2EN_R {
        SW2EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline(always)]
    pub fn sw3en(&self) -> SW3EN_R {
        SW3EN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline(always)]
    pub fn sw4en(&self) -> SW4EN_R {
        SW4EN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline(always)]
    pub fn sw5en(&self) -> SW5EN_R {
        SW5EN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline(always)]
    pub fn sw6en(&self) -> SW6EN_R {
        SW6EN_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline(always)]
    pub fn sw7en(&self) -> SW7EN_R {
        SW7EN_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
