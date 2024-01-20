#[doc = "Register `ECC[%s]` reader"]
pub type R = crate::R<ECC_SPEC>;
#[doc = "Register `ECC[%s]` writer"]
pub type W = crate::W<ECC_SPEC>;
#[doc = "Field `ECC` reader - Parity check bits for key0"]
pub type ECC_R = crate::FieldReader<u16>;
#[doc = "Field `ECC` writer - Parity check bits for key0"]
pub type ECC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RLOCK` reader - read lock to key0 0: key read enable 1: key always read as 0"]
pub type RLOCK_R = crate::BitReader;
#[doc = "Field `RLOCK` writer - read lock to key0 0: key read enable 1: key always read as 0"]
pub type RLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLOCK` reader - write lock to key0 0: write enable 1: write ignored"]
pub type WLOCK_R = crate::BitReader;
#[doc = "Field `WLOCK` writer - write lock to key0 0: write enable 1: write ignored"]
pub type WLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Parity check bits for key0"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - read lock to key0 0: key read enable 1: key always read as 0"]
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write lock to key0 0: write enable 1: write ignored"]
    #[inline(always)]
    pub fn wlock(&self) -> WLOCK_R {
        WLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Parity check bits for key0"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<ECC_SPEC> {
        ECC_W::new(self, 0)
    }
    #[doc = "Bit 30 - read lock to key0 0: key read enable 1: key always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn rlock(&mut self) -> RLOCK_W<ECC_SPEC> {
        RLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - write lock to key0 0: write enable 1: write ignored"]
    #[inline(always)]
    #[must_use]
    pub fn wlock(&mut self) -> WLOCK_W<ECC_SPEC> {
        WLOCK_W::new(self, 31)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_SPEC;
impl crate::RegisterSpec for ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc::R`](R) reader structure"]
impl crate::Readable for ECC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc::W`](W) writer structure"]
impl crate::Writable for ECC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC[%s]
to value 0"]
impl crate::Resettable for ECC_SPEC {
    const RESET_VALUE: u32 = 0;
}
