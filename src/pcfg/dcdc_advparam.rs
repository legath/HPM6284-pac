#[doc = "Register `DCDC_ADVPARAM` reader"]
pub type R = crate::R<DCDC_ADVPARAM_SPEC>;
#[doc = "Register `DCDC_ADVPARAM` writer"]
pub type W = crate::W<DCDC_ADVPARAM_SPEC>;
#[doc = "Field `MAX_DUT` reader - maximum duty cycle"]
pub type MAX_DUT_R = crate::FieldReader;
#[doc = "Field `MAX_DUT` writer - maximum duty cycle"]
pub type MAX_DUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MIN_DUT` reader - minimum duty cycle"]
pub type MIN_DUT_R = crate::FieldReader;
#[doc = "Field `MIN_DUT` writer - minimum duty cycle"]
pub type MIN_DUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - maximum duty cycle"]
    #[inline(always)]
    pub fn max_dut(&self) -> MAX_DUT_R {
        MAX_DUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - minimum duty cycle"]
    #[inline(always)]
    pub fn min_dut(&self) -> MIN_DUT_R {
        MIN_DUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - maximum duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn max_dut(&mut self) -> MAX_DUT_W<DCDC_ADVPARAM_SPEC> {
        MAX_DUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - minimum duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn min_dut(&mut self) -> MIN_DUT_W<DCDC_ADVPARAM_SPEC> {
        MIN_DUT_W::new(self, 8)
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
#[doc = "DCDC advance parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_advparam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_advparam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_ADVPARAM_SPEC;
impl crate::RegisterSpec for DCDC_ADVPARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_advparam::R`](R) reader structure"]
impl crate::Readable for DCDC_ADVPARAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_advparam::W`](W) writer structure"]
impl crate::Writable for DCDC_ADVPARAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_ADVPARAM to value 0x6e1c"]
impl crate::Resettable for DCDC_ADVPARAM_SPEC {
    const RESET_VALUE: u32 = 0x6e1c;
}
