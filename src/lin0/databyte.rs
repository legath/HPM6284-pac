#[doc = "Register `DATABYTE[%s]` reader"]
pub type R = crate::R<DATABYTE_SPEC>;
#[doc = "Register `DATABYTE[%s]` writer"]
pub type W = crate::W<DATABYTE_SPEC>;
#[doc = "Field `DATA_BYTE` reader - data byte"]
pub type DATA_BYTE_R = crate::FieldReader;
#[doc = "Field `DATA_BYTE` writer - data byte"]
pub type DATA_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data byte"]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_byte(&mut self) -> DATA_BYTE_W<DATABYTE_SPEC> {
        DATA_BYTE_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`databyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATABYTE_SPEC;
impl crate::RegisterSpec for DATABYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databyte::R`](R) reader structure"]
impl crate::Readable for DATABYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`databyte::W`](W) writer structure"]
impl crate::Writable for DATABYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATABYTE[%s]
to value 0"]
impl crate::Resettable for DATABYTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
