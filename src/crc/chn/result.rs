#[doc = "Register `result` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Register `result` writer"]
pub type W = crate::W<RESULT_SPEC>;
#[doc = "Field `RESULT` reader - crc result"]
pub type RESULT_R = crate::FieldReader<u32>;
#[doc = "Field `RESULT` writer - crc result"]
pub type RESULT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - crc result"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - crc result"]
    #[inline(always)]
    #[must_use]
    pub fn result(&mut self) -> RESULT_W<RESULT_SPEC> {
        RESULT_W::new(self, 0)
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
#[doc = "chn&amp;index0 result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets result to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
