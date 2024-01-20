#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `RESET` reader - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD` reader - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
pub type HOLD_R = crate::BitReader;
#[doc = "Field `HOLD` writer - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_WAKE` reader - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
pub type FLAG_WAKE_R = crate::BitReader;
#[doc = "Field `FLAG_WAKE` writer - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
pub type FLAG_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG` reader - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
pub type FLAG_R = crate::BitReader;
#[doc = "Field `FLAG` writer - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
pub type FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag_wake(&self) -> FLAG_WAKE_R {
        FLAG_WAKE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CONTROL_SPEC> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HOLD_W<CONTROL_SPEC> {
        HOLD_W::new(self, 4)
    }
    #[doc = "Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag_wake(&mut self) -> FLAG_WAKE_W<CONTROL_SPEC> {
        FLAG_WAKE_W::new(self, 30)
    }
    #[doc = "Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<CONTROL_SPEC> {
        FLAG_W::new(self, 31)
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
#[doc = "Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0x8000_0000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
