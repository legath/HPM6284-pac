#[doc = "Register `CAPNEG[%s]` reader"]
pub type R = crate::R<CAPNEG_SPEC>;
#[doc = "Register `CAPNEG[%s]` writer"]
pub type W = crate::W<CAPNEG_SPEC>;
#[doc = "Field `CAPNEG` reader - counter value captured at input signal falling edge"]
pub type CAPNEG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - counter value captured at input signal falling edge"]
    #[inline(always)]
    pub fn capneg(&self) -> CAPNEG_R {
        CAPNEG_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capneg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capneg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPNEG_SPEC;
impl crate::RegisterSpec for CAPNEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capneg::R`](R) reader structure"]
impl crate::Readable for CAPNEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capneg::W`](W) writer structure"]
impl crate::Writable for CAPNEG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPNEG[%s]
to value 0"]
impl crate::Resettable for CAPNEG_SPEC {
    const RESET_VALUE: u32 = 0;
}
