#[doc = "Register `DCDC_DEBUG` reader"]
pub type R = crate::R<DCDC_DEBUG_SPEC>;
#[doc = "Register `DCDC_DEBUG` writer"]
pub type W = crate::W<DCDC_DEBUG_SPEC>;
#[doc = "Field `UPDATE_TIME` reader - DCDC voltage change time in 24M clock cycles, default value is 1mS"]
pub type UPDATE_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `UPDATE_TIME` writer - DCDC voltage change time in 24M clock cycles, default value is 1mS"]
pub type UPDATE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - DCDC voltage change time in 24M clock cycles, default value is 1mS"]
    #[inline(always)]
    pub fn update_time(&self) -> UPDATE_TIME_R {
        UPDATE_TIME_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - DCDC voltage change time in 24M clock cycles, default value is 1mS"]
    #[inline(always)]
    #[must_use]
    pub fn update_time(&mut self) -> UPDATE_TIME_W<DCDC_DEBUG_SPEC> {
        UPDATE_TIME_W::new(self, 0)
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
#[doc = "DCDC Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_DEBUG_SPEC;
impl crate::RegisterSpec for DCDC_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_debug::R`](R) reader structure"]
impl crate::Readable for DCDC_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_debug::W`](W) writer structure"]
impl crate::Writable for DCDC_DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_DEBUG to value 0x5dbf"]
impl crate::Resettable for DCDC_DEBUG_SPEC {
    const RESET_VALUE: u32 = 0x5dbf;
}
