#[doc = "Register `AGE` reader"]
pub type R = crate::R<AGE_SPEC>;
#[doc = "Register `AGE` writer"]
pub type W = crate::W<AGE_SPEC>;
#[doc = "Field `AGE` reader - age of T register in 24MHz clock cycles"]
pub type AGE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - age of T register in 24MHz clock cycles"]
    #[inline(always)]
    pub fn age(&self) -> AGE_R {
        AGE_R::new(self.bits)
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
#[doc = "Sample age\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`age::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`age::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGE_SPEC;
impl crate::RegisterSpec for AGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`age::R`](R) reader structure"]
impl crate::Readable for AGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`age::W`](W) writer structure"]
impl crate::Writable for AGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGE to value 0"]
impl crate::Resettable for AGE_SPEC {
    const RESET_VALUE: u32 = 0;
}
