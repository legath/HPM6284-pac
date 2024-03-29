#[doc = "Register `ALARM0` reader"]
pub type R = crate::R<ALARM0_SPEC>;
#[doc = "Register `ALARM0` writer"]
pub type W = crate::W<ALARM0_SPEC>;
#[doc = "Field `ALARM` reader - Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
pub type ALARM_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM` writer - Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
pub type ALARM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
    #[inline(always)]
    #[must_use]
    pub fn alarm(&mut self) -> ALARM_W<ALARM0_SPEC> {
        ALARM_W::new(self, 0)
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
#[doc = "RTC alarm0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_SPEC;
impl crate::RegisterSpec for ALARM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0::R`](R) reader structure"]
impl crate::Readable for ALARM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0::W`](W) writer structure"]
impl crate::Writable for ALARM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM0 to value 0"]
impl crate::Resettable for ALARM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
