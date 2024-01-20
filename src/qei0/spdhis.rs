#[doc = "Register `SPDHIS[%s]` reader"]
pub type R = crate::R<SPDHIS_SPEC>;
#[doc = "Register `SPDHIS[%s]` writer"]
pub type W = crate::W<SPDHIS_SPEC>;
#[doc = "Field `SPDHIS0` reader - copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
pub type SPDHIS0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    #[inline(always)]
    pub fn spdhis0(&self) -> SPDHIS0_R {
        SPDHIS0_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdhis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdhis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDHIS_SPEC;
impl crate::RegisterSpec for SPDHIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdhis::R`](R) reader structure"]
impl crate::Readable for SPDHIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdhis::W`](W) writer structure"]
impl crate::Writable for SPDHIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDHIS[%s]
to value 0"]
impl crate::Resettable for SPDHIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
