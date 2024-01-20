#[doc = "Register `DCDC_START_TIME` reader"]
pub type R = crate::R<DCDC_START_TIME_SPEC>;
#[doc = "Register `DCDC_START_TIME` writer"]
pub type W = crate::W<DCDC_START_TIME_SPEC>;
#[doc = "Field `START_TIME` reader - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
pub type START_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `START_TIME` writer - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
pub type START_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
    #[inline(always)]
    pub fn start_time(&self) -> START_TIME_R {
        START_TIME_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
    #[inline(always)]
    #[must_use]
    pub fn start_time(&mut self) -> START_TIME_W<DCDC_START_TIME_SPEC> {
        START_TIME_W::new(self, 0)
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
#[doc = "DCDC ramp time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_start_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_start_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_START_TIME_SPEC;
impl crate::RegisterSpec for DCDC_START_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_start_time::R`](R) reader structure"]
impl crate::Readable for DCDC_START_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_start_time::W`](W) writer structure"]
impl crate::Writable for DCDC_START_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_START_TIME to value 0x0001_193f"]
impl crate::Resettable for DCDC_START_TIME_SPEC {
    const RESET_VALUE: u32 = 0x0001_193f;
}
