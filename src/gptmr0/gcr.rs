#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `SWSYNCT` reader - set this bitfield to trigger software coutner sync event"]
pub type SWSYNCT_R = crate::FieldReader;
#[doc = "Field `SWSYNCT` writer - set this bitfield to trigger software coutner sync event"]
pub type SWSYNCT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - set this bitfield to trigger software coutner sync event"]
    #[inline(always)]
    pub fn swsynct(&self) -> SWSYNCT_R {
        SWSYNCT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - set this bitfield to trigger software coutner sync event"]
    #[inline(always)]
    #[must_use]
    pub fn swsynct(&mut self) -> SWSYNCT_W<GCR_SPEC> {
        SWSYNCT_W::new(self, 0)
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
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
