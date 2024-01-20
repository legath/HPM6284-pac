#[doc = "Register `SCAMP` reader"]
pub type R = crate::R<SCAMP_SPEC>;
#[doc = "Register `SCAMP` writer"]
pub type W = crate::W<SCAMP_SPEC>;
#[doc = "Field `VAL` reader - instant Amplitude Results"]
pub type VAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - instant Amplitude Results"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
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
#[doc = "instant Amplitude Results\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCAMP_SPEC;
impl crate::RegisterSpec for SCAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scamp::R`](R) reader structure"]
impl crate::Readable for SCAMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scamp::W`](W) writer structure"]
impl crate::Writable for SCAMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAMP to value 0"]
impl crate::Resettable for SCAMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
