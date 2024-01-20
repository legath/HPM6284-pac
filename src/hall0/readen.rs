#[doc = "Register `readen` reader"]
pub type R = crate::R<READEN_SPEC>;
#[doc = "Register `readen` writer"]
pub type W = crate::W<READEN_SPEC>;
#[doc = "Field `WFEN` reader - 1- load counters to their read registers when w flag set"]
pub type WFEN_R = crate::BitReader;
#[doc = "Field `WFEN` writer - 1- load counters to their read registers when w flag set"]
pub type WFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFEN` reader - 1- load counters to their read registers when v flag set"]
pub type VFEN_R = crate::BitReader;
#[doc = "Field `VFEN` writer - 1- load counters to their read registers when v flag set"]
pub type VFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFEN` reader - 1- load counters to their read registers when u flag set"]
pub type UFEN_R = crate::BitReader;
#[doc = "Field `UFEN` writer - 1- load counters to their read registers when u flag set"]
pub type UFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHDLYEN` reader - 1- load counters to their read registers when phdly flag set"]
pub type PHDLYEN_R = crate::BitReader;
#[doc = "Field `PHDLYEN` writer - 1- load counters to their read registers when phdly flag set"]
pub type PHDLYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHPREEN` reader - 1- load counters to their read registers when phpre flag set"]
pub type PHPREEN_R = crate::BitReader;
#[doc = "Field `PHPREEN` writer - 1- load counters to their read registers when phpre flag set"]
pub type PHPREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHUPTEN` reader - 1- load counters to their read registers when phupt flag set"]
pub type PHUPTEN_R = crate::BitReader;
#[doc = "Field `PHUPTEN` writer - 1- load counters to their read registers when phupt flag set"]
pub type PHUPTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGEN` reader - 1- load counters to their read registers when wdg flag set"]
pub type WDGEN_R = crate::BitReader;
#[doc = "Field `WDGEN` writer - 1- load counters to their read registers when wdg flag set"]
pub type WDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - 1- load counters to their read registers when w flag set"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1- load counters to their read registers when v flag set"]
    #[inline(always)]
    pub fn vfen(&self) -> VFEN_R {
        VFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1- load counters to their read registers when u flag set"]
    #[inline(always)]
    pub fn ufen(&self) -> UFEN_R {
        UFEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- load counters to their read registers when phdly flag set"]
    #[inline(always)]
    pub fn phdlyen(&self) -> PHDLYEN_R {
        PHDLYEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when phpre flag set"]
    #[inline(always)]
    pub fn phpreen(&self) -> PHPREEN_R {
        PHPREEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when phupt flag set"]
    #[inline(always)]
    pub fn phupten(&self) -> PHUPTEN_R {
        PHUPTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - 1- load counters to their read registers when w flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wfen(&mut self) -> WFEN_W<READEN_SPEC> {
        WFEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1- load counters to their read registers when v flag set"]
    #[inline(always)]
    #[must_use]
    pub fn vfen(&mut self) -> VFEN_W<READEN_SPEC> {
        VFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1- load counters to their read registers when u flag set"]
    #[inline(always)]
    #[must_use]
    pub fn ufen(&mut self) -> UFEN_W<READEN_SPEC> {
        UFEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - 1- load counters to their read registers when phdly flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phdlyen(&mut self) -> PHDLYEN_W<READEN_SPEC> {
        PHDLYEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when phpre flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phpreen(&mut self) -> PHPREEN_W<READEN_SPEC> {
        PHPREEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when phupt flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phupten(&mut self) -> PHUPTEN_W<READEN_SPEC> {
        PHUPTEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgen(&mut self) -> WDGEN_W<READEN_SPEC> {
        WDGEN_W::new(self, 31)
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
