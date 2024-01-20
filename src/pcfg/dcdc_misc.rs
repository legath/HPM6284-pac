#[doc = "Register `DCDC_MISC` reader"]
pub type R = crate::R<DCDC_MISC_SPEC>;
#[doc = "Register `DCDC_MISC` writer"]
pub type W = crate::W<DCDC_MISC_SPEC>;
#[doc = "Field `EN_STEP` reader - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
pub type EN_STEP_R = crate::BitReader;
#[doc = "Field `EN_STEP` writer - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
pub type EN_STEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SEL` reader - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
pub type CLK_SEL_R = crate::BitReader;
#[doc = "Field `CLK_SEL` writer - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
pub type CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY` reader - enable delay 0: delay disabled, 1: delay enabled"]
pub type DELAY_R = crate::BitReader;
#[doc = "Field `DELAY` writer - enable delay 0: delay disabled, 1: delay enabled"]
pub type DELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OL_HYST` reader - current hysteres range 0: 12.5mV 1: 25mV"]
pub type OL_HYST_R = crate::BitReader;
#[doc = "Field `OL_HYST` writer - current hysteres range 0: 12.5mV 1: 25mV"]
pub type OL_HYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OL_THRE` reader - overload for threshold for lod power mode"]
pub type OL_THRE_R = crate::FieldReader;
#[doc = "Field `OL_THRE` writer - overload for threshold for lod power mode"]
pub type OL_THRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DC_FF` reader - Loop feed forward number"]
pub type DC_FF_R = crate::FieldReader;
#[doc = "Field `DC_FF` writer - Loop feed forward number"]
pub type DC_FF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RC_SCALE` reader - Loop RC scale threshold"]
pub type RC_SCALE_R = crate::BitReader;
#[doc = "Field `RC_SCALE` writer - Loop RC scale threshold"]
pub type RC_SCALE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_THRS` reader - hysteres threshold"]
pub type HYST_THRS_R = crate::BitReader;
#[doc = "Field `HYST_THRS` writer - hysteres threshold"]
pub type HYST_THRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_SIGN` reader - hysteres sign"]
pub type HYST_SIGN_R = crate::BitReader;
#[doc = "Field `HYST_SIGN` writer - hysteres sign"]
pub type HYST_SIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_HYST` reader - hysteres enable"]
pub type EN_HYST_R = crate::BitReader;
#[doc = "Field `EN_HYST` writer - hysteres enable"]
pub type EN_HYST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
    #[inline(always)]
    pub fn en_step(&self) -> EN_STEP_R {
        EN_STEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable delay 0: delay disabled, 1: delay enabled"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - current hysteres range 0: 12.5mV 1: 25mV"]
    #[inline(always)]
    pub fn ol_hyst(&self) -> OL_HYST_R {
        OL_HYST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - overload for threshold for lod power mode"]
    #[inline(always)]
    pub fn ol_thre(&self) -> OL_THRE_R {
        OL_THRE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Loop feed forward number"]
    #[inline(always)]
    pub fn dc_ff(&self) -> DC_FF_R {
        DC_FF_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Loop RC scale threshold"]
    #[inline(always)]
    pub fn rc_scale(&self) -> RC_SCALE_R {
        RC_SCALE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - hysteres threshold"]
    #[inline(always)]
    pub fn hyst_thrs(&self) -> HYST_THRS_R {
        HYST_THRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - hysteres sign"]
    #[inline(always)]
    pub fn hyst_sign(&self) -> HYST_SIGN_R {
        HYST_SIGN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - hysteres enable"]
    #[inline(always)]
    pub fn en_hyst(&self) -> EN_HYST_R {
        EN_HYST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en_step(&mut self) -> EN_STEP_W<DCDC_MISC_SPEC> {
        EN_STEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<DCDC_MISC_SPEC> {
        CLK_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable delay 0: delay disabled, 1: delay enabled"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<DCDC_MISC_SPEC> {
        DELAY_W::new(self, 2)
    }
    #[doc = "Bit 4 - current hysteres range 0: 12.5mV 1: 25mV"]
    #[inline(always)]
    #[must_use]
    pub fn ol_hyst(&mut self) -> OL_HYST_W<DCDC_MISC_SPEC> {
        OL_HYST_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - overload for threshold for lod power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ol_thre(&mut self) -> OL_THRE_W<DCDC_MISC_SPEC> {
        OL_THRE_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Loop feed forward number"]
    #[inline(always)]
    #[must_use]
    pub fn dc_ff(&mut self) -> DC_FF_W<DCDC_MISC_SPEC> {
        DC_FF_W::new(self, 16)
    }
    #[doc = "Bit 20 - Loop RC scale threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rc_scale(&mut self) -> RC_SCALE_W<DCDC_MISC_SPEC> {
        RC_SCALE_W::new(self, 20)
    }
    #[doc = "Bit 24 - hysteres threshold"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_thrs(&mut self) -> HYST_THRS_W<DCDC_MISC_SPEC> {
        HYST_THRS_W::new(self, 24)
    }
    #[doc = "Bit 25 - hysteres sign"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_sign(&mut self) -> HYST_SIGN_W<DCDC_MISC_SPEC> {
        HYST_SIGN_W::new(self, 25)
    }
    #[doc = "Bit 28 - hysteres enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_hyst(&mut self) -> EN_HYST_W<DCDC_MISC_SPEC> {
        EN_HYST_W::new(self, 28)
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
#[doc = "DCDC misc parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_MISC_SPEC;
impl crate::RegisterSpec for DCDC_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_misc::R`](R) reader structure"]
impl crate::Readable for DCDC_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_misc::W`](W) writer structure"]
impl crate::Writable for DCDC_MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_MISC to value 0x0007_0100"]
impl crate::Resettable for DCDC_MISC_SPEC {
    const RESET_VALUE: u32 = 0x0007_0100;
}
