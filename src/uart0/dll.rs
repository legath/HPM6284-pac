#[doc = "Register `DLL` reader"]
pub type R = crate::R<DLL_SPEC>;
#[doc = "Register `DLL` writer"]
pub type W = crate::W<DLL_SPEC>;
#[doc = "Field `DLL` reader - Least significant byte of the Divisor Latch"]
pub type DLL_R = crate::FieldReader;
#[doc = "Field `DLL` writer - Least significant byte of the Divisor Latch"]
pub type DLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Least significant byte of the Divisor Latch"]
    #[inline(always)]
    pub fn dll(&self) -> DLL_R {
        DLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Least significant byte of the Divisor Latch"]
    #[inline(always)]
    #[must_use]
    pub fn dll(&mut self) -> DLL_W<DLL_SPEC> {
        DLL_W::new(self, 0)
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
#[doc = "Divisor Latch LSB (when DLAB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for DLL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
