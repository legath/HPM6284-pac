#[doc = "Register `GPTIMER0CTRL` reader"]
pub type R = crate::R<GPTIMER0CTRL_SPEC>;
#[doc = "Register `GPTIMER0CTRL` writer"]
pub type W = crate::W<GPTIMER0CTRL_SPEC>;
#[doc = "Field `GPTCNT` reader - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer."]
pub type GPTCNT_R = crate::FieldReader<u32>;
#[doc = "Field `GPTMODE` reader - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
pub type GPTMODE_R = crate::BitReader;
#[doc = "Field `GPTMODE` writer - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
pub type GPTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTRST` writer - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD"]
pub type GPTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTRUN` reader - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
pub type GPTRUN_R = crate::BitReader;
#[doc = "Field `GPTRUN` writer - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
pub type GPTRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub fn gptcnt(&self) -> GPTCNT_R {
        GPTCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
    #[inline(always)]
    pub fn gptmode(&self) -> GPTMODE_R {
        GPTMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
    #[inline(always)]
    pub fn gptrun(&self) -> GPTRUN_R {
        GPTRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gptmode(&mut self) -> GPTMODE_W<GPTIMER0CTRL_SPEC> {
        GPTMODE_W::new(self, 24)
    }
    #[doc = "Bit 30 - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD"]
    #[inline(always)]
    #[must_use]
    pub fn gptrst(&mut self) -> GPTRST_W<GPTIMER0CTRL_SPEC> {
        GPTRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
    #[inline(always)]
    #[must_use]
    pub fn gptrun(&mut self) -> GPTRUN_W<GPTIMER0CTRL_SPEC> {
        GPTRUN_W::new(self, 31)
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
#[doc = "General Purpose Timer #0 Controller Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer0ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer0ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPTIMER0CTRL_SPEC;
impl crate::RegisterSpec for GPTIMER0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptimer0ctrl::R`](R) reader structure"]
impl crate::Readable for GPTIMER0CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gptimer0ctrl::W`](W) writer structure"]
impl crate::Writable for GPTIMER0CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTIMER0CTRL to value 0"]
impl crate::Resettable for GPTIMER0CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
