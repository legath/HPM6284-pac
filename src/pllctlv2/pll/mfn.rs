#[doc = "Register `MFN` reader"]
pub type R = crate::R<MFN_SPEC>;
#[doc = "Register `MFN` writer"]
pub type W = crate::W<MFN_SPEC>;
#[doc = "Field `MFN` reader - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
pub type MFN_R = crate::FieldReader<u32>;
#[doc = "Field `MFN` writer - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
pub type MFN_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
    #[inline(always)]
    #[must_use]
    pub fn mfn(&mut self) -> MFN_W<MFN_SPEC> {
        MFN_W::new(self, 0)
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
#[doc = "PLL0 fraction numerator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFN_SPEC;
impl crate::RegisterSpec for MFN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfn::R`](R) reader structure"]
impl crate::Readable for MFN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mfn::W`](W) writer structure"]
impl crate::Writable for MFN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MFN to value 0x0989_6800"]
impl crate::Resettable for MFN_SPEC {
    const RESET_VALUE: u32 = 0x0989_6800;
}
