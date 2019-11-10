#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `CMP0`"]
pub type CMP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `C0O`"]
pub type C0O_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1O`"]
pub type C1O_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn c0o(&self) -> C0O_R {
        C0O_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn c1o(&self) -> C1O_R {
        C1O_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
