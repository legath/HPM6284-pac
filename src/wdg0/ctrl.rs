#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Enable or disable the watchdog timer 0: Disable 1: Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable or disable the watchdog timer 0: Disable 1: Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - Clock source of timer: 0: EXTCLK 1: PCLK"]
pub type CLKSEL_R = crate::BitReader;
#[doc = "Field `CLKSEL` writer - Clock source of timer: 0: EXTCLK 1: PCLK"]
pub type CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable or disable the watchdog reset 0: Disable 1: Enable"]
pub type RSTEN_R = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable or disable the watchdog reset 0: Disable 1: Enable"]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTIME` reader - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
pub type INTTIME_R = crate::FieldReader;
#[doc = "Field `INTTIME` writer - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
pub type INTTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSTTIME` reader - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
pub type RSTTIME_R = crate::FieldReader;
#[doc = "Field `RSTTIME` writer - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
pub type RSTTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable or disable the watchdog timer 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock source of timer: 0: EXTCLK 1: PCLK"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable the watchdog reset 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
    #[inline(always)]
    pub fn inttime(&self) -> INTTIME_R {
        INTTIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
    #[inline(always)]
    pub fn rsttime(&self) -> RSTTIME_R {
        RSTTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the watchdog timer 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock source of timer: 0: EXTCLK 1: PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CTRL_SPEC> {
        CLKSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<CTRL_SPEC> {
        INTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable the watchdog reset 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<CTRL_SPEC> {
        RSTEN_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
    #[inline(always)]
    #[must_use]
    pub fn inttime(&mut self) -> INTTIME_W<CTRL_SPEC> {
        INTTIME_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
    #[inline(always)]
    #[must_use]
    pub fn rsttime(&mut self) -> RSTTIME_W<CTRL_SPEC> {
        RSTTIME_W::new(self, 8)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
