#[doc = "Register `SOFTWARE_RESET` reader"]
pub type R = crate::R<SOFTWARE_RESET_SPEC>;
#[doc = "Register `SOFTWARE_RESET` writer"]
pub type W = crate::W<SOFTWARE_RESET_SPEC>;
#[doc = "Field `COUNTER` reader - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
pub type COUNTER_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER` writer - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<SOFTWARE_RESET_SPEC> {
        COUNTER_W::new(self, 0)
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
#[doc = "Software reset counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`software_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`software_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFTWARE_RESET_SPEC;
impl crate::RegisterSpec for SOFTWARE_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`software_reset::R`](R) reader structure"]
impl crate::Readable for SOFTWARE_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`software_reset::W`](W) writer structure"]
impl crate::Writable for SOFTWARE_RESET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTWARE_RESET to value 0"]
impl crate::Resettable for SOFTWARE_RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
