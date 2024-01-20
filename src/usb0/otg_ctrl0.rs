#[doc = "Register `OTG_CTRL0` reader"]
pub type R = crate::R<OTG_CTRL0_SPEC>;
#[doc = "Register `OTG_CTRL0` writer"]
pub type W = crate::W<OTG_CTRL0_SPEC>;
#[doc = "Field `SER_MODE_SUSPEND_EN` reader - for naneng usbphy, only switch to serial mode when suspend"]
pub type SER_MODE_SUSPEND_EN_R = crate::BitReader;
#[doc = "Field `SER_MODE_SUSPEND_EN` writer - for naneng usbphy, only switch to serial mode when suspend"]
pub type SER_MODE_SUSPEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_OVER_CUR_DIS` reader - No description avaiable"]
pub type OTG_OVER_CUR_DIS_R = crate::BitReader;
#[doc = "Field `OTG_OVER_CUR_DIS` writer - No description avaiable"]
pub type OTG_OVER_CUR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_OVER_CUR_POL` reader - No description avaiable"]
pub type OTG_OVER_CUR_POL_R = crate::BitReader;
#[doc = "Field `OTG_OVER_CUR_POL` writer - No description avaiable"]
pub type OTG_OVER_CUR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_POWER_MASK` reader - No description avaiable"]
pub type OTG_POWER_MASK_R = crate::BitReader;
#[doc = "Field `OTG_POWER_MASK` writer - No description avaiable"]
pub type OTG_POWER_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_WAKEUP_INT_ENABLE` reader - No description avaiable"]
pub type OTG_WAKEUP_INT_ENABLE_R = crate::BitReader;
#[doc = "Field `OTG_WAKEUP_INT_ENABLE` writer - No description avaiable"]
pub type OTG_WAKEUP_INT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_UTMI_RESET_SW` reader - default 1 for naneng usbphy"]
pub type OTG_UTMI_RESET_SW_R = crate::BitReader;
#[doc = "Field `OTG_UTMI_RESET_SW` writer - default 1 for naneng usbphy"]
pub type OTG_UTMI_RESET_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_UTMI_SUSPENDM_SW` reader - default 0 for naneng usbphy"]
pub type OTG_UTMI_SUSPENDM_SW_R = crate::BitReader;
#[doc = "Field `OTG_UTMI_SUSPENDM_SW` writer - default 0 for naneng usbphy"]
pub type OTG_UTMI_SUSPENDM_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_VBUS_SOURCE_SEL` reader - No description avaiable"]
pub type OTG_VBUS_SOURCE_SEL_R = crate::BitReader;
#[doc = "Field `OTG_VBUS_SOURCE_SEL` writer - No description avaiable"]
pub type OTG_VBUS_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_ID_WAKEUP_EN` reader - No description avaiable"]
pub type OTG_ID_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `OTG_ID_WAKEUP_EN` writer - No description avaiable"]
pub type OTG_ID_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_VBUS_WAKEUP_EN` reader - No description avaiable"]
pub type OTG_VBUS_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `OTG_VBUS_WAKEUP_EN` writer - No description avaiable"]
pub type OTG_VBUS_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORESUME_EN` reader - No description avaiable"]
pub type AUTORESUME_EN_R = crate::BitReader;
#[doc = "Field `AUTORESUME_EN` writer - No description avaiable"]
pub type AUTORESUME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_WKDPDMCHG_EN` reader - No description avaiable"]
pub type OTG_WKDPDMCHG_EN_R = crate::BitReader;
#[doc = "Field `OTG_WKDPDMCHG_EN` writer - No description avaiable"]
pub type OTG_WKDPDMCHG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - for naneng usbphy, only switch to serial mode when suspend"]
    #[inline(always)]
    pub fn ser_mode_suspend_en(&self) -> SER_MODE_SUSPEND_EN_R {
        SER_MODE_SUSPEND_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_dis(&self) -> OTG_OVER_CUR_DIS_R {
        OTG_OVER_CUR_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    pub fn otg_over_cur_pol(&self) -> OTG_OVER_CUR_POL_R {
        OTG_OVER_CUR_POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - No description avaiable"]
    #[inline(always)]
    pub fn otg_power_mask(&self) -> OTG_POWER_MASK_R {
        OTG_POWER_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wakeup_int_enable(&self) -> OTG_WAKEUP_INT_ENABLE_R {
        OTG_WAKEUP_INT_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - default 1 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_reset_sw(&self) -> OTG_UTMI_RESET_SW_R {
        OTG_UTMI_RESET_SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - default 0 for naneng usbphy"]
    #[inline(always)]
    pub fn otg_utmi_suspendm_sw(&self) -> OTG_UTMI_SUSPENDM_SW_R {
        OTG_UTMI_SUSPENDM_SW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_source_sel(&self) -> OTG_VBUS_SOURCE_SEL_R {
        OTG_VBUS_SOURCE_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    pub fn otg_id_wakeup_en(&self) -> OTG_ID_WAKEUP_EN_R {
        OTG_ID_WAKEUP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn otg_vbus_wakeup_en(&self) -> OTG_VBUS_WAKEUP_EN_R {
        OTG_VBUS_WAKEUP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn otg_wkdpdmchg_en(&self) -> OTG_WKDPDMCHG_EN_R {
        OTG_WKDPDMCHG_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - for naneng usbphy, only switch to serial mode when suspend"]
    #[inline(always)]
    #[must_use]
    pub fn ser_mode_suspend_en(&mut self) -> SER_MODE_SUSPEND_EN_W<OTG_CTRL0_SPEC> {
        SER_MODE_SUSPEND_EN_W::new(self, 4)
    }
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_over_cur_dis(&mut self) -> OTG_OVER_CUR_DIS_W<OTG_CTRL0_SPEC> {
        OTG_OVER_CUR_DIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_over_cur_pol(&mut self) -> OTG_OVER_CUR_POL_W<OTG_CTRL0_SPEC> {
        OTG_OVER_CUR_POL_W::new(self, 8)
    }
    #[doc = "Bit 9 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_power_mask(&mut self) -> OTG_POWER_MASK_W<OTG_CTRL0_SPEC> {
        OTG_POWER_MASK_W::new(self, 9)
    }
    #[doc = "Bit 10 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_wakeup_int_enable(&mut self) -> OTG_WAKEUP_INT_ENABLE_W<OTG_CTRL0_SPEC> {
        OTG_WAKEUP_INT_ENABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - default 1 for naneng usbphy"]
    #[inline(always)]
    #[must_use]
    pub fn otg_utmi_reset_sw(&mut self) -> OTG_UTMI_RESET_SW_W<OTG_CTRL0_SPEC> {
        OTG_UTMI_RESET_SW_W::new(self, 11)
    }
    #[doc = "Bit 12 - default 0 for naneng usbphy"]
    #[inline(always)]
    #[must_use]
    pub fn otg_utmi_suspendm_sw(&mut self) -> OTG_UTMI_SUSPENDM_SW_W<OTG_CTRL0_SPEC> {
        OTG_UTMI_SUSPENDM_SW_W::new(self, 12)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_vbus_source_sel(&mut self) -> OTG_VBUS_SOURCE_SEL_W<OTG_CTRL0_SPEC> {
        OTG_VBUS_SOURCE_SEL_W::new(self, 13)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_id_wakeup_en(&mut self) -> OTG_ID_WAKEUP_EN_W<OTG_CTRL0_SPEC> {
        OTG_ID_WAKEUP_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_vbus_wakeup_en(&mut self) -> OTG_VBUS_WAKEUP_EN_W<OTG_CTRL0_SPEC> {
        OTG_VBUS_WAKEUP_EN_W::new(self, 17)
    }
    #[doc = "Bit 19 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W<OTG_CTRL0_SPEC> {
        AUTORESUME_EN_W::new(self, 19)
    }
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn otg_wkdpdmchg_en(&mut self) -> OTG_WKDPDMCHG_EN_W<OTG_CTRL0_SPEC> {
        OTG_WKDPDMCHG_EN_W::new(self, 25)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_CTRL0_SPEC;
impl crate::RegisterSpec for OTG_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_ctrl0::R`](R) reader structure"]
impl crate::Readable for OTG_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_ctrl0::W`](W) writer structure"]
impl crate::Writable for OTG_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_CTRL0 to value 0"]
impl crate::Resettable for OTG_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
