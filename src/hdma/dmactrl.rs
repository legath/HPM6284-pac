#[doc = "Register `DMACtrl` reader"]
pub type R = crate::R<DMACTRL_SPEC>;
#[doc = "Register `DMACtrl` writer"]
pub type W = crate::W<DMACTRL_SPEC>;
#[doc = "Field `RESET` writer - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<DMACTRL_SPEC> {
        RESET_W::new(self, 0)
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
#[doc = "DMAC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTRL_SPEC;
impl crate::RegisterSpec for DMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DMACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DMACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACtrl to value 0"]
impl crate::Resettable for DMACTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
