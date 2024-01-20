#[doc = "Register `MFI` reader"]
pub type R = crate::R<MFI_SPEC>;
#[doc = "Register `MFI` writer"]
pub type W = crate::W<MFI_SPEC>;
#[doc = "Field `MFI` reader - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid"]
pub type MFI_R = crate::FieldReader;
#[doc = "Field `MFI` writer - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid"]
pub type MFI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENABLE` reader - PLL enable status 0: PLL is off 1: PLL is on"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `RESPONSE` reader - PLL status 0: PLL is not stable 1: PLL is stable for use"]
pub type RESPONSE_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag 0: PLL is stable or shutdown 1: PLL is changing status"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid"]
    #[inline(always)]
    pub fn mfi(&self) -> MFI_R {
        MFI_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 28 - PLL enable status 0: PLL is off 1: PLL is on"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL status 0: PLL is not stable 1: PLL is stable for use"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy flag 0: PLL is stable or shutdown 1: PLL is changing status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid"]
    #[inline(always)]
    #[must_use]
    pub fn mfi(&mut self) -> MFI_W<MFI_SPEC> {
        MFI_W::new(self, 0)
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
#[doc = "PLL0 multiple register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFI_SPEC;
impl crate::RegisterSpec for MFI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfi::R`](R) reader structure"]
impl crate::Readable for MFI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mfi::W`](W) writer structure"]
impl crate::Writable for MFI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MFI to value 0x10"]
impl crate::Resettable for MFI_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
