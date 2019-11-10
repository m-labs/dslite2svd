#[doc = "Reader of register BOOTCFG"]
pub type R = crate::R<u32, super::BOOTCFG>;
#[doc = "Reader of field `DBG0`"]
pub type DBG0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBG1`"]
pub type DBG1_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, bool>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `POL`"]
pub type POL_R = crate::R<bool, bool>;
#[doc = "Boot GPIO Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Pin 0"]
    _0,
    #[doc = "1: Pin 1"]
    _1,
    #[doc = "2: Pin 2"]
    _2,
    #[doc = "3: Pin 3"]
    _3,
    #[doc = "4: Pin 4"]
    _4,
    #[doc = "5: Pin 5"]
    _5,
    #[doc = "6: Pin 6"]
    _6,
    #[doc = "7: Pin 7"]
    _7,
}
impl From<PIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        match variant {
            PIN_A::_0 => 0,
            PIN_A::_1 => 1,
            PIN_A::_2 => 2,
            PIN_A::_3 => 3,
            PIN_A::_4 => 4,
            PIN_A::_5 => 5,
            PIN_A::_6 => 6,
            PIN_A::_7 => 7,
        }
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<u8, PIN_A>;
impl PIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            0 => PIN_A::_0,
            1 => PIN_A::_1,
            2 => PIN_A::_2,
            3 => PIN_A::_3,
            4 => PIN_A::_4,
            5 => PIN_A::_5,
            6 => PIN_A::_6,
            7 => PIN_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PIN_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PIN_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PIN_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == PIN_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PIN_A::_7
    }
}
#[doc = "Boot GPIO Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_A {
    #[doc = "0: Port A"]
    A,
    #[doc = "1: Port B"]
    B,
    #[doc = "2: Port C"]
    C,
    #[doc = "3: Port D"]
    D,
    #[doc = "4: Port E"]
    E,
    #[doc = "5: Port F"]
    F,
    #[doc = "6: Port G"]
    G,
    #[doc = "7: Port H"]
    H,
}
impl From<PORT_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_A) -> Self {
        match variant {
            PORT_A::A => 0,
            PORT_A::B => 1,
            PORT_A::C => 2,
            PORT_A::D => 3,
            PORT_A::E => 4,
            PORT_A::F => 5,
            PORT_A::G => 6,
            PORT_A::H => 7,
        }
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<u8, PORT_A>;
impl PORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_A {
        match self.bits {
            0 => PORT_A::A,
            1 => PORT_A::B,
            2 => PORT_A::C,
            3 => PORT_A::D,
            4 => PORT_A::E,
            5 => PORT_A::F,
            6 => PORT_A::G,
            7 => PORT_A::H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PORT_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PORT_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PORT_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PORT_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PORT_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PORT_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PORT_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PORT_A::H
    }
}
#[doc = "Reader of field `NW`"]
pub type NW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn dbg0(&self) -> DBG0_R {
        DBG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn dbg1(&self) -> DBG1_R {
        DBG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn nw(&self) -> NW_R {
        NW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
