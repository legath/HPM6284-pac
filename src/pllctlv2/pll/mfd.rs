#[doc = "Register `MFD` reader"]
pub type R = crate::R<MFD_SPEC>;
#[doc = "Register `MFD` writer"]
pub type W = crate::W<MFD_SPEC>;
#[doc = "Field `MFD` reader - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
pub type MFD_R = crate::FieldReader<u32>;
#[doc = "Field `MFD` writer - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
pub type MFD_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
    #[inline(always)]
    pub fn mfd(&self) -> MFD_R {
        MFD_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
    #[inline(always)]
    #[must_use]
    pub fn mfd(&mut self) -> MFD_W<MFD_SPEC> {
        MFD_W::new(self, 0)
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
#[doc = "PLL0 fraction demoninator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFD_SPEC;
impl crate::RegisterSpec for MFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfd::R`](R) reader structure"]
impl crate::Readable for MFD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mfd::W`](W) writer structure"]
impl crate::Writable for MFD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MFD to value 0x0e4e_1c00"]
impl crate::Resettable for MFD_SPEC {
    const RESET_VALUE: u32 = 0x0e4e_1c00;
}
