#[doc = "Register `ENDN` reader"]
pub type R = crate::R<ENDN_SPEC>;
#[doc = "Register `ENDN` writer"]
pub type W = crate::W<ENDN_SPEC>;
#[doc = "Field `EVT` reader - Endianness Test Value The endianness test value is 0x87654321."]
pub type EVT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endianness Test Value The endianness test value is 0x87654321."]
    #[inline(always)]
    pub fn evt(&self) -> EVT_R {
        EVT_R::new(self.bits)
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
#[doc = "endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDN_SPEC;
impl crate::RegisterSpec for ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for ENDN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endn::W`](W) writer structure"]
impl crate::Writable for ENDN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDN to value 0x8765_4321"]
impl crate::Resettable for ENDN_SPEC {
    const RESET_VALUE: u32 = 0x8765_4321;
}
