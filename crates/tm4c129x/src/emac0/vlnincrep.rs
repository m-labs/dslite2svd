#[doc = "Reader of register VLNINCREP"]
pub type R = crate::R<u32, super::VLNINCREP>;
#[doc = "Writer for register VLNINCREP"]
pub type W = crate::W<u32, super::VLNINCREP>;
#[doc = "Register VLNINCREP `reset()`'s with value 0"]
impl crate::ResetValue for super::VLNINCREP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLT`"]
pub type VLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLT`"]
pub struct VLT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "VLAN Tag Control in Transmit Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLC_A {
    #[doc = "0: No VLAN tag deletion, insertion, or replacement"]
    NONE,
    #[doc = "1: VLAN tag deletion"]
    TAGDEL,
    #[doc = "2: VLAN tag insertion"]
    TAGINS,
    #[doc = "3: VLAN tag replacement"]
    TAGREP,
}
impl From<VLC_A> for u8 {
    #[inline(always)]
    fn from(variant: VLC_A) -> Self {
        match variant {
            VLC_A::NONE => 0,
            VLC_A::TAGDEL => 1,
            VLC_A::TAGINS => 2,
            VLC_A::TAGREP => 3,
        }
    }
}
#[doc = "Reader of field `VLC`"]
pub type VLC_R = crate::R<u8, VLC_A>;
impl VLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLC_A {
        match self.bits {
            0 => VLC_A::NONE,
            1 => VLC_A::TAGDEL,
            2 => VLC_A::TAGINS,
            3 => VLC_A::TAGREP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VLC_A::NONE
    }
    #[doc = "Checks if the value of the field is `TAGDEL`"]
    #[inline(always)]
    pub fn is_tagdel(&self) -> bool {
        *self == VLC_A::TAGDEL
    }
    #[doc = "Checks if the value of the field is `TAGINS`"]
    #[inline(always)]
    pub fn is_tagins(&self) -> bool {
        *self == VLC_A::TAGINS
    }
    #[doc = "Checks if the value of the field is `TAGREP`"]
    #[inline(always)]
    pub fn is_tagrep(&self) -> bool {
        *self == VLC_A::TAGREP
    }
}
#[doc = "Write proxy for field `VLC`"]
pub struct VLC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VLC_A::NONE)
    }
    #[doc = "VLAN tag deletion"]
    #[inline(always)]
    pub fn tagdel(self) -> &'a mut W {
        self.variant(VLC_A::TAGDEL)
    }
    #[doc = "VLAN tag insertion"]
    #[inline(always)]
    pub fn tagins(self) -> &'a mut W {
        self.variant(VLC_A::TAGINS)
    }
    #[doc = "VLAN tag replacement"]
    #[inline(always)]
    pub fn tagrep(self) -> &'a mut W {
        self.variant(VLC_A::TAGREP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `VLP`"]
pub type VLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLP`"]
pub struct VLP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CSVL`"]
pub type CSVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSVL`"]
pub struct CSVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSVL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W {
        VLT_W { w: self }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W {
        VLC_W { w: self }
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W {
        VLP_W { w: self }
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W {
        CSVL_W { w: self }
    }
}
