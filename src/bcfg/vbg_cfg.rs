#[doc = "Register `VBG_CFG` reader"]
pub type R = crate::R<VBG_CFG_SPEC>;
#[doc = "Register `VBG_CFG` writer"]
pub type W = crate::W<VBG_CFG_SPEC>;
#[doc = "Field `VBG_P50` reader - Bandgap 0.50V output trim"]
pub type VBG_P50_R = crate::FieldReader;
#[doc = "Field `VBG_P50` writer - Bandgap 0.50V output trim"]
pub type VBG_P50_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_P65` reader - Bandgap 0.65V output trim"]
pub type VBG_P65_R = crate::FieldReader;
#[doc = "Field `VBG_P65` writer - Bandgap 0.65V output trim"]
pub type VBG_P65_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_1P0` reader - Bandgap 1.0V output trim"]
pub type VBG_1P0_R = crate::FieldReader;
#[doc = "Field `VBG_1P0` writer - Bandgap 1.0V output trim"]
pub type VBG_1P0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `POWER_SAVE` reader - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
pub type POWER_SAVE_R = crate::BitReader;
#[doc = "Field `POWER_SAVE` writer - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
pub type POWER_SAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MODE` reader - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
pub type LP_MODE_R = crate::BitReader;
#[doc = "Field `LP_MODE` writer - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
pub type LP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBG_TRIMMED` reader - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_R = crate::BitReader;
#[doc = "Field `VBG_TRIMMED` writer - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Bandgap 0.50V output trim"]
    #[inline(always)]
    pub fn vbg_p50(&self) -> VBG_P50_R {
        VBG_P50_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Bandgap 0.65V output trim"]
    #[inline(always)]
    pub fn vbg_p65(&self) -> VBG_P65_R {
        VBG_P65_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Bandgap 1.0V output trim"]
    #[inline(always)]
    pub fn vbg_1p0(&self) -> VBG_1P0_R {
        VBG_1P0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
    #[inline(always)]
    pub fn power_save(&self) -> POWER_SAVE_R {
        POWER_SAVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&self) -> VBG_TRIMMED_R {
        VBG_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bandgap 0.50V output trim"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_p50(&mut self) -> VBG_P50_W<VBG_CFG_SPEC> {
        VBG_P50_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Bandgap 0.65V output trim"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_p65(&mut self) -> VBG_P65_W<VBG_CFG_SPEC> {
        VBG_P65_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Bandgap 1.0V output trim"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_1p0(&mut self) -> VBG_1P0_W<VBG_CFG_SPEC> {
        VBG_1P0_W::new(self, 16)
    }
    #[doc = "Bit 24 - Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
    #[inline(always)]
    #[must_use]
    pub fn power_save(&mut self) -> POWER_SAVE_W<VBG_CFG_SPEC> {
        POWER_SAVE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode(&mut self) -> LP_MODE_W<VBG_CFG_SPEC> {
        LP_MODE_W::new(self, 25)
    }
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trimmed(&mut self) -> VBG_TRIMMED_W<VBG_CFG_SPEC> {
        VBG_TRIMMED_W::new(self, 31)
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
#[doc = "Bandgap config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbg_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbg_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBG_CFG_SPEC;
impl crate::RegisterSpec for VBG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbg_cfg::R`](R) reader structure"]
impl crate::Readable for VBG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbg_cfg::W`](W) writer structure"]
impl crate::Writable for VBG_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBG_CFG to value 0"]
impl crate::Resettable for VBG_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
