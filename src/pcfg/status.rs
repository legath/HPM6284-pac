#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `TRIM_F` reader - default fine trim value"]
pub type TRIM_F_R = crate::FieldReader;
#[doc = "Field `TRIM_C` reader - default coarse trim value"]
pub type TRIM_C_R = crate::FieldReader;
#[doc = "Field `EN_TRIM` reader - default value takes effect 0: default value is invalid 1: default value is valid"]
pub type EN_TRIM_R = crate::BitReader;
#[doc = "Field `SEL24M` reader - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M"]
pub type SEL24M_R = crate::BitReader;
#[doc = "Field `SEL32K` reader - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K"]
pub type SEL32K_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - default fine trim value"]
    #[inline(always)]
    pub fn trim_f(&self) -> TRIM_F_R {
        TRIM_F_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - default coarse trim value"]
    #[inline(always)]
    pub fn trim_c(&self) -> TRIM_C_R {
        TRIM_C_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - default value takes effect 0: default value is invalid 1: default value is valid"]
    #[inline(always)]
    pub fn en_trim(&self) -> EN_TRIM_R {
        EN_TRIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M"]
    #[inline(always)]
    pub fn sel24m(&self) -> SEL24M_R {
        SEL24M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K"]
    #[inline(always)]
    pub fn sel32k(&self) -> SEL32K_R {
        SEL32K_R::new(((self.bits >> 20) & 1) != 0)
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
#[doc = "RC 24M track status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
