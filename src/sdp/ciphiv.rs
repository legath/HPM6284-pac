#[doc = "Register `CIPHIV[%s]` reader"]
pub type R = crate::R<CIPHIV_SPEC>;
#[doc = "Register `CIPHIV[%s]` writer"]
pub type W = crate::W<CIPHIV_SPEC>;
#[doc = "Field `CIPHIV` reader - cipher initialization vector."]
pub type CIPHIV_R = crate::FieldReader<u32>;
#[doc = "Field `CIPHIV` writer - cipher initialization vector."]
pub type CIPHIV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cipher initialization vector."]
    #[inline(always)]
    pub fn ciphiv(&self) -> CIPHIV_R {
        CIPHIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cipher initialization vector."]
    #[inline(always)]
    #[must_use]
    pub fn ciphiv(&mut self) -> CIPHIV_W<CIPHIV_SPEC> {
        CIPHIV_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciphiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciphiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIPHIV_SPEC;
impl crate::RegisterSpec for CIPHIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciphiv::R`](R) reader structure"]
impl crate::Readable for CIPHIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ciphiv::W`](W) writer structure"]
impl crate::Writable for CIPHIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIPHIV[%s]
to value 0"]
impl crate::Resettable for CIPHIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
