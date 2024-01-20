#[doc = "Register `SCHTLZ` reader"]
pub type R = crate::R<SCHTLZ_SPEC>;
#[doc = "Register `SCHTLZ` writer"]
pub type W = crate::W<SCHTLZ_SPEC>;
#[doc = "Field `VAL` reader - Amplitude Threshold for zero crossing"]
pub type VAL_R = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Amplitude Threshold for zero crossing"]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Amplitude Threshold for zero crossing"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Amplitude Threshold for zero crossing"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<SCHTLZ_SPEC> {
        VAL_W::new(self, 0)
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
#[doc = "Amplitude Threshold for zero crossing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`schtlz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`schtlz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCHTLZ_SPEC;
impl crate::RegisterSpec for SCHTLZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`schtlz::R`](R) reader structure"]
impl crate::Readable for SCHTLZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`schtlz::W`](W) writer structure"]
impl crate::Writable for SCHTLZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCHTLZ to value 0"]
impl crate::Resettable for SCHTLZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
