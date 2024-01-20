#[doc = "Register `MONOL` reader"]
pub type R = crate::R<MONOL_SPEC>;
#[doc = "Register `MONOL` writer"]
pub type W = crate::W<MONOL_SPEC>;
#[doc = "Field `COUNTER` reader - low part of monotonica counter, write to this counter will cause counter increase by 1"]
pub type COUNTER_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER` writer - low part of monotonica counter, write to this counter will cause counter increase by 1"]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low part of monotonica counter, write to this counter will cause counter increase by 1"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low part of monotonica counter, write to this counter will cause counter increase by 1"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<MONOL_SPEC> {
        COUNTER_W::new(self, 0)
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
#[doc = "Low part of monotonic counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MONOL_SPEC;
impl crate::RegisterSpec for MONOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monol::R`](R) reader structure"]
impl crate::Readable for MONOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`monol::W`](W) writer structure"]
impl crate::Writable for MONOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MONOL to value 0"]
impl crate::Resettable for MONOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
