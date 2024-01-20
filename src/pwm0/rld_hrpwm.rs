#[doc = "Register `rld_hrpwm` reader"]
pub type R = crate::R<RLD_HRPWM_SPEC>;
#[doc = "Register `rld_hrpwm` writer"]
pub type W = crate::W<RLD_HRPWM_SPEC>;
#[doc = "Field `RLD_HR` reader - pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
pub type RLD_HR_R = crate::FieldReader;
#[doc = "Field `RLD_HR` writer - pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
pub type RLD_HR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLD` reader - No description avaiable"]
pub type RLD_R = crate::FieldReader<u32>;
#[doc = "Field `RLD` writer - No description avaiable"]
pub type RLD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
    #[inline(always)]
    pub fn rld_hr(&self) -> RLD_HR_R {
        RLD_HR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rld_hr(&mut self) -> RLD_HR_W<RLD_HRPWM_SPEC> {
        RLD_HR_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn rld(&mut self) -> RLD_W<RLD_HRPWM_SPEC> {
        RLD_W::new(self, 8)
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
#[doc = "Counter reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld_hrpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld_hrpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLD_HRPWM_SPEC;
impl crate::RegisterSpec for RLD_HRPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rld_hrpwm::R`](R) reader structure"]
impl crate::Readable for RLD_HRPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rld_hrpwm::W`](W) writer structure"]
impl crate::Writable for RLD_HRPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rld_hrpwm to value 0"]
impl crate::Resettable for RLD_HRPWM_SPEC {
    const RESET_VALUE: u32 = 0;
}
