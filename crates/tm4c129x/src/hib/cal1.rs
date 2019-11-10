#[doc = "Reader of register CAL1"]
pub type R = crate::R<u32, super::CAL1>;
#[doc = "Reader of field `DOM`"]
pub type DOM_R = crate::R<u8, u8>;
#[doc = "Reader of field `MON`"]
pub type MON_R = crate::R<u8, u8>;
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u8, u8>;
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, u8>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
