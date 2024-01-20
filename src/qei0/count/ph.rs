#[doc = "Register `ph` reader"]
pub type R = crate::R<PH_SPEC>;
#[doc = "Register `ph` writer"]
pub type W = crate::W<PH_SPEC>;
#[doc = "Field `PHCNT` reader - phcnt value"]
pub type PHCNT_R = crate::FieldReader<u32>;
#[doc = "Field `BSTAT` reader - 1- b input is high 0- b input is low"]
pub type BSTAT_R = crate::BitReader;
#[doc = "Field `ASTAT` reader - 1- a input is high 0- a input is low"]
pub type ASTAT_R = crate::BitReader;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:20 - phcnt value"]
    #[inline(always)]
    pub fn phcnt(&self) -> PHCNT_R {
        PHCNT_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 25 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    pub fn bstat(&self) -> BSTAT_R {
        BSTAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1- a input is high 0- a input is low"]
    #[inline(always)]
    pub fn astat(&self) -> ASTAT_R {
        ASTAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 30) & 1) != 0)
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
#[doc = "Phase counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PH_SPEC;
impl crate::RegisterSpec for PH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ph::R`](R) reader structure"]
impl crate::Readable for PH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ph::W`](W) writer structure"]
impl crate::Writable for PH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ph to value 0"]
impl crate::Resettable for PH_SPEC {
    const RESET_VALUE: u32 = 0;
}
