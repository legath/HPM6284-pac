#[doc = "Register `DstAddrH` reader"]
pub type R = crate::R<DST_ADDR_H_SPEC>;
#[doc = "Register `DstAddrH` writer"]
pub type W = crate::W<DST_ADDR_H_SPEC>;
#[doc = "Field `DSTADDRH` reader - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
pub type DSTADDRH_R = crate::FieldReader<u32>;
#[doc = "Field `DSTADDRH` writer - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
pub type DSTADDRH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn dstaddrh(&self) -> DSTADDRH_R {
        DSTADDRH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn dstaddrh(&mut self) -> DSTADDRH_W<DST_ADDR_H_SPEC> {
        DSTADDRH_W::new(self, 0)
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
#[doc = "Channel n Destination Address High Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DST_ADDR_H_SPEC;
impl crate::RegisterSpec for DST_ADDR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_h::R`](R) reader structure"]
impl crate::Readable for DST_ADDR_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_h::W`](W) writer structure"]
impl crate::Writable for DST_ADDR_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DstAddrH to value 0x01"]
impl crate::Resettable for DST_ADDR_H_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
