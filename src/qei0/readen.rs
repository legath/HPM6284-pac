#[doc = "Register `readen` reader"]
pub type R = crate::R<READEN_SPEC>;
#[doc = "Register `readen` writer"]
pub type W = crate::W<READEN_SPEC>;
#[doc = "Field `ZPHFEN` reader - 1- load counters to their read registers when zphf flag set"]
pub type ZPHFEN_R = crate::BitReader;
#[doc = "Field `ZPHFEN` writer - 1- load counters to their read registers when zphf flag set"]
pub type ZPHFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSCMPFEN` reader - 1- load counters to their read registers when poscmpf flag set"]
pub type POSCMPFEN_R = crate::BitReader;
#[doc = "Field `POSCMPFEN` writer - 1- load counters to their read registers when poscmpf flag set"]
pub type POSCMPFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOMEFEN` reader - 1- load counters to their read registers when homef flag set"]
pub type HOMEFEN_R = crate::BitReader;
#[doc = "Field `HOMEFEN` writer - 1- load counters to their read registers when homef flag set"]
pub type HOMEFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGFEN` reader - 1- load counters to their read registers when wdg flag set"]
pub type WDGFEN_R = crate::BitReader;
#[doc = "Field `WDGFEN` writer - 1- load counters to their read registers when wdg flag set"]
pub type WDGFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - 1- load counters to their read registers when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&self) -> ZPHFEN_R {
        ZPHFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&self) -> POSCMPFEN_R {
        POSCMPFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when homef flag set"]
    #[inline(always)]
    pub fn homefen(&self) -> HOMEFEN_R {
        HOMEFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&self) -> WDGFEN_R {
        WDGFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - 1- load counters to their read registers when zphf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn zphfen(&mut self) -> ZPHFEN_W<READEN_SPEC> {
        ZPHFEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when poscmpf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpfen(&mut self) -> POSCMPFEN_W<READEN_SPEC> {
        POSCMPFEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when homef flag set"]
    #[inline(always)]
    #[must_use]
    pub fn homefen(&mut self) -> HOMEFEN_W<READEN_SPEC> {
        HOMEFEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgfen(&mut self) -> WDGFEN_W<READEN_SPEC> {
        WDGFEN_W::new(self, 31)
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
#[doc = "Read event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READEN_SPEC;
impl crate::RegisterSpec for READEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readen::R`](R) reader structure"]
impl crate::Readable for READEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`readen::W`](W) writer structure"]
impl crate::Writable for READEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets readen to value 0"]
impl crate::Resettable for READEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
