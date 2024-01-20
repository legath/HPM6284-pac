#[doc = "Register `DCDC_CURRENT` reader"]
pub type R = crate::R<DCDC_CURRENT_SPEC>;
#[doc = "Register `DCDC_CURRENT` writer"]
pub type W = crate::W<DCDC_CURRENT_SPEC>;
#[doc = "Field `LEVEL` reader - DCDC current level, current level is num * 50mA"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `VALID` reader - Current level valid 0: data is invalid 1: data is valid"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `ESTI_EN` reader - enable current measure"]
pub type ESTI_EN_R = crate::BitReader;
#[doc = "Field `ESTI_EN` writer - enable current measure"]
pub type ESTI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - DCDC current level, current level is num * 50mA"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Current level valid 0: data is invalid 1: data is valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - enable current measure"]
    #[inline(always)]
    pub fn esti_en(&self) -> ESTI_EN_R {
        ESTI_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - enable current measure"]
    #[inline(always)]
    #[must_use]
    pub fn esti_en(&mut self) -> ESTI_EN_W<DCDC_CURRENT_SPEC> {
        ESTI_EN_W::new(self, 15)
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
#[doc = "DCDC current estimation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_current::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_current::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_CURRENT_SPEC;
impl crate::RegisterSpec for DCDC_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_current::R`](R) reader structure"]
impl crate::Readable for DCDC_CURRENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_current::W`](W) writer structure"]
impl crate::Writable for DCDC_CURRENT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_CURRENT to value 0"]
impl crate::Resettable for DCDC_CURRENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
