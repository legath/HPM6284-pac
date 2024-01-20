#[doc = "Register `ANASTS[%s]` reader"]
pub type R = crate::R<ANASTS_SPEC>;
#[doc = "Register `ANASTS[%s]` writer"]
pub type W = crate::W<ANASTS_SPEC>;
#[doc = "Field `CALON` reader - calibration status. will be set by hardware after setting cal_start. cleared after calibration finished"]
pub type CALON_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - calibration status. will be set by hardware after setting cal_start. cleared after calibration finished"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anasts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anasts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANASTS_SPEC;
impl crate::RegisterSpec for ANASTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anasts::R`](R) reader structure"]
impl crate::Readable for ANASTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`anasts::W`](W) writer structure"]
impl crate::Writable for ANASTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANASTS[%s]
to value 0"]
impl crate::Resettable for ANASTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
