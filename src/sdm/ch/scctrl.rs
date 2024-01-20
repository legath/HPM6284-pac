#[doc = "Register `SCCTRL` reader"]
pub type R = crate::R<SCCTRL_SPEC>;
#[doc = "Register `SCCTRL` writer"]
pub type W = crate::W<SCCTRL_SPEC>;
#[doc = "Field `EN` reader - Amplitude Path Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Amplitude Path Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGN_INI_SAMPLES` reader - NotZero: Ignore the first samples that are not accurate Zero: Use all samples"]
pub type IGN_INI_SAMPLES_R = crate::FieldReader;
#[doc = "Field `IGN_INI_SAMPLES` writer - NotZero: Ignore the first samples that are not accurate Zero: Use all samples"]
pub type IGN_INI_SAMPLES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CIC_DEC_RATIO` reader - CIC decimation ratio. 0 means div-by-32"]
pub type CIC_DEC_RATIO_R = crate::FieldReader;
#[doc = "Field `CIC_DEC_RATIO` writer - CIC decimation ratio. 0 means div-by-32"]
pub type CIC_DEC_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SGD_ORDR` reader - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
pub type SGD_ORDR_R = crate::FieldReader;
#[doc = "Field `SGD_ORDR` writer - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
pub type SGD_ORDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL_IE` reader - LLT interrupt Enable"]
pub type LL_IE_R = crate::BitReader;
#[doc = "Field `LL_IE` writer - LLT interrupt Enable"]
pub type LL_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HL_IE` reader - HLT Interrupt Enable"]
pub type HL_IE_R = crate::BitReader;
#[doc = "Field `HL_IE` writer - HLT Interrupt Enable"]
pub type HL_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MF_IE` reader - Module failure Interrupt enable"]
pub type MF_IE_R = crate::BitReader;
#[doc = "Field `MF_IE` writer - Module failure Interrupt enable"]
pub type MF_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HZ_EN` reader - Zero Crossing Enable"]
pub type HZ_EN_R = crate::BitReader;
#[doc = "Field `HZ_EN` writer - Zero Crossing Enable"]
pub type HZ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Amplitude Path Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - NotZero: Ignore the first samples that are not accurate Zero: Use all samples"]
    #[inline(always)]
    pub fn ign_ini_samples(&self) -> IGN_INI_SAMPLES_R {
        IGN_INI_SAMPLES_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:8 - CIC decimation ratio. 0 means div-by-32"]
    #[inline(always)]
    pub fn cic_dec_ratio(&self) -> CIC_DEC_RATIO_R {
        CIC_DEC_RATIO_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 18:19 - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
    #[inline(always)]
    pub fn sgd_ordr(&self) -> SGD_ORDR_R {
        SGD_ORDR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - LLT interrupt Enable"]
    #[inline(always)]
    pub fn ll_ie(&self) -> LL_IE_R {
        LL_IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HLT Interrupt Enable"]
    #[inline(always)]
    pub fn hl_ie(&self) -> HL_IE_R {
        HL_IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Module failure Interrupt enable"]
    #[inline(always)]
    pub fn mf_ie(&self) -> MF_IE_R {
        MF_IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Zero Crossing Enable"]
    #[inline(always)]
    pub fn hz_en(&self) -> HZ_EN_R {
        HZ_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Amplitude Path Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<SCCTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - NotZero: Ignore the first samples that are not accurate Zero: Use all samples"]
    #[inline(always)]
    #[must_use]
    pub fn ign_ini_samples(&mut self) -> IGN_INI_SAMPLES_W<SCCTRL_SPEC> {
        IGN_INI_SAMPLES_W::new(self, 1)
    }
    #[doc = "Bits 4:8 - CIC decimation ratio. 0 means div-by-32"]
    #[inline(always)]
    #[must_use]
    pub fn cic_dec_ratio(&mut self) -> CIC_DEC_RATIO_W<SCCTRL_SPEC> {
        CIC_DEC_RATIO_W::new(self, 4)
    }
    #[doc = "Bits 18:19 - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
    #[inline(always)]
    #[must_use]
    pub fn sgd_ordr(&mut self) -> SGD_ORDR_W<SCCTRL_SPEC> {
        SGD_ORDR_W::new(self, 18)
    }
    #[doc = "Bit 20 - LLT interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ll_ie(&mut self) -> LL_IE_W<SCCTRL_SPEC> {
        LL_IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - HLT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hl_ie(&mut self) -> HL_IE_W<SCCTRL_SPEC> {
        HL_IE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Module failure Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mf_ie(&mut self) -> MF_IE_W<SCCTRL_SPEC> {
        MF_IE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Zero Crossing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hz_en(&mut self) -> HZ_EN_W<SCCTRL_SPEC> {
        HZ_EN_W::new(self, 23)
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
#[doc = "Amplitude Path Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCTRL_SPEC;
impl crate::RegisterSpec for SCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scctrl::R`](R) reader structure"]
impl crate::Readable for SCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scctrl::W`](W) writer structure"]
impl crate::Writable for SCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCCTRL to value 0"]
impl crate::Resettable for SCCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
