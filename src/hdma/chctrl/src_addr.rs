#[doc = "Register `SrcAddr` reader"]
pub type R = crate::R<SRC_ADDR_SPEC>;
#[doc = "Register `SrcAddr` writer"]
pub type W = crate::W<SRC_ADDR_SPEC>;
#[doc = "Field `SRCADDRL` reader - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
pub type SRCADDRL_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDRL` writer - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
pub type SRCADDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    #[inline(always)]
    pub fn srcaddrl(&self) -> SRCADDRL_R {
        SRCADDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn srcaddrl(&mut self) -> SRCADDRL_W<SRC_ADDR_SPEC> {
        SRCADDRL_W::new(self, 0)
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
#[doc = "Channel n Source Address Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_ADDR_SPEC;
impl crate::RegisterSpec for SRC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr::R`](R) reader structure"]
impl crate::Readable for SRC_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`src_addr::W`](W) writer structure"]
impl crate::Writable for SRC_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SrcAddr to value 0x01"]
impl crate::Resettable for SRC_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
