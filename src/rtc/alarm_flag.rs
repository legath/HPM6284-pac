#[doc = "Register `ALARM_FLAG` reader"]
pub type R = crate::R<ALARM_FLAG_SPEC>;
#[doc = "Register `ALARM_FLAG` writer"]
pub type W = crate::W<ALARM_FLAG_SPEC>;
#[doc = "Field `ALARM0` reader - alarm0 happen"]
pub type ALARM0_R = crate::BitReader;
#[doc = "Field `ALARM0` writer - alarm0 happen"]
pub type ALARM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM1` reader - alarm1 happen"]
pub type ALARM1_R = crate::BitReader;
#[doc = "Field `ALARM1` writer - alarm1 happen"]
pub type ALARM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - alarm0 happen"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - alarm1 happen"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - alarm0 happen"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<ALARM_FLAG_SPEC> {
        ALARM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - alarm1 happen"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> ALARM1_W<ALARM_FLAG_SPEC> {
        ALARM1_W::new(self, 1)
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
#[doc = "RTC alarm flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM_FLAG_SPEC;
impl crate::RegisterSpec for ALARM_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm_flag::R`](R) reader structure"]
impl crate::Readable for ALARM_FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm_flag::W`](W) writer structure"]
impl crate::Writable for ALARM_FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM_FLAG to value 0"]
impl crate::Resettable for ALARM_FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
