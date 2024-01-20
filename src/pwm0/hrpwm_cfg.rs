#[doc = "Register `hrpwm_cfg` reader"]
pub type R = crate::R<HRPWM_CFG_SPEC>;
#[doc = "Register `hrpwm_cfg` writer"]
pub type W = crate::W<HRPWM_CFG_SPEC>;
#[doc = "Field `CAL_START` writer - calibration start. software setting this bit to start calibration process. each bit for one channel."]
pub type CAL_START_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - calibration start. software setting this bit to start calibration process. each bit for one channel."]
    #[inline(always)]
    #[must_use]
    pub fn cal_start(&mut self) -> CAL_START_W<HRPWM_CFG_SPEC> {
        CAL_START_W::new(self, 0)
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
#[doc = "hrpwm config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrpwm_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrpwm_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRPWM_CFG_SPEC;
impl crate::RegisterSpec for HRPWM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrpwm_cfg::R`](R) reader structure"]
impl crate::Readable for HRPWM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrpwm_cfg::W`](W) writer structure"]
impl crate::Writable for HRPWM_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hrpwm_cfg to value 0"]
impl crate::Resettable for HRPWM_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
