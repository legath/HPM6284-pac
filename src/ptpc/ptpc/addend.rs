#[doc = "Register `addend` reader"]
pub type R = crate::R<ADDEND_SPEC>;
#[doc = "Register `addend` writer"]
pub type W = crate::W<ADDEND_SPEC>;
#[doc = "Field `ADDEND` reader - used in fine update mode only"]
pub type ADDEND_R = crate::FieldReader<u32>;
#[doc = "Field `ADDEND` writer - used in fine update mode only"]
pub type ADDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - used in fine update mode only"]
    #[inline(always)]
    pub fn addend(&self) -> ADDEND_R {
        ADDEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used in fine update mode only"]
    #[inline(always)]
    #[must_use]
    pub fn addend(&mut self) -> ADDEND_W<ADDEND_SPEC> {
        ADDEND_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDEND_SPEC;
impl crate::RegisterSpec for ADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addend::R`](R) reader structure"]
impl crate::Readable for ADDEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addend::W`](W) writer structure"]
impl crate::Writable for ADDEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets addend to value 0"]
impl crate::Resettable for ADDEND_SPEC {
    const RESET_VALUE: u32 = 0;
}
