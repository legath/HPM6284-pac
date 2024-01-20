#[doc = "Register `DMACFG[%s]` reader"]
pub type R = crate::R<DMACFG_SPEC>;
#[doc = "Register `DMACFG[%s]` writer"]
pub type W = crate::W<DMACFG_SPEC>;
#[doc = "Field `DMASRCSEL` reader - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_R = crate::FieldReader;
#[doc = "Field `DMASRCSEL` writer - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This field selects one of the DMA requests as the DMA request output."]
    #[inline(always)]
    pub fn dmasrcsel(&self) -> DMASRCSEL_R {
        DMASRCSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field selects one of the DMA requests as the DMA request output."]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcsel(&mut self) -> DMASRCSEL_W<DMACFG_SPEC> {
        DMASRCSEL_W::new(self, 0)
    }
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACFG[%s]
to value 0"]
impl crate::Resettable for DMACFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
