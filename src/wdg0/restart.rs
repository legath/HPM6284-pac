#[doc = "Register `Restart` reader"]
pub type R = crate::R<RESTART_SPEC>;
#[doc = "Register `Restart` writer"]
pub type W = crate::W<RESTART_SPEC>;
#[doc = "Field `RESTART` writer - Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
pub type RESTART_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<RESTART_SPEC> {
        RESTART_W::new(self, 0)
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
#[doc = "Restart Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`restart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`restart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restart::R`](R) reader structure"]
impl crate::Readable for RESTART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`restart::W`](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Restart to value 0"]
impl crate::Resettable for RESTART_SPEC {
    const RESET_VALUE: u32 = 0;
}
