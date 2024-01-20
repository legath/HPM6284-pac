#[doc = "Register `w` reader"]
pub type R = crate::R<W_SPEC>;
#[doc = "Register `w` writer"]
pub type W = crate::W<W_SPEC>;
#[doc = "Field `WCNT` reader - wcnt counter"]
pub type WCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - wcnt counter"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new(self.bits & 0x0fff_ffff)
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
#[doc = "W counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W_SPEC;
impl crate::RegisterSpec for W_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w::R`](R) reader structure"]
impl crate::Readable for W_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w::W`](W) writer structure"]
impl crate::Writable for W_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets w to value 0"]
impl crate::Resettable for W_SPEC {
    const RESET_VALUE: u32 = 0;
}
