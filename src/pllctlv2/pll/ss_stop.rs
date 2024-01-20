#[doc = "Register `SS_STOP` reader"]
pub type R = crate::R<SS_STOP_SPEC>;
#[doc = "Register `SS_STOP` writer"]
pub type W = crate::W<SS_STOP_SPEC>;
#[doc = "Field `STOP` reader - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
pub type STOP_R = crate::FieldReader<u32>;
#[doc = "Field `STOP` writer - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<SS_STOP_SPEC> {
        STOP_W::new(self, 0)
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
#[doc = "PLL0 spread spectrum stop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_STOP_SPEC;
impl crate::RegisterSpec for SS_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_stop::R`](R) reader structure"]
impl crate::Readable for SS_STOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_stop::W`](W) writer structure"]
impl crate::Writable for SS_STOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_STOP to value 0"]
impl crate::Resettable for SS_STOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
