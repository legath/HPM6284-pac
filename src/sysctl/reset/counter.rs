#[doc = "Register `counter` reader"]
pub type R = crate::R<COUNTER_SPEC>;
#[doc = "Register `counter` writer"]
pub type W = crate::W<COUNTER_SPEC>;
#[doc = "Field `COUNTER` reader - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type COUNTER_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER` writer - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<COUNTER_SPEC> {
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
#[doc = "Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for COUNTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets counter to value 0x03"]
impl crate::Resettable for COUNTER_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
