#[doc = "Register `SDCTRLE` reader"]
pub type R = crate::R<SDCTRLE_SPEC>;
#[doc = "Register `SDCTRLE` writer"]
pub type W = crate::W<SDCTRLE_SPEC>;
#[doc = "Field `IGN_INI_SAMPLES` reader - NotZero: Don't store the first samples that are not accurate Zero: Store all samples"]
pub type IGN_INI_SAMPLES_R = crate::FieldReader;
#[doc = "Field `IGN_INI_SAMPLES` writer - NotZero: Don't store the first samples that are not accurate Zero: Store all samples"]
pub type IGN_INI_SAMPLES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CIC_DEC_RATIO` reader - CIC decimation ratio. 0 means div-by-256"]
pub type CIC_DEC_RATIO_R = crate::FieldReader;
#[doc = "Field `CIC_DEC_RATIO` writer - CIC decimation ratio. 0 means div-by-256"]
pub type CIC_DEC_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CIC_SCL` reader - CIC shift control"]
pub type CIC_SCL_R = crate::FieldReader;
#[doc = "Field `CIC_SCL` writer - CIC shift control"]
pub type CIC_SCL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWMSYNC` reader - Asserted to double sync the PWM trigger signal"]
pub type PWMSYNC_R = crate::BitReader;
#[doc = "Field `PWMSYNC` writer - Asserted to double sync the PWM trigger signal"]
pub type PWMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGD_ORDR` reader - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
pub type SGD_ORDR_R = crate::FieldReader;
#[doc = "Field `SGD_ORDR` writer - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
pub type SGD_ORDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - NotZero: Don't store the first samples that are not accurate Zero: Store all samples"]
    #[inline(always)]
    pub fn ign_ini_samples(&self) -> IGN_INI_SAMPLES_R {
        IGN_INI_SAMPLES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:10 - CIC decimation ratio. 0 means div-by-256"]
    #[inline(always)]
    pub fn cic_dec_ratio(&self) -> CIC_DEC_RATIO_R {
        CIC_DEC_RATIO_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 11:14 - CIC shift control"]
    #[inline(always)]
    pub fn cic_scl(&self) -> CIC_SCL_R {
        CIC_SCL_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Asserted to double sync the PWM trigger signal"]
    #[inline(always)]
    pub fn pwmsync(&self) -> PWMSYNC_R {
        PWMSYNC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
    #[inline(always)]
    pub fn sgd_ordr(&self) -> SGD_ORDR_R {
        SGD_ORDR_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - NotZero: Don't store the first samples that are not accurate Zero: Store all samples"]
    #[inline(always)]
    #[must_use]
    pub fn ign_ini_samples(&mut self) -> IGN_INI_SAMPLES_W<SDCTRLE_SPEC> {
        IGN_INI_SAMPLES_W::new(self, 0)
    }
    #[doc = "Bits 3:10 - CIC decimation ratio. 0 means div-by-256"]
    #[inline(always)]
    #[must_use]
    pub fn cic_dec_ratio(&mut self) -> CIC_DEC_RATIO_W<SDCTRLE_SPEC> {
        CIC_DEC_RATIO_W::new(self, 3)
    }
    #[doc = "Bits 11:14 - CIC shift control"]
    #[inline(always)]
    #[must_use]
    pub fn cic_scl(&mut self) -> CIC_SCL_W<SDCTRLE_SPEC> {
        CIC_SCL_W::new(self, 11)
    }
    #[doc = "Bit 16 - Asserted to double sync the PWM trigger signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwmsync(&mut self) -> PWMSYNC_W<SDCTRLE_SPEC> {
        PWMSYNC_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC"]
    #[inline(always)]
    #[must_use]
    pub fn sgd_ordr(&mut self) -> SGD_ORDR_W<SDCTRLE_SPEC> {
        SGD_ORDR_W::new(self, 17)
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
#[doc = "Data Path Control Extra Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdctrle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdctrle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCTRLE_SPEC;
impl crate::RegisterSpec for SDCTRLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdctrle::R`](R) reader structure"]
impl crate::Readable for SDCTRLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdctrle::W`](W) writer structure"]
impl crate::Writable for SDCTRLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCTRLE to value 0"]
impl crate::Resettable for SDCTRLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
