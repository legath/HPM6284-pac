#[doc = "Register `XTAL` reader"]
pub type R = crate::R<XTAL_SPEC>;
#[doc = "Register `XTAL` writer"]
pub type W = crate::W<XTAL_SPEC>;
#[doc = "Field `RAMP_TIME` reader - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
pub type RAMP_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `RAMP_TIME` writer - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
pub type RAMP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ENABLE` reader - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `RESPONSE` reader - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
pub type RESPONSE_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
    #[inline(always)]
    pub fn ramp_time(&self) -> RAMP_TIME_R {
        RAMP_TIME_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_time(&mut self) -> RAMP_TIME_W<XTAL_SPEC> {
        RAMP_TIME_W::new(self, 0)
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
#[doc = "OSC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL_SPEC;
impl crate::RegisterSpec for XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal::R`](R) reader structure"]
impl crate::Readable for XTAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal::W`](W) writer structure"]
impl crate::Writable for XTAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL to value 0x0001_ffff"]
impl crate::Resettable for XTAL_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
