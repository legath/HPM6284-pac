#[doc = "Register `CAPPOS` reader"]
pub type R = crate::R<CAPPOS_SPEC>;
#[doc = "Register `CAPPOS` writer"]
pub type W = crate::W<CAPPOS_SPEC>;
#[doc = "Field `CAPPOS` reader - This register contains the counter value captured at input signal rising edge"]
pub type CAPPOS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the counter value captured at input signal rising edge"]
    #[inline(always)]
    pub fn cappos(&self) -> CAPPOS_R {
        CAPPOS_R::new(self.bits)
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
#[doc = "Capture rising edge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cappos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cappos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPPOS_SPEC;
impl crate::RegisterSpec for CAPPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cappos::R`](R) reader structure"]
impl crate::Readable for CAPPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cappos::W`](W) writer structure"]
impl crate::Writable for CAPPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPPOS to value 0"]
impl crate::Resettable for CAPPOS_SPEC {
    const RESET_VALUE: u32 = 0;
}
