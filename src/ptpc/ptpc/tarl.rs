#[doc = "Register `tarl` reader"]
pub type R = crate::R<TARL_SPEC>;
#[doc = "Register `tarl` writer"]
pub type W = crate::W<TARL_SPEC>;
#[doc = "Field `TARGET_TIME_LOW` reader - No description avaiable"]
pub type TARGET_TIME_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET_TIME_LOW` writer - No description avaiable"]
pub type TARGET_TIME_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn target_time_low(&self) -> TARGET_TIME_LOW_R {
        TARGET_TIME_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn target_time_low(&mut self) -> TARGET_TIME_LOW_W<TARL_SPEC> {
        TARGET_TIME_LOW_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tarl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tarl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARL_SPEC;
impl crate::RegisterSpec for TARL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tarl::R`](R) reader structure"]
impl crate::Readable for TARL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tarl::W`](W) writer structure"]
impl crate::Writable for TARL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tarl to value 0"]
impl crate::Resettable for TARL_SPEC {
    const RESET_VALUE: u32 = 0;
}
