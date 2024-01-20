#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TXEFS_SPEC>;
#[doc = "Register `TXEFS` writer"]
pub type W = crate::W<TXEFS_SPEC>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
pub type EFPI_R = crate::FieldReader;
#[doc = "Field `EFF` reader - Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
pub type EFF_R = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
pub type TEFL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "tx event fifo status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TXEFS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txefs::W`](W) writer structure"]
impl crate::Writable for TXEFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
