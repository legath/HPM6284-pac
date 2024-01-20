#[doc = "Register `spdcmp` reader"]
pub type R = crate::R<SPDCMP_SPEC>;
#[doc = "Register `spdcmp` writer"]
pub type W = crate::W<SPDCMP_SPEC>;
#[doc = "Field `SPDCMP` reader - spdcnt position compare value"]
pub type SPDCMP_R = crate::FieldReader<u32>;
#[doc = "Field `SPDCMP` writer - spdcnt position compare value"]
pub type SPDCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - spdcnt position compare value"]
    #[inline(always)]
    pub fn spdcmp(&self) -> SPDCMP_R {
        SPDCMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - spdcnt position compare value"]
    #[inline(always)]
    #[must_use]
    pub fn spdcmp(&mut self) -> SPDCMP_W<SPDCMP_SPEC> {
        SPDCMP_W::new(self, 0)
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
#[doc = "Speed comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDCMP_SPEC;
impl crate::RegisterSpec for SPDCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdcmp::R`](R) reader structure"]
impl crate::Readable for SPDCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdcmp::W`](W) writer structure"]
impl crate::Writable for SPDCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets spdcmp to value 0"]
impl crate::Resettable for SPDCMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
