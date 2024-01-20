#[doc = "Register `ADVANCED` reader"]
pub type R = crate::R<ADVANCED_SPEC>;
#[doc = "Register `ADVANCED` writer"]
pub type W = crate::W<ADVANCED_SPEC>;
#[doc = "Field `DITHER` reader - Enable dither function"]
pub type DITHER_R = crate::BitReader;
#[doc = "Field `DITHER` writer - Enable dither function"]
pub type DITHER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW` reader - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us"]
pub type SLOW_R = crate::BitReader;
#[doc = "Field `SLOW` writer - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us"]
pub type SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Enable dither function"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us"]
    #[inline(always)]
    pub fn slow(&self) -> SLOW_R {
        SLOW_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enable dither function"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<ADVANCED_SPEC> {
        DITHER_W::new(self, 24)
    }
    #[doc = "Bit 28 - Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us"]
    #[inline(always)]
    #[must_use]
    pub fn slow(&mut self) -> SLOW_W<ADVANCED_SPEC> {
        SLOW_W::new(self, 28)
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
#[doc = "PLL0 advance configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`advanced::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADVANCED_SPEC;
impl crate::RegisterSpec for ADVANCED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`advanced::R`](R) reader structure"]
impl crate::Readable for ADVANCED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`advanced::W`](W) writer structure"]
impl crate::Writable for ADVANCED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADVANCED to value 0"]
impl crate::Resettable for ADVANCED_SPEC {
    const RESET_VALUE: u32 = 0;
}
