#[doc = "Register `FLAG` reader"]
pub type R = crate::R<FLAG_SPEC>;
#[doc = "Register `FLAG` writer"]
pub type W = crate::W<FLAG_SPEC>;
#[doc = "Field `IRQ` reader - IRQ flag, write 1 to clear"]
pub type IRQ_R = crate::BitReader;
#[doc = "Field `IRQ` writer - IRQ flag, write 1 to clear"]
pub type IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER_TEMP` reader - Clear over temperature status, write 1 to clear"]
pub type OVER_TEMP_R = crate::BitReader;
#[doc = "Field `OVER_TEMP` writer - Clear over temperature status, write 1 to clear"]
pub type OVER_TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDER_TEMP` reader - Clear under temperature status, write 1 to clear"]
pub type UNDER_TEMP_R = crate::BitReader;
#[doc = "Field `UNDER_TEMP` writer - Clear under temperature status, write 1 to clear"]
pub type UNDER_TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_MAX_CLR` reader - Clear maximum recorder of temerature, write 1 to clear"]
pub type RECORD_MAX_CLR_R = crate::BitReader;
#[doc = "Field `RECORD_MAX_CLR` writer - Clear maximum recorder of temerature, write 1 to clear"]
pub type RECORD_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_MIN_CLR` reader - Clear minimum recorder of temerature, write 1 to clear"]
pub type RECORD_MIN_CLR_R = crate::BitReader;
#[doc = "Field `RECORD_MIN_CLR` writer - Clear minimum recorder of temerature, write 1 to clear"]
pub type RECORD_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRQ flag, write 1 to clear"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Clear over temperature status, write 1 to clear"]
    #[inline(always)]
    pub fn over_temp(&self) -> OVER_TEMP_R {
        OVER_TEMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clear under temperature status, write 1 to clear"]
    #[inline(always)]
    pub fn under_temp(&self) -> UNDER_TEMP_R {
        UNDER_TEMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear maximum recorder of temerature, write 1 to clear"]
    #[inline(always)]
    pub fn record_max_clr(&self) -> RECORD_MAX_CLR_R {
        RECORD_MAX_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear minimum recorder of temerature, write 1 to clear"]
    #[inline(always)]
    pub fn record_min_clr(&self) -> RECORD_MIN_CLR_R {
        RECORD_MIN_CLR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ flag, write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<FLAG_SPEC> {
        IRQ_W::new(self, 0)
    }
    #[doc = "Bit 16 - Clear over temperature status, write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn over_temp(&mut self) -> OVER_TEMP_W<FLAG_SPEC> {
        OVER_TEMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear under temperature status, write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn under_temp(&mut self) -> UNDER_TEMP_W<FLAG_SPEC> {
        UNDER_TEMP_W::new(self, 17)
    }
    #[doc = "Bit 20 - Clear maximum recorder of temerature, write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn record_max_clr(&mut self) -> RECORD_MAX_CLR_W<FLAG_SPEC> {
        RECORD_MAX_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear minimum recorder of temerature, write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn record_min_clr(&mut self) -> RECORD_MIN_CLR_W<FLAG_SPEC> {
        RECORD_MIN_CLR_W::new(self, 21)
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
#[doc = "Temperature flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLAG_SPEC;
impl crate::RegisterSpec for FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flag::R`](R) reader structure"]
impl crate::Readable for FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flag::W`](W) writer structure"]
impl crate::Writable for FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLAG to value 0"]
impl crate::Resettable for FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
