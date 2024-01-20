#[doc = "Register `SELECT` reader"]
pub type R = crate::R<SELECT_SPEC>;
#[doc = "Register `SELECT` writer"]
pub type W = crate::W<SELECT_SPEC>;
#[doc = "Field `SELECT` reader - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
pub type SELECT_R = crate::BitReader;
#[doc = "Field `SELECT` writer - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
pub type SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<SELECT_SPEC> {
        SELECT_W::new(self, 0)
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
#[doc = "Key selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELECT_SPEC;
impl crate::RegisterSpec for SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`select::R`](R) reader structure"]
impl crate::Readable for SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`select::W`](W) writer structure"]
impl crate::Writable for SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SELECT to value 0"]
impl crate::Resettable for SELECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
