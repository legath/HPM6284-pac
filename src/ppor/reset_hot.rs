#[doc = "Register `RESET_HOT` reader"]
pub type R = crate::R<RESET_HOT_SPEC>;
#[doc = "Register `RESET_HOT` writer"]
pub type W = crate::W<RESET_HOT_SPEC>;
#[doc = "Field `TYPE` reader - reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type TYPE_R = crate::FieldReader<u32>;
#[doc = "Field `TYPE` writer - reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<RESET_HOT_SPEC> {
        TYPE_W::new(self, 0)
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
#[doc = "reset type triggered by reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_HOT_SPEC;
impl crate::RegisterSpec for RESET_HOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_hot::R`](R) reader structure"]
impl crate::Readable for RESET_HOT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_hot::W`](W) writer structure"]
impl crate::Writable for RESET_HOT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_HOT to value 0"]
impl crate::Resettable for RESET_HOT_SPEC {
    const RESET_VALUE: u32 = 0;
}
