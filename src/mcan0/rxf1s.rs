#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<RXF1S_SPEC>;
#[doc = "Register `RXF1S` writer"]
pub type W = crate::W<RXF1S_SPEC>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63."]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
pub type F1PI_R = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
pub type F1F_R = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = ‘1’ will not set this flag."]
pub type RF1L_R = crate::BitReader;
#[doc = "Field `DMS` reader - Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set"]
pub type DMS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = ‘1’ will not set this flag."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
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
#[doc = "rx fifo1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf1s::W`](W) writer structure"]
impl crate::Writable for RXF1S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {
    const RESET_VALUE: u32 = 0;
}
