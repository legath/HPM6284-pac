#[doc = "Register `CMP_HRPWM[%s]` reader"]
pub type R = crate::R<CMP_HRPWM_SPEC>;
#[doc = "Register `CMP_HRPWM[%s]` writer"]
pub type W = crate::W<CMP_HRPWM_SPEC>;
#[doc = "Field `CMP_HR` reader - high resolution compare value"]
pub type CMP_HR_R = crate::FieldReader;
#[doc = "Field `CMP_HR` writer - high resolution compare value"]
pub type CMP_HR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP` reader - No description avaiable"]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - No description avaiable"]
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - high resolution compare value"]
    #[inline(always)]
    pub fn cmp_hr(&self) -> CMP_HR_R {
        CMP_HR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - high resolution compare value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_hr(&mut self) -> CMP_HR_W<CMP_HRPWM_SPEC> {
        CMP_HR_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<CMP_HRPWM_SPEC> {
        CMP_W::new(self, 8)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_hrpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_hrpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_HRPWM_SPEC;
impl crate::RegisterSpec for CMP_HRPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_hrpwm::R`](R) reader structure"]
impl crate::Readable for CMP_HRPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp_hrpwm::W`](W) writer structure"]
impl crate::Writable for CMP_HRPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_HRPWM[%s]
to value 0"]
impl crate::Resettable for CMP_HRPWM_SPEC {
    const RESET_VALUE: u32 = 0;
}
