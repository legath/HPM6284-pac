#[doc = "Register `phidx` reader"]
pub type R = crate::R<PHIDX_SPEC>;
#[doc = "Register `phidx` writer"]
pub type W = crate::W<PHIDX_SPEC>;
#[doc = "Field `PHIDX` reader - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
pub type PHIDX_R = crate::FieldReader<u32>;
#[doc = "Field `PHIDX` writer - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
pub type PHIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
    #[inline(always)]
    pub fn phidx(&self) -> PHIDX_R {
        PHIDX_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn phidx(&mut self) -> PHIDX_W<PHIDX_SPEC> {
        PHIDX_W::new(self, 0)
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
#[doc = "Phase index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phidx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phidx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHIDX_SPEC;
impl crate::RegisterSpec for PHIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phidx::R`](R) reader structure"]
impl crate::Readable for PHIDX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phidx::W`](W) writer structure"]
impl crate::Writable for PHIDX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phidx to value 0"]
impl crate::Resettable for PHIDX_SPEC {
    const RESET_VALUE: u32 = 0;
}
