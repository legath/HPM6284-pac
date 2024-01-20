#[doc = "Register `SAMPLE_CFG[%s]` reader"]
pub type R = crate::R<SAMPLE_CFG_SPEC>;
#[doc = "Register `SAMPLE_CFG[%s]` writer"]
pub type W = crate::W<SAMPLE_CFG_SPEC>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER` reader - sample clock number, base on clock_period, default one period"]
pub type SAMPLE_CLOCK_NUMBER_R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER` writer - sample clock number, base on clock_period, default one period"]
pub type SAMPLE_CLOCK_NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER_SHIFT` reader - shift for sample clock number"]
pub type SAMPLE_CLOCK_NUMBER_SHIFT_R = crate::FieldReader;
#[doc = "Field `SAMPLE_CLOCK_NUMBER_SHIFT` writer - shift for sample clock number"]
pub type SAMPLE_CLOCK_NUMBER_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:8 - sample clock number, base on clock_period, default one period"]
    #[inline(always)]
    pub fn sample_clock_number(&self) -> SAMPLE_CLOCK_NUMBER_R {
        SAMPLE_CLOCK_NUMBER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - shift for sample clock number"]
    #[inline(always)]
    pub fn sample_clock_number_shift(&self) -> SAMPLE_CLOCK_NUMBER_SHIFT_R {
        SAMPLE_CLOCK_NUMBER_SHIFT_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - sample clock number, base on clock_period, default one period"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_number(&mut self) -> SAMPLE_CLOCK_NUMBER_W<SAMPLE_CFG_SPEC> {
        SAMPLE_CLOCK_NUMBER_W::new(self, 0)
    }
    #[doc = "Bits 9:11 - shift for sample clock number"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_number_shift(&mut self) -> SAMPLE_CLOCK_NUMBER_SHIFT_W<SAMPLE_CFG_SPEC> {
        SAMPLE_CLOCK_NUMBER_SHIFT_W::new(self, 9)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_CFG_SPEC;
impl crate::RegisterSpec for SAMPLE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_cfg::R`](R) reader structure"]
impl crate::Readable for SAMPLE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_cfg::W`](W) writer structure"]
impl crate::Writable for SAMPLE_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLE_CFG[%s]
to value 0"]
impl crate::Resettable for SAMPLE_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
