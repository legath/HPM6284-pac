#[doc = "Register `u` reader"]
pub type R = crate::R<U_SPEC>;
#[doc = "Register `u` writer"]
pub type W = crate::W<U_SPEC>;
#[doc = "Field `UCNT` reader - ucnt counter"]
pub type UCNT_R = crate::FieldReader<u32>;
#[doc = "Field `WSTAT` reader - this bit indicate W state"]
pub type WSTAT_R = crate::BitReader;
#[doc = "Field `VSTAT` reader - this bit indicate V state"]
pub type VSTAT_R = crate::BitReader;
#[doc = "Field `USTAT` reader - this bit indicate U state"]
pub type USTAT_R = crate::BitReader;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:27 - ucnt counter"]
    #[inline(always)]
    pub fn ucnt(&self) -> UCNT_R {
        UCNT_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - this bit indicate W state"]
    #[inline(always)]
    pub fn wstat(&self) -> WSTAT_R {
        WSTAT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - this bit indicate V state"]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - this bit indicate U state"]
    #[inline(always)]
    pub fn ustat(&self) -> USTAT_R {
        USTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
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
#[doc = "U counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_SPEC;
impl crate::RegisterSpec for U_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u::R`](R) reader structure"]
impl crate::Readable for U_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u::W`](W) writer structure"]
impl crate::Writable for U_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets u to value 0"]
impl crate::Resettable for U_SPEC {
    const RESET_VALUE: u32 = 0;
}
