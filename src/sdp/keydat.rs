#[doc = "Register `KEYDAT` reader"]
pub type R = crate::R<KEYDAT_SPEC>;
#[doc = "Register `KEYDAT` writer"]
pub type W = crate::W<KEYDAT_SPEC>;
#[doc = "Field `KEYDAT` reader - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
pub type KEYDAT_R = crate::FieldReader<u32>;
#[doc = "Field `KEYDAT` writer - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
pub type KEYDAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
    #[inline(always)]
    pub fn keydat(&self) -> KEYDAT_R {
        KEYDAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
    #[inline(always)]
    #[must_use]
    pub fn keydat(&mut self) -> KEYDAT_W<KEYDAT_SPEC> {
        KEYDAT_W::new(self, 0)
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
#[doc = "Key Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keydat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keydat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYDAT_SPEC;
impl crate::RegisterSpec for KEYDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keydat::R`](R) reader structure"]
impl crate::Readable for KEYDAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keydat::W`](W) writer structure"]
impl crate::Writable for KEYDAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYDAT to value 0x30"]
impl crate::Resettable for KEYDAT_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
