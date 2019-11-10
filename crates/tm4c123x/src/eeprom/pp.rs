#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - EEPROM Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x1f) as u8)
    }
}
