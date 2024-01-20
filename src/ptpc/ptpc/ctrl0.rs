#[doc = "Register `Ctrl0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `Ctrl0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `TIMER_ENABLE` reader - No description avaiable"]
pub type TIMER_ENABLE_R = crate::BitReader;
#[doc = "Field `TIMER_ENABLE` writer - No description avaiable"]
pub type TIMER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINE_COARSE_SEL` reader - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
pub type FINE_COARSE_SEL_R = crate::BitReader;
#[doc = "Field `FINE_COARSE_SEL` writer - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
pub type FINE_COARSE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT_TIMER` writer - initial timer with ts_updt, pulse, clear after set"]
pub type INIT_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE_TIMER` writer - update timer with +/- ts_updt, pulse, clear after set"]
pub type UPDATE_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_EN` reader - set to enable compare, will be cleared by HW when compare event triggered"]
pub type COMP_EN_R = crate::BitReader;
#[doc = "Field `COMP_EN` writer - set to enable compare, will be cleared by HW when compare event triggered"]
pub type COMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPT_SNAP_NEG_EN` reader - No description avaiable"]
pub type CAPT_SNAP_NEG_EN_R = crate::BitReader;
#[doc = "Field `CAPT_SNAP_NEG_EN` writer - No description avaiable"]
pub type CAPT_SNAP_NEG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPT_SNAP_POS_EN` reader - set will use posege of input capture signal to latch timestamp value"]
pub type CAPT_SNAP_POS_EN_R = crate::BitReader;
#[doc = "Field `CAPT_SNAP_POS_EN` writer - set will use posege of input capture signal to latch timestamp value"]
pub type CAPT_SNAP_POS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPT_SNAP_KEEP` reader - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
pub type CAPT_SNAP_KEEP_R = crate::BitReader;
#[doc = "Field `CAPT_SNAP_KEEP` writer - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
pub type CAPT_SNAP_KEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBSEC_DIGITAL_ROLLOVER` reader - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
pub type SUBSEC_DIGITAL_ROLLOVER_R = crate::BitReader;
#[doc = "Field `SUBSEC_DIGITAL_ROLLOVER` writer - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
pub type SUBSEC_DIGITAL_ROLLOVER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn timer_enable(&self) -> TIMER_ENABLE_R {
        TIMER_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
    #[inline(always)]
    pub fn fine_coarse_sel(&self) -> FINE_COARSE_SEL_R {
        FINE_COARSE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - set to enable compare, will be cleared by HW when compare event triggered"]
    #[inline(always)]
    pub fn comp_en(&self) -> COMP_EN_R {
        COMP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn capt_snap_neg_en(&self) -> CAPT_SNAP_NEG_EN_R {
        CAPT_SNAP_NEG_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set will use posege of input capture signal to latch timestamp value"]
    #[inline(always)]
    pub fn capt_snap_pos_en(&self) -> CAPT_SNAP_POS_EN_R {
        CAPT_SNAP_POS_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    #[inline(always)]
    pub fn capt_snap_keep(&self) -> CAPT_SNAP_KEEP_R {
        CAPT_SNAP_KEEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    #[inline(always)]
    pub fn subsec_digital_rollover(&self) -> SUBSEC_DIGITAL_ROLLOVER_R {
        SUBSEC_DIGITAL_ROLLOVER_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_enable(&mut self) -> TIMER_ENABLE_W<CTRL0_SPEC> {
        TIMER_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
    #[inline(always)]
    #[must_use]
    pub fn fine_coarse_sel(&mut self) -> FINE_COARSE_SEL_W<CTRL0_SPEC> {
        FINE_COARSE_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - initial timer with ts_updt, pulse, clear after set"]
    #[inline(always)]
    #[must_use]
    pub fn init_timer(&mut self) -> INIT_TIMER_W<CTRL0_SPEC> {
        INIT_TIMER_W::new(self, 2)
    }
    #[doc = "Bit 3 - update timer with +/- ts_updt, pulse, clear after set"]
    #[inline(always)]
    #[must_use]
    pub fn update_timer(&mut self) -> UPDATE_TIMER_W<CTRL0_SPEC> {
        UPDATE_TIMER_W::new(self, 3)
    }
    #[doc = "Bit 4 - set to enable compare, will be cleared by HW when compare event triggered"]
    #[inline(always)]
    #[must_use]
    pub fn comp_en(&mut self) -> COMP_EN_W<CTRL0_SPEC> {
        COMP_EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn capt_snap_neg_en(&mut self) -> CAPT_SNAP_NEG_EN_W<CTRL0_SPEC> {
        CAPT_SNAP_NEG_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - set will use posege of input capture signal to latch timestamp value"]
    #[inline(always)]
    #[must_use]
    pub fn capt_snap_pos_en(&mut self) -> CAPT_SNAP_POS_EN_W<CTRL0_SPEC> {
        CAPT_SNAP_POS_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    #[inline(always)]
    #[must_use]
    pub fn capt_snap_keep(&mut self) -> CAPT_SNAP_KEEP_W<CTRL0_SPEC> {
        CAPT_SNAP_KEEP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    #[inline(always)]
    #[must_use]
    pub fn subsec_digital_rollover(&mut self) -> SUBSEC_DIGITAL_ROLLOVER_W<CTRL0_SPEC> {
        SUBSEC_DIGITAL_ROLLOVER_W::new(self, 9)
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
#[doc = "Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Ctrl0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
