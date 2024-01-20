#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `SELECTION` reader - clock measurement selection 0: clk_32k 1: clk_irc24m 2: clk_xtal_24m 3: clk_usb0_phy 8: clk0_osc0 9: clk0_pll0 10: clk1_pll0 11: clk2_pll0 12: clk0_pll1 13: clk1_pll1 14: clk0_pll2 15: clk1_pll2 128: clk_top_cpu0 129: clk_top_mct0 130: clk_top_mct1 131: clk_top_xpi0 132: clk_top_tmr0 133: clk_top_tmr1 134: clk_top_tmr2 135: clk_top_tmr3 136: clk_top_urt0 137: clk_top_urt1 138: clk_top_urt2 139: clk_top_urt3 140: clk_top_urt4 141: clk_top_urt5 142: clk_top_urt6 143: clk_top_urt7 144: clk_top_i2c0 145: clk_top_i2c1 146: clk_top_i2c2 147: clk_top_i2c3 148: clk_top_spi0 149: clk_top_spi1 150: clk_top_spi2 151: clk_top_spi3 152: clk_top_can0 153: clk_top_can1 154: clk_top_can2 155: clk_top_can3 156: clk_top_ptpc 157: clk_top_ana0 158: clk_top_ana1 159: clk_top_ana2 160: clk_top_ana3 161: clk_top_ana4 162: clk_top_ref0 163: clk_top_ref1 164: clk_top_lin0 165: clk_top_lin1 166: clk_top_lin2 167: clk_top_lin3"]
pub type SELECTION_R = crate::FieldReader;
#[doc = "Field `SELECTION` writer - clock measurement selection 0: clk_32k 1: clk_irc24m 2: clk_xtal_24m 3: clk_usb0_phy 8: clk0_osc0 9: clk0_pll0 10: clk1_pll0 11: clk2_pll0 12: clk0_pll1 13: clk1_pll1 14: clk0_pll2 15: clk1_pll2 128: clk_top_cpu0 129: clk_top_mct0 130: clk_top_mct1 131: clk_top_xpi0 132: clk_top_tmr0 133: clk_top_tmr1 134: clk_top_tmr2 135: clk_top_tmr3 136: clk_top_urt0 137: clk_top_urt1 138: clk_top_urt2 139: clk_top_urt3 140: clk_top_urt4 141: clk_top_urt5 142: clk_top_urt6 143: clk_top_urt7 144: clk_top_i2c0 145: clk_top_i2c1 146: clk_top_i2c2 147: clk_top_i2c3 148: clk_top_spi0 149: clk_top_spi1 150: clk_top_spi2 151: clk_top_spi3 152: clk_top_can0 153: clk_top_can1 154: clk_top_can2 155: clk_top_can3 156: clk_top_ptpc 157: clk_top_ana0 158: clk_top_ana1 159: clk_top_ana2 160: clk_top_ana3 161: clk_top_ana4 162: clk_top_ref0 163: clk_top_ref1 164: clk_top_lin0 165: clk_top_lin1 166: clk_top_lin2 167: clk_top_lin3"]
pub type SELECTION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REFERENCE` reader - refrence clock selection, 0: 32k 1: 24M"]
pub type REFERENCE_R = crate::BitReader;
#[doc = "Field `REFERENCE` writer - refrence clock selection, 0: 32k 1: 24M"]
pub type REFERENCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCURACY` reader - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
pub type ACCURACY_R = crate::BitReader;
#[doc = "Field `ACCURACY` writer - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
pub type ACCURACY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - start measurement"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - start measurement"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOW` reader - clock frequency lower than lower limit"]
pub type LOW_R = crate::BitReader;
#[doc = "Field `LOW` writer - clock frequency lower than lower limit"]
pub type LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGH` reader - clock frequency higher than upper limit"]
pub type HIGH_R = crate::BitReader;
#[doc = "Field `HIGH` writer - clock frequency higher than upper limit"]
pub type HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - output divider"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - output divider"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTEN` reader - enable clock output"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - enable clock output"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_BUSY` reader - divider is applying new setting"]
pub type DIV_BUSY_R = crate::BitReader;
#[doc = "Field `VALID` reader - result is ready for read 0: not ready 1: result is ready"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - result is ready for read 0: not ready 1: result is ready"]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - clock measurement selection 0: clk_32k 1: clk_irc24m 2: clk_xtal_24m 3: clk_usb0_phy 8: clk0_osc0 9: clk0_pll0 10: clk1_pll0 11: clk2_pll0 12: clk0_pll1 13: clk1_pll1 14: clk0_pll2 15: clk1_pll2 128: clk_top_cpu0 129: clk_top_mct0 130: clk_top_mct1 131: clk_top_xpi0 132: clk_top_tmr0 133: clk_top_tmr1 134: clk_top_tmr2 135: clk_top_tmr3 136: clk_top_urt0 137: clk_top_urt1 138: clk_top_urt2 139: clk_top_urt3 140: clk_top_urt4 141: clk_top_urt5 142: clk_top_urt6 143: clk_top_urt7 144: clk_top_i2c0 145: clk_top_i2c1 146: clk_top_i2c2 147: clk_top_i2c3 148: clk_top_spi0 149: clk_top_spi1 150: clk_top_spi2 151: clk_top_spi3 152: clk_top_can0 153: clk_top_can1 154: clk_top_can2 155: clk_top_can3 156: clk_top_ptpc 157: clk_top_ana0 158: clk_top_ana1 159: clk_top_ana2 160: clk_top_ana3 161: clk_top_ana4 162: clk_top_ref0 163: clk_top_ref1 164: clk_top_lin0 165: clk_top_lin1 166: clk_top_lin2 167: clk_top_lin3"]
    #[inline(always)]
    pub fn selection(&self) -> SELECTION_R {
        SELECTION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - refrence clock selection, 0: 32k 1: 24M"]
    #[inline(always)]
    pub fn reference(&self) -> REFERENCE_R {
        REFERENCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - start measurement"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - clock frequency lower than lower limit"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clock frequency higher than upper limit"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - output divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - enable clock output"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - divider is applying new setting"]
    #[inline(always)]
    pub fn div_busy(&self) -> DIV_BUSY_R {
        DIV_BUSY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - result is ready for read 0: not ready 1: result is ready"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock measurement selection 0: clk_32k 1: clk_irc24m 2: clk_xtal_24m 3: clk_usb0_phy 8: clk0_osc0 9: clk0_pll0 10: clk1_pll0 11: clk2_pll0 12: clk0_pll1 13: clk1_pll1 14: clk0_pll2 15: clk1_pll2 128: clk_top_cpu0 129: clk_top_mct0 130: clk_top_mct1 131: clk_top_xpi0 132: clk_top_tmr0 133: clk_top_tmr1 134: clk_top_tmr2 135: clk_top_tmr3 136: clk_top_urt0 137: clk_top_urt1 138: clk_top_urt2 139: clk_top_urt3 140: clk_top_urt4 141: clk_top_urt5 142: clk_top_urt6 143: clk_top_urt7 144: clk_top_i2c0 145: clk_top_i2c1 146: clk_top_i2c2 147: clk_top_i2c3 148: clk_top_spi0 149: clk_top_spi1 150: clk_top_spi2 151: clk_top_spi3 152: clk_top_can0 153: clk_top_can1 154: clk_top_can2 155: clk_top_can3 156: clk_top_ptpc 157: clk_top_ana0 158: clk_top_ana1 159: clk_top_ana2 160: clk_top_ana3 161: clk_top_ana4 162: clk_top_ref0 163: clk_top_ref1 164: clk_top_lin0 165: clk_top_lin1 166: clk_top_lin2 167: clk_top_lin3"]
    #[inline(always)]
    #[must_use]
    pub fn selection(&mut self) -> SELECTION_W<CONTROL_SPEC> {
        SELECTION_W::new(self, 0)
    }
    #[doc = "Bit 8 - refrence clock selection, 0: 32k 1: 24M"]
    #[inline(always)]
    #[must_use]
    pub fn reference(&mut self) -> REFERENCE_W<CONTROL_SPEC> {
        REFERENCE_W::new(self, 8)
    }
    #[doc = "Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> ACCURACY_W<CONTROL_SPEC> {
        ACCURACY_W::new(self, 9)
    }
    #[doc = "Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CONTROL_SPEC> {
        MODE_W::new(self, 10)
    }
    #[doc = "Bit 12 - start measurement"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CONTROL_SPEC> {
        START_W::new(self, 12)
    }
    #[doc = "Bit 14 - clock frequency lower than lower limit"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<CONTROL_SPEC> {
        LOW_W::new(self, 14)
    }
    #[doc = "Bit 15 - clock frequency higher than upper limit"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<CONTROL_SPEC> {
        HIGH_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - output divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CONTROL_SPEC> {
        DIV_W::new(self, 16)
    }
    #[doc = "Bit 24 - enable clock output"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<CONTROL_SPEC> {
        OUTEN_W::new(self, 24)
    }
    #[doc = "Bit 31 - result is ready for read 0: not ready 1: result is ready"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<CONTROL_SPEC> {
        VALID_W::new(self, 31)
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
#[doc = "Clock measure and monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
