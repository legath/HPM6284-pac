#[doc = "Register `ATBH` reader"]
pub type R = crate::R<ATBH_SPEC>;
#[doc = "Register `ATBH` writer"]
pub type W = crate::W<ATBH_SPEC>;
#[doc = "Field `TBH` reader - timebase for timestamp generation 63-32"]
pub type TBH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - timebase for timestamp generation 63-32"]
    #[inline(always)]
    pub fn tbh(&self) -> TBH_R {
        TBH_R::new(self.bits)
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
#[doc = "actual timebase high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atbh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atbh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATBH_SPEC;
impl crate::RegisterSpec for ATBH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atbh::R`](R) reader structure"]
impl crate::Readable for ATBH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atbh::W`](W) writer structure"]
impl crate::Writable for ATBH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATBH to value 0"]
impl crate::Resettable for ATBH_SPEC {
    const RESET_VALUE: u32 = 0;
}
