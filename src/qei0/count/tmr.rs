#[doc = "Register `tmr` reader"]
pub type R = crate::R<TMR_SPEC>;
#[doc = "Register `tmr` writer"]
pub type W = crate::W<TMR_SPEC>;
#[doc = "Field `TMRCNT` reader - 32 bit free run timer"]
pub type TMRCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit free run timer"]
    #[inline(always)]
    pub fn tmrcnt(&self) -> TMRCNT_R {
        TMRCNT_R::new(self.bits)
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
#[doc = "Timer counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tmr to value 0"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
