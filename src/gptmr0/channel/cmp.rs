#[doc = "Register `CMP[%s]` reader"]
pub type R = crate::R<CMP_SPEC>;
#[doc = "Register `CMP[%s]` writer"]
pub type W = crate::W<CMP_SPEC>;
#[doc = "Field `CMP` reader - compare value 0"]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - compare value 0"]
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - compare value 0"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - compare value 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<CMP_SPEC> {
        CMP_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP[%s]
to value 0xffff_ffff"]
impl crate::Resettable for CMP_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
