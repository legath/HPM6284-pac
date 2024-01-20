#[doc = "Register `XTAL32K_CFG` reader"]
pub type R = crate::R<XTAL32K_CFG_SPEC>;
#[doc = "Register `XTAL32K_CFG` writer"]
pub type W = crate::W<XTAL32K_CFG_SPEC>;
#[doc = "Field `AMP` reader - crystal 32k amplifier"]
pub type AMP_R = crate::FieldReader;
#[doc = "Field `AMP` writer - crystal 32k amplifier"]
pub type AMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFG` reader - crystal 32k config"]
pub type CFG_R = crate::BitReader;
#[doc = "Field `CFG` writer - crystal 32k config"]
pub type CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMSEL` reader - crystal 32k gm selection"]
pub type GMSEL_R = crate::FieldReader;
#[doc = "Field `GMSEL` writer - crystal 32k gm selection"]
pub type GMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HYST_EN` reader - crystal 32k hysteres enable"]
pub type HYST_EN_R = crate::BitReader;
#[doc = "Field `HYST_EN` writer - crystal 32k hysteres enable"]
pub type HYST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - crystal 32k amplifier"]
    #[inline(always)]
    pub fn amp(&self) -> AMP_R {
        AMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - crystal 32k config"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - crystal 32k gm selection"]
    #[inline(always)]
    pub fn gmsel(&self) -> GMSEL_R {
        GMSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - crystal 32k hysteres enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - crystal 32k amplifier"]
    #[inline(always)]
    #[must_use]
    pub fn amp(&mut self) -> AMP_W<XTAL32K_CFG_SPEC> {
        AMP_W::new(self, 0)
    }
    #[doc = "Bit 4 - crystal 32k config"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<XTAL32K_CFG_SPEC> {
        CFG_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - crystal 32k gm selection"]
    #[inline(always)]
    #[must_use]
    pub fn gmsel(&mut self) -> GMSEL_W<XTAL32K_CFG_SPEC> {
        GMSEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - crystal 32k hysteres enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HYST_EN_W<XTAL32K_CFG_SPEC> {
        HYST_EN_W::new(self, 12)
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
#[doc = "XTAL 32K config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL32K_CFG_SPEC;
impl crate::RegisterSpec for XTAL32K_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k_cfg::R`](R) reader structure"]
impl crate::Readable for XTAL32K_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal32k_cfg::W`](W) writer structure"]
impl crate::Writable for XTAL32K_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL32K_CFG to value 0"]
impl crate::Resettable for XTAL32K_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
