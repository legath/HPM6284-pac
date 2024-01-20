#[doc = "Register `TPM` reader"]
pub type R = crate::R<TPM_SPEC>;
#[doc = "Register `TPM` writer"]
pub type W = crate::W<TPM_SPEC>;
#[doc = "Field `TPM` reader - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
pub type TPM_R = crate::FieldReader;
#[doc = "Field `TPM` writer - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
pub type TPM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
    #[inline(always)]
    #[must_use]
    pub fn tpm(&mut self) -> TPM_W<TPM_SPEC> {
        TPM_W::new(self, 0)
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
#[doc = "I2C Timing Paramater Multiplier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPM_SPEC;
impl crate::RegisterSpec for TPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpm::R`](R) reader structure"]
impl crate::Readable for TPM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpm::W`](W) writer structure"]
impl crate::Writable for TPM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPM to value 0"]
impl crate::Resettable for TPM_SPEC {
    const RESET_VALUE: u32 = 0;
}
