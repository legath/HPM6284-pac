#[doc = "Register `ALARM_EN` reader"]
pub type R = crate::R<ALARM_EN_SPEC>;
#[doc = "Register `ALARM_EN` writer"]
pub type W = crate::W<ALARM_EN_SPEC>;
#[doc = "Field `ENABLE0` reader - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
pub type ENABLE0_R = crate::BitReader;
#[doc = "Field `ENABLE0` writer - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
pub type ENABLE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE1` reader - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
pub type ENABLE1_R = crate::BitReader;
#[doc = "Field `ENABLE1` writer - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
pub type ENABLE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable0(&mut self) -> ENABLE0_W<ALARM_EN_SPEC> {
        ENABLE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable1(&mut self) -> ENABLE1_W<ALARM_EN_SPEC> {
        ENABLE1_W::new(self, 1)
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
#[doc = "RTC alarm enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM_EN_SPEC;
impl crate::RegisterSpec for ALARM_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm_en::R`](R) reader structure"]
impl crate::Readable for ALARM_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm_en::W`](W) writer structure"]
impl crate::Writable for ALARM_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM_EN to value 0"]
impl crate::Resettable for ALARM_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
