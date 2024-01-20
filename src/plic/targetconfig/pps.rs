#[doc = "Register `PPS` reader"]
pub type R = crate::R<PPS_SPEC>;
#[doc = "Register `PPS` writer"]
pub type W = crate::W<PPS_SPEC>;
#[doc = "Field `PRIORITY_PREEMPTED` reader - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
pub type PRIORITY_PREEMPTED_R = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY_PREEMPTED` writer - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
pub type PRIORITY_PREEMPTED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
    #[inline(always)]
    pub fn priority_preempted(&self) -> PRIORITY_PREEMPTED_R {
        PRIORITY_PREEMPTED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn priority_preempted(&mut self) -> PRIORITY_PREEMPTED_W<PPS_SPEC> {
        PRIORITY_PREEMPTED_W::new(self, 0)
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
#[doc = "Preempted priority stack\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPS_SPEC;
impl crate::RegisterSpec for PPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps::R`](R) reader structure"]
impl crate::Readable for PPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pps::W`](W) writer structure"]
impl crate::Writable for PPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPS to value 0"]
impl crate::Resettable for PPS_SPEC {
    const RESET_VALUE: u32 = 0;
}
