#[doc = "Register `TSS1` reader"]
pub type R = crate::R<TSS1_SPEC>;
#[doc = "Register `TSS1` writer"]
pub type W = crate::W<TSS1_SPEC>;
#[doc = "Field `TSN` reader - Timestamp New Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when a timestamp was stored in the related Timestamp register. Reading a Timestamp register resets the related bit."]
pub type TSN_R = crate::FieldReader<u16>;
#[doc = "Field `TSL` reader - Timestamp Lost Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when the timestamp stored in the related Timestamp register was overwritten before it was read. Reading a Timestamp register resets the related bit."]
pub type TSL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp New Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when a timestamp was stored in the related Timestamp register. Reading a Timestamp register resets the related bit."]
    #[inline(always)]
    pub fn tsn(&self) -> TSN_R {
        TSN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Timestamp Lost Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when the timestamp stored in the related Timestamp register was overwritten before it was read. Reading a Timestamp register resets the related bit."]
    #[inline(always)]
    pub fn tsl(&self) -> TSL_R {
        TSL_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "timestamp status1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tss1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tss1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSS1_SPEC;
impl crate::RegisterSpec for TSS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tss1::R`](R) reader structure"]
impl crate::Readable for TSS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tss1::W`](W) writer structure"]
impl crate::Writable for TSS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSS1 to value 0"]
impl crate::Resettable for TSS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
