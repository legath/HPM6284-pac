#[doc = "Register `sta_hrpwm` reader"]
pub type R = crate::R<STA_HRPWM_SPEC>;
#[doc = "Register `sta_hrpwm` writer"]
pub type W = crate::W<STA_HRPWM_SPEC>;
#[doc = "Field `STA` reader - No description avaiable"]
pub type STA_R = crate::FieldReader<u32>;
#[doc = "Field `STA` writer - No description avaiable"]
pub type STA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<STA_HRPWM_SPEC> {
        STA_W::new(self, 8)
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
#[doc = "Counter start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta_hrpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta_hrpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STA_HRPWM_SPEC;
impl crate::RegisterSpec for STA_HRPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta_hrpwm::R`](R) reader structure"]
impl crate::Readable for STA_HRPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sta_hrpwm::W`](W) writer structure"]
impl crate::Writable for STA_HRPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sta_hrpwm to value 0"]
impl crate::Resettable for STA_HRPWM_SPEC {
    const RESET_VALUE: u32 = 0;
}
