#[doc = "Register `SS_STEP` reader"]
pub type R = crate::R<SS_STEP_SPEC>;
#[doc = "Register `SS_STEP` writer"]
pub type W = crate::W<SS_STEP_SPEC>;
#[doc = "Field `STEP` reader - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
pub type STEP_R = crate::FieldReader<u32>;
#[doc = "Field `STEP` writer - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<SS_STEP_SPEC> {
        STEP_W::new(self, 0)
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
#[doc = "PLL0 spread spectrum step register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_step::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_step::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_STEP_SPEC;
impl crate::RegisterSpec for SS_STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_step::R`](R) reader structure"]
impl crate::Readable for SS_STEP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_step::W`](W) writer structure"]
impl crate::Writable for SS_STEP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_STEP to value 0"]
impl crate::Resettable for SS_STEP_SPEC {
    const RESET_VALUE: u32 = 0;
}
