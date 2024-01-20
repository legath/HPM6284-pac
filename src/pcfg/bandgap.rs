#[doc = "Register `BANDGAP` reader"]
pub type R = crate::R<BANDGAP_SPEC>;
#[doc = "Register `BANDGAP` writer"]
pub type W = crate::W<BANDGAP_SPEC>;
#[doc = "Field `VBG_P50_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_P50_TRIM_R = crate::FieldReader;
#[doc = "Field `VBG_P50_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_P50_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_P65_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_P65_TRIM_R = crate::FieldReader;
#[doc = "Field `VBG_P65_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_P65_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_1P0_TRIM` reader - Banggap 1.0V output trim value"]
pub type VBG_1P0_TRIM_R = crate::FieldReader;
#[doc = "Field `VBG_1P0_TRIM` writer - Banggap 1.0V output trim value"]
pub type VBG_1P0_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `POWER_SAVE` reader - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
pub type POWER_SAVE_R = crate::BitReader;
#[doc = "Field `POWER_SAVE` writer - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
pub type POWER_SAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWPOWER_MODE` reader - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
pub type LOWPOWER_MODE_R = crate::BitReader;
#[doc = "Field `LOWPOWER_MODE` writer - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
pub type LOWPOWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBG_TRIMMED` reader - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_R = crate::BitReader;
#[doc = "Field `VBG_TRIMMED` writer - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
pub type VBG_TRIMMED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p50_trim(&self) -> VBG_P50_TRIM_R {
        VBG_P50_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_p65_trim(&self) -> VBG_P65_TRIM_R {
        VBG_P65_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Banggap 1.0V output trim value"]
    #[inline(always)]
    pub fn vbg_1p0_trim(&self) -> VBG_1P0_TRIM_R {
        VBG_1P0_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
    #[inline(always)]
    pub fn power_save(&self) -> POWER_SAVE_R {
        POWER_SAVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
    #[inline(always)]
    pub fn lowpower_mode(&self) -> LOWPOWER_MODE_R {
        LOWPOWER_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    pub fn vbg_trimmed(&self) -> VBG_TRIMMED_R {
        VBG_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Banggap 1.0V output trim value"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_p50_trim(&mut self) -> VBG_P50_TRIM_W<BANDGAP_SPEC> {
        VBG_P50_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Banggap 1.0V output trim value"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_p65_trim(&mut self) -> VBG_P65_TRIM_W<BANDGAP_SPEC> {
        VBG_P65_TRIM_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Banggap 1.0V output trim value"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_1p0_trim(&mut self) -> VBG_1P0_TRIM_W<BANDGAP_SPEC> {
        VBG_1P0_TRIM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
    #[inline(always)]
    #[must_use]
    pub fn power_save(&mut self) -> POWER_SAVE_W<BANDGAP_SPEC> {
        POWER_SAVE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lowpower_mode(&mut self) -> LOWPOWER_MODE_W<BANDGAP_SPEC> {
        LOWPOWER_MODE_W::new(self, 25)
    }
    #[doc = "Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trimmed(&mut self) -> VBG_TRIMMED_W<BANDGAP_SPEC> {
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
#[doc = "BANGGAP control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bandgap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bandgap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BANDGAP_SPEC;
impl crate::RegisterSpec for BANDGAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bandgap::R`](R) reader structure"]
impl crate::Readable for BANDGAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bandgap::W`](W) writer structure"]
impl crate::Writable for BANDGAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANDGAP to value 0x0010_1010"]
impl crate::Resettable for BANDGAP_SPEC {
    const RESET_VALUE: u32 = 0x0010_1010;
}
