#[doc = "Register `IDMisc` reader"]
pub type R = crate::R<IDMISC_SPEC>;
#[doc = "Register `IDMisc` writer"]
pub type W = crate::W<IDMISC_SPEC>;
#[doc = "Field `IDLE_FLAG` reader - DMA Idle Flag 0 - DMA is busy 1 - DMA is dile"]
pub type IDLE_FLAG_R = crate::BitReader;
impl R {
    #[doc = "Bit 15 - DMA Idle Flag 0 - DMA is busy 1 - DMA is dile"]
    #[inline(always)]
    pub fn idle_flag(&self) -> IDLE_FLAG_R {
        IDLE_FLAG_R::new(((self.bits >> 15) & 1) != 0)
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
#[doc = "ID Misc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMISC_SPEC;
impl crate::RegisterSpec for IDMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmisc::R`](R) reader structure"]
impl crate::Readable for IDMISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idmisc::W`](W) writer structure"]
impl crate::Writable for IDMISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMisc to value 0"]
impl crate::Resettable for IDMISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
