#[doc = "Register `STEPTIME` reader"]
pub type R = crate::R<STEPTIME_SPEC>;
#[doc = "Register `STEPTIME` writer"]
pub type W = crate::W<STEPTIME_SPEC>;
#[doc = "Field `STEPTIME` reader - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
pub type STEPTIME_R = crate::FieldReader<u16>;
#[doc = "Field `STEPTIME` writer - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
pub type STEPTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
    #[inline(always)]
    pub fn steptime(&self) -> STEPTIME_R {
        STEPTIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
    #[inline(always)]
    #[must_use]
    pub fn steptime(&mut self) -> STEPTIME_W<STEPTIME_SPEC> {
        STEPTIME_W::new(self, 0)
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
#[doc = "PLL0 step time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`steptime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`steptime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STEPTIME_SPEC;
impl crate::RegisterSpec for STEPTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`steptime::R`](R) reader structure"]
impl crate::Readable for STEPTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`steptime::W`](W) writer structure"]
impl crate::Writable for STEPTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STEPTIME to value 0x09c4"]
impl crate::Resettable for STEPTIME_SPEC {
    const RESET_VALUE: u32 = 0x09c4;
}
