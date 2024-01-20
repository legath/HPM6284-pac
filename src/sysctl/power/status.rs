#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `LF_ACK` reader - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
pub type LF_ACK_R = crate::BitReader;
#[doc = "Field `LF_DISABLE` reader - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
pub type LF_DISABLE_R = crate::BitReader;
#[doc = "Field `FLAG_WAKE` reader - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
pub type FLAG_WAKE_R = crate::BitReader;
#[doc = "Field `FLAG_WAKE` writer - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
pub type FLAG_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG` reader - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
pub type FLAG_R = crate::BitReader;
#[doc = "Field `FLAG` writer - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
pub type FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    #[inline(always)]
    pub fn lf_ack(&self) -> LF_ACK_R {
        LF_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    #[inline(always)]
    pub fn lf_disable(&self) -> LF_DISABLE_R {
        LF_DISABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag_wake(&self) -> FLAG_WAKE_R {
        FLAG_WAKE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag_wake(&mut self) -> FLAG_WAKE_W<STATUS_SPEC> {
        FLAG_WAKE_W::new(self, 30)
    }
    #[doc = "Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<STATUS_SPEC> {
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
#[doc = "Power Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0x8000_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
