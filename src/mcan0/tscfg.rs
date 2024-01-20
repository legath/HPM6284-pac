#[doc = "Register `TSCFG` reader"]
pub type R = crate::R<TSCFG_SPEC>;
#[doc = "Register `TSCFG` writer"]
pub type W = crate::W<TSCFG_SPEC>;
#[doc = "Field `TSUE` reader - Timestamp Unit Enable 0: TSU disabled 1: TSU enabled"]
pub type TSUE_R = crate::BitReader;
#[doc = "Field `TSUE` writer - Timestamp Unit Enable 0: TSU disabled 1: TSU enabled"]
pub type TSUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCS` reader - Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]"]
pub type TBCS_R = crate::BitReader;
#[doc = "Field `TBCS` writer - Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]"]
pub type TBCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCP` reader - Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF"]
pub type SCP_R = crate::BitReader;
#[doc = "Field `SCP` writer - Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF"]
pub type SCP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN64` reader - set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one"]
pub type EN64_R = crate::BitReader;
#[doc = "Field `EN64` writer - set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one"]
pub type EN64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPRE` reader - Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
pub type TBPRE_R = crate::FieldReader;
#[doc = "Field `TBPRE` writer - Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
pub type TBPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Timestamp Unit Enable 0: TSU disabled 1: TSU enabled"]
    #[inline(always)]
    pub fn tsue(&self) -> TSUE_R {
        TSUE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]"]
    #[inline(always)]
    pub fn tbcs(&self) -> TBCS_R {
        TBCS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF"]
    #[inline(always)]
    pub fn scp(&self) -> SCP_R {
        SCP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one"]
    #[inline(always)]
    pub fn en64(&self) -> EN64_R {
        EN64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
    #[inline(always)]
    pub fn tbpre(&self) -> TBPRE_R {
        TBPRE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Unit Enable 0: TSU disabled 1: TSU enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tsue(&mut self) -> TSUE_W<TSCFG_SPEC> {
        TSUE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tbcs(&mut self) -> TBCS_W<TSCFG_SPEC> {
        TBCS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF"]
    #[inline(always)]
    #[must_use]
    pub fn scp(&mut self) -> SCP_W<TSCFG_SPEC> {
        SCP_W::new(self, 2)
    }
    #[doc = "Bit 3 - set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one"]
    #[inline(always)]
    #[must_use]
    pub fn en64(&mut self) -> EN64_W<TSCFG_SPEC> {
        EN64_W::new(self, 3)
    }
    #[doc = "Bits 8:15 - Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
    #[inline(always)]
    #[must_use]
    pub fn tbpre(&mut self) -> TBPRE_W<TSCFG_SPEC> {
        TBPRE_W::new(self, 8)
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
#[doc = "timestamp configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCFG_SPEC;
impl crate::RegisterSpec for TSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscfg::R`](R) reader structure"]
impl crate::Readable for TSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscfg::W`](W) writer structure"]
impl crate::Writable for TSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCFG to value 0"]
impl crate::Resettable for TSCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
