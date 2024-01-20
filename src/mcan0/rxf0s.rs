#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<RXF0S_SPEC>;
#[doc = "Register `RXF0S` writer"]
pub type W = crate::W<RXF0S_SPEC>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
pub type F0FL_R = crate::FieldReader;
#[doc = "Field `F0GI` reader - Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63."]
pub type F0GI_R = crate::FieldReader;
#[doc = "Field `F0PI` reader - Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
pub type F0PI_R = crate::FieldReader;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
pub type F0F_R = crate::BitReader;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = ‘1’ will not set this flag."]
pub type RF0L_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = ‘1’ will not set this flag."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
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
#[doc = "rx fifo 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for RXF0S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf0s::W`](W) writer structure"]
impl crate::Writable for RXF0S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0S_SPEC {
    const RESET_VALUE: u32 = 0;
}
