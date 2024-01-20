#[doc = "Register `CAPDTY` reader"]
pub type R = crate::R<CAPDTY_SPEC>;
#[doc = "Register `CAPDTY` writer"]
pub type W = crate::W<CAPDTY_SPEC>;
#[doc = "Field `MEAS_HIGH` reader - This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
pub type MEAS_HIGH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    #[inline(always)]
    pub fn meas_high(&self) -> MEAS_HIGH_R {
        MEAS_HIGH_R::new(self.bits)
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
#[doc = "PWM duty cycle measure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capdty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capdty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPDTY_SPEC;
impl crate::RegisterSpec for CAPDTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capdty::R`](R) reader structure"]
impl crate::Readable for CAPDTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capdty::W`](W) writer structure"]
impl crate::Writable for CAPDTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPDTY to value 0"]
impl crate::Resettable for CAPDTY_SPEC {
    const RESET_VALUE: u32 = 0;
}
