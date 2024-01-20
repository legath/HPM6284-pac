#[doc = "Register `DstAddr` reader"]
pub type R = crate::R<DST_ADDR_SPEC>;
#[doc = "Register `DstAddr` writer"]
pub type W = crate::W<DST_ADDR_SPEC>;
#[doc = "Field `DSTADDRL` reader - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
pub type DSTADDRL_R = crate::FieldReader<u32>;
#[doc = "Field `DSTADDRL` writer - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
pub type DSTADDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    #[inline(always)]
    pub fn dstaddrl(&self) -> DSTADDRL_R {
        DSTADDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn dstaddrl(&mut self) -> DSTADDRL_W<DST_ADDR_SPEC> {
        DSTADDRL_W::new(self, 0)
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
#[doc = "Channel n Destination Address Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DST_ADDR_SPEC;
impl crate::RegisterSpec for DST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr::R`](R) reader structure"]
impl crate::Readable for DST_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dst_addr::W`](W) writer structure"]
impl crate::Writable for DST_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DstAddr to value 0x01"]
impl crate::Resettable for DST_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
