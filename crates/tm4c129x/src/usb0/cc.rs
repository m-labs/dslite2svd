#[doc = "Reader of register CC"]
pub type R = crate::R<u32, super::CC>;
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Reader of field `CSD`"]
pub type CSD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - PLL Clock Divisor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Clock Source/Direction"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
