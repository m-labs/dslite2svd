#[doc = "Reader of register SSFSTAT2"]
pub type R = crate::R<u32, super::SSFSTAT2>;
#[doc = "Reader of field `TPTR`"]
pub type TPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPTR`"]
pub type HPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn tptr(&self) -> TPTR_R {
        TPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn hptr(&self) -> HPTR_R {
        HPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
