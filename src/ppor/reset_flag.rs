#[doc = "Register `RESET_FLAG` reader"]
pub type R = crate::R<RESET_FLAG_SPEC>;
#[doc = "Register `RESET_FLAG` writer"]
pub type W = crate::W<RESET_FLAG_SPEC>;
#[doc = "Field `FLAG` writer - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type FLAG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<RESET_FLAG_SPEC> {
        FLAG_W::new(self, 0)
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
#[doc = "flag indicate reset source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_FLAG_SPEC;
impl crate::RegisterSpec for RESET_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_flag::R`](R) reader structure"]
impl crate::Readable for RESET_FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_flag::W`](W) writer structure"]
impl crate::Writable for RESET_FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_FLAG to value 0"]
impl crate::Resettable for RESET_FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
