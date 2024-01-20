#[doc = "Register `SDATA` reader"]
pub type R = crate::R<SDATA_SPEC>;
#[doc = "Register `SDATA` writer"]
pub type W = crate::W<SDATA_SPEC>;
#[doc = "Field `VAL` reader - Data"]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
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
#[doc = "Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDATA_SPEC;
impl crate::RegisterSpec for SDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdata::R`](R) reader structure"]
impl crate::Readable for SDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdata::W`](W) writer structure"]
impl crate::Writable for SDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDATA to value 0"]
impl crate::Resettable for SDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
