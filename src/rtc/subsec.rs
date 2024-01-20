#[doc = "Register `SUBSEC` reader"]
pub type R = crate::R<SUBSEC_SPEC>;
#[doc = "Register `SUBSEC` writer"]
pub type W = crate::W<SUBSEC_SPEC>;
#[doc = "Field `SUBSEC` reader - sub second counter"]
pub type SUBSEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sub second counter"]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new(self.bits)
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
#[doc = "Sub-second counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUBSEC_SPEC;
impl crate::RegisterSpec for SUBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsec::R`](R) reader structure"]
impl crate::Readable for SUBSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`subsec::W`](W) writer structure"]
impl crate::Writable for SUBSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SUBSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
