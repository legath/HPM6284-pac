#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `REFSEL` reader - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M"]
pub type REFSEL_R = crate::BitReader;
#[doc = "Field `REFSEL` writer - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M"]
pub type REFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPREAD` reader - Enable spread spectrum function. This field supports changing during PLL running."]
pub type SPREAD_R = crate::BitReader;
#[doc = "Field `SPREAD` writer - Enable spread spectrum function. This field supports changing during PLL running."]
pub type SPREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable spread spectrum function. This field supports changing during PLL running."]
    #[inline(always)]
    pub fn spread(&self) -> SPREAD_R {
        SPREAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<CONFIG_SPEC> {
        REFSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable spread spectrum function. This field supports changing during PLL running."]
    #[inline(always)]
    #[must_use]
    pub fn spread(&mut self) -> SPREAD_W<CONFIG_SPEC> {
        SPREAD_W::new(self, 8)
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
#[doc = "PLL0 confguration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
