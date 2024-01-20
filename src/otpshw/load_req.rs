#[doc = "Register `LOAD_REQ` reader"]
pub type R = crate::R<LOAD_REQ_SPEC>;
#[doc = "Register `LOAD_REQ` writer"]
pub type W = crate::W<LOAD_REQ_SPEC>;
#[doc = "Field `REQUEST` reader - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
pub type REQUEST_R = crate::FieldReader;
#[doc = "Field `REQUEST` writer - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
pub type REQUEST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> REQUEST_W<LOAD_REQ_SPEC> {
        REQUEST_W::new(self, 0)
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
#[doc = "LOAD Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_REQ_SPEC;
impl crate::RegisterSpec for LOAD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_req::R`](R) reader structure"]
impl crate::Readable for LOAD_REQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_req::W`](W) writer structure"]
impl crate::Writable for LOAD_REQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD_REQ to value 0x07"]
impl crate::Resettable for LOAD_REQ_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
