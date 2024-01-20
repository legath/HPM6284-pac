#[doc = "Register `current` reader"]
pub type R = crate::R<CURRENT_SPEC>;
#[doc = "Register `current` writer"]
pub type W = crate::W<CURRENT_SPEC>;
#[doc = "Field `FREQUENCY` reader - self updating measure result"]
pub type FREQUENCY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - self updating measure result"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
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
#[doc = "Clock measure result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_SPEC;
impl crate::RegisterSpec for CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current::R`](R) reader structure"]
impl crate::Readable for CURRENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`current::W`](W) writer structure"]
impl crate::Writable for CURRENT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets current to value 0"]
impl crate::Resettable for CURRENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
