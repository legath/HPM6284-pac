#[doc = "Register `PHY_CTRL1` reader"]
pub type R = crate::R<PHY_CTRL1_SPEC>;
#[doc = "Register `PHY_CTRL1` writer"]
pub type W = crate::W<PHY_CTRL1_SPEC>;
#[doc = "Field `UTMI_OTG_SUSPENDM` reader - OTG suspend, not utmi_suspendm"]
pub type UTMI_OTG_SUSPENDM_R = crate::BitReader;
#[doc = "Field `UTMI_OTG_SUSPENDM` writer - OTG suspend, not utmi_suspendm"]
pub type UTMI_OTG_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_CFG_RST_N` reader - No description avaiable"]
pub type UTMI_CFG_RST_N_R = crate::BitReader;
#[doc = "Field `UTMI_CFG_RST_N` writer - No description avaiable"]
pub type UTMI_CFG_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - OTG suspend, not utmi_suspendm"]
    #[inline(always)]
    pub fn utmi_otg_suspendm(&self) -> UTMI_OTG_SUSPENDM_R {
        UTMI_OTG_SUSPENDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 20 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_cfg_rst_n(&self) -> UTMI_CFG_RST_N_R {
        UTMI_CFG_RST_N_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OTG suspend, not utmi_suspendm"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_otg_suspendm(&mut self) -> UTMI_OTG_SUSPENDM_W<PHY_CTRL1_SPEC> {
        UTMI_OTG_SUSPENDM_W::new(self, 1)
    }
    #[doc = "Bit 20 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_cfg_rst_n(&mut self) -> UTMI_CFG_RST_N_W<PHY_CTRL1_SPEC> {
        UTMI_CFG_RST_N_W::new(self, 20)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_CTRL1_SPEC;
impl crate::RegisterSpec for PHY_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_ctrl1::R`](R) reader structure"]
impl crate::Readable for PHY_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_ctrl1::W`](W) writer structure"]
impl crate::Writable for PHY_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_CTRL1 to value 0"]
impl crate::Resettable for PHY_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
