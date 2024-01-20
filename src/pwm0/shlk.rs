#[doc = "Register `shlk` reader"]
pub type R = crate::R<SHLK_SPEC>;
#[doc = "Register `shlk` writer"]
pub type W = crate::W<SHLK_SPEC>;
#[doc = "Field `SHLK` reader - write 1 to lock all shawdow register, wirte access is not permitted"]
pub type SHLK_R = crate::BitReader;
#[doc = "Field `SHLK` writer - write 1 to lock all shawdow register, wirte access is not permitted"]
pub type SHLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - write 1 to lock all shawdow register, wirte access is not permitted"]
    #[inline(always)]
    pub fn shlk(&self) -> SHLK_R {
        SHLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - write 1 to lock all shawdow register, wirte access is not permitted"]
    #[inline(always)]
    #[must_use]
    pub fn shlk(&mut self) -> SHLK_W<SHLK_SPEC> {
        SHLK_W::new(self, 31)
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
#[doc = "Shadow registers lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shlk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shlk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHLK_SPEC;
impl crate::RegisterSpec for SHLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shlk::R`](R) reader structure"]
impl crate::Readable for SHLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shlk::W`](W) writer structure"]
impl crate::Writable for SHLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets shlk to value 0"]
impl crate::Resettable for SHLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
