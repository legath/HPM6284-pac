#[doc = "Register `frcmd` reader"]
pub type R = crate::R<FRCMD_SPEC>;
#[doc = "Register `frcmd` writer"]
pub type W = crate::W<FRCMD_SPEC>;
#[doc = "Field `FRCMD` reader - 2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
pub type FRCMD_R = crate::FieldReader<u16>;
#[doc = "Field `FRCMD` writer - 2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
pub type FRCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
    #[inline(always)]
    pub fn frcmd(&self) -> FRCMD_R {
        FRCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
    #[inline(always)]
    #[must_use]
    pub fn frcmd(&mut self) -> FRCMD_W<FRCMD_SPEC> {
        FRCMD_W::new(self, 0)
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
#[doc = "Force output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRCMD_SPEC;
impl crate::RegisterSpec for FRCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frcmd::R`](R) reader structure"]
impl crate::Readable for FRCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frcmd::W`](W) writer structure"]
impl crate::Writable for FRCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets frcmd to value 0"]
impl crate::Resettable for FRCMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
