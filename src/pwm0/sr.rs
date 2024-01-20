#[doc = "Register `sr` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `sr` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `CMPFX` writer - comparator output compare or input capture flag"]
pub type CMPFX_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RLDF` writer - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
pub type RLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFRLDF` writer - half reload flag, this flag set when cnt count to rld/2"]
pub type HALFRLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRLDF` writer - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
pub type XRLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTF` writer - fault condition flag"]
pub type FAULTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfx(&mut self) -> CMPFX_W<SR_SPEC> {
        CMPFX_W::new(self, 0)
    }
    #[doc = "Bit 24 - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn rldf(&mut self) -> RLDF_W<SR_SPEC> {
        RLDF_W::new(self, 24)
    }
    #[doc = "Bit 25 - half reload flag, this flag set when cnt count to rld/2"]
    #[inline(always)]
    #[must_use]
    pub fn halfrldf(&mut self) -> HALFRLDF_W<SR_SPEC> {
        HALFRLDF_W::new(self, 25)
    }
    #[doc = "Bit 26 - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn xrldf(&mut self) -> XRLDF_W<SR_SPEC> {
        XRLDF_W::new(self, 26)
    }
    #[doc = "Bit 27 - fault condition flag"]
    #[inline(always)]
    #[must_use]
    pub fn faultf(&mut self) -> FAULTF_W<SR_SPEC> {
        FAULTF_W::new(self, 27)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sr to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
