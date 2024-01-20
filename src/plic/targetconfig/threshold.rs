#[doc = "Register `THRESHOLD` reader"]
pub type R = crate::R<THRESHOLD_SPEC>;
#[doc = "Register `THRESHOLD` writer"]
pub type W = crate::W<THRESHOLD_SPEC>;
#[doc = "Field `THRESHOLD` reader - Interrupt priority threshold."]
pub type THRESHOLD_R = crate::FieldReader<u32>;
#[doc = "Field `THRESHOLD` writer - Interrupt priority threshold."]
pub type THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt priority threshold."]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt priority threshold."]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<THRESHOLD_SPEC> {
        THRESHOLD_W::new(self, 0)
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
#[doc = "Target0 priority threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRESHOLD_SPEC;
impl crate::RegisterSpec for THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold::R`](R) reader structure"]
impl crate::Readable for THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`threshold::W`](W) writer structure"]
impl crate::Writable for THRESHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRESHOLD to value 0"]
impl crate::Resettable for THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
