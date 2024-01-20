#[doc = "Register `SECOND` reader"]
pub type R = crate::R<SECOND_SPEC>;
#[doc = "Register `SECOND` writer"]
pub type W = crate::W<SECOND_SPEC>;
#[doc = "Field `SECOND` reader - second counter"]
pub type SECOND_R = crate::FieldReader<u32>;
#[doc = "Field `SECOND` writer - second counter"]
pub type SECOND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - second counter"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - second counter"]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<SECOND_SPEC> {
        SECOND_W::new(self, 0)
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
#[doc = "Second counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`second::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`second::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECOND_SPEC;
impl crate::RegisterSpec for SECOND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`second::R`](R) reader structure"]
impl crate::Readable for SECOND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`second::W`](W) writer structure"]
impl crate::Writable for SECOND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECOND to value 0"]
impl crate::Resettable for SECOND_SPEC {
    const RESET_VALUE: u32 = 0;
}
