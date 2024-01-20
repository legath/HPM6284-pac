#[doc = "Register `DCDC_RESUME_TIME` reader"]
pub type R = crate::R<DCDC_RESUME_TIME_SPEC>;
#[doc = "Register `DCDC_RESUME_TIME` writer"]
pub type W = crate::W<DCDC_RESUME_TIME_SPEC>;
#[doc = "Field `RESUME_TIME` reader - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
pub type RESUME_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `RESUME_TIME` writer - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
pub type RESUME_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
    #[inline(always)]
    pub fn resume_time(&self) -> RESUME_TIME_R {
        RESUME_TIME_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
    #[inline(always)]
    #[must_use]
    pub fn resume_time(&mut self) -> RESUME_TIME_W<DCDC_RESUME_TIME_SPEC> {
        RESUME_TIME_W::new(self, 0)
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
#[doc = "DCDC resume time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_resume_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_resume_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_RESUME_TIME_SPEC;
impl crate::RegisterSpec for DCDC_RESUME_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_resume_time::R`](R) reader structure"]
impl crate::Readable for DCDC_RESUME_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_resume_time::W`](W) writer structure"]
impl crate::Writable for DCDC_RESUME_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_RESUME_TIME to value 0x8c9f"]
impl crate::Resettable for DCDC_RESUME_TIME_SPEC {
    const RESET_VALUE: u32 = 0x8c9f;
}
