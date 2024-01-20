#[doc = "Register `MONOH` reader"]
pub type R = crate::R<MONOH_SPEC>;
#[doc = "Register `MONOH` writer"]
pub type W = crate::W<MONOH_SPEC>;
#[doc = "Field `COUNTER` reader - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
pub type COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `COUNTER` writer - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EPOCH` reader - Fuse value for high part of monotonica"]
pub type EPOCH_R = crate::FieldReader<u16>;
#[doc = "Field `EPOCH` writer - Fuse value for high part of monotonica"]
pub type EPOCH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Fuse value for high part of monotonica"]
    #[inline(always)]
    pub fn epoch(&self) -> EPOCH_R {
        EPOCH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<MONOH_SPEC> {
        COUNTER_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Fuse value for high part of monotonica"]
    #[inline(always)]
    #[must_use]
    pub fn epoch(&mut self) -> EPOCH_W<MONOH_SPEC> {
        EPOCH_W::new(self, 16)
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
#[doc = "High part of monotonic counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monoh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monoh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MONOH_SPEC;
impl crate::RegisterSpec for MONOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monoh::R`](R) reader structure"]
impl crate::Readable for MONOH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`monoh::W`](W) writer structure"]
impl crate::Writable for MONOH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MONOH to value 0"]
impl crate::Resettable for MONOH_SPEC {
    const RESET_VALUE: u32 = 0;
}
