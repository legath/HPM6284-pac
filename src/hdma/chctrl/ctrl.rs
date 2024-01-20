#[doc = "Register `Ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `Ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Channel enable bit 0x0: Disable 0x1: Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Channel enable bit 0x0: Disable 0x1: Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCMASK` reader - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
pub type INTTCMASK_R = crate::BitReader;
#[doc = "Field `INTTCMASK` writer - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
pub type INTTCMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRMASK` reader - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
pub type INTERRMASK_R = crate::BitReader;
#[doc = "Field `INTERRMASK` writer - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
pub type INTERRMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTABTMASK` reader - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
pub type INTABTMASK_R = crate::BitReader;
#[doc = "Field `INTABTMASK` writer - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
pub type INTABTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTREQSEL` reader - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
pub type DSTREQSEL_R = crate::FieldReader;
#[doc = "Field `DSTREQSEL` writer - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
pub type DSTREQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRCREQSEL` reader - Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
pub type SRCREQSEL_R = crate::FieldReader;
#[doc = "Field `SRCREQSEL` writer - Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
pub type SRCREQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSTADDRCTRL` reader - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
pub type DSTADDRCTRL_R = crate::FieldReader;
#[doc = "Field `DSTADDRCTRL` writer - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
pub type DSTADDRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRCADDRCTRL` reader - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
pub type SRCADDRCTRL_R = crate::FieldReader;
#[doc = "Field `SRCADDRCTRL` writer - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
pub type SRCADDRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSTMODE` reader - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
pub type DSTMODE_R = crate::BitReader;
#[doc = "Field `DSTMODE` writer - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
pub type DSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCMODE` reader - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
pub type SRCMODE_R = crate::BitReader;
#[doc = "Field `SRCMODE` writer - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
pub type SRCMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTWIDTH` reader - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
pub type DSTWIDTH_R = crate::FieldReader;
#[doc = "Field `DSTWIDTH` writer - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
pub type DSTWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRCWIDTH` reader - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
pub type SRCWIDTH_R = crate::FieldReader;
#[doc = "Field `SRCWIDTH` writer - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
pub type SRCWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRCBURSTSIZE` reader - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7"]
pub type SRCBURSTSIZE_R = crate::FieldReader;
#[doc = "Field `SRCBURSTSIZE` writer - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7"]
pub type SRCBURSTSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY` reader - Channel priority level 0x0: Lower priority 0x1: Higher priority"]
pub type PRIORITY_R = crate::BitReader;
#[doc = "Field `PRIORITY` writer - Channel priority level 0x0: Lower priority 0x1: Higher priority"]
pub type PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTBUSINFIDX` reader - Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
pub type DSTBUSINFIDX_R = crate::BitReader;
#[doc = "Field `DSTBUSINFIDX` writer - Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
pub type DSTBUSINFIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCBUSINFIDX` reader - Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
pub type SRCBUSINFIDX_R = crate::BitReader;
#[doc = "Field `SRCBUSINFIDX` writer - Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
pub type SRCBUSINFIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable bit 0x0: Disable 0x1: Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    #[inline(always)]
    pub fn inttcmask(&self) -> INTTCMASK_R {
        INTTCMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    #[inline(always)]
    pub fn interrmask(&self) -> INTERRMASK_R {
        INTERRMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    #[inline(always)]
    pub fn intabtmask(&self) -> INTABTMASK_R {
        INTABTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    #[inline(always)]
    pub fn dstreqsel(&self) -> DSTREQSEL_R {
        DSTREQSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    #[inline(always)]
    pub fn srcreqsel(&self) -> SRCREQSEL_R {
        SRCREQSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    #[inline(always)]
    pub fn dstaddrctrl(&self) -> DSTADDRCTRL_R {
        DSTADDRCTRL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    #[inline(always)]
    pub fn srcaddrctrl(&self) -> SRCADDRCTRL_R {
        SRCADDRCTRL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    #[inline(always)]
    pub fn dstmode(&self) -> DSTMODE_R {
        DSTMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    #[inline(always)]
    pub fn srcmode(&self) -> SRCMODE_R {
        SRCMODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
    #[inline(always)]
    pub fn dstwidth(&self) -> DSTWIDTH_R {
        DSTWIDTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
    #[inline(always)]
    pub fn srcwidth(&self) -> SRCWIDTH_R {
        SRCWIDTH_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7"]
    #[inline(always)]
    pub fn srcburstsize(&self) -> SRCBURSTSIZE_R {
        SRCBURSTSIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    #[inline(always)]
    pub fn dstbusinfidx(&self) -> DSTBUSINFIDX_R {
        DSTBUSINFIDX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    #[inline(always)]
    pub fn srcbusinfidx(&self) -> SRCBUSINFIDX_R {
        SRCBUSINFIDX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable bit 0x0: Disable 0x1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn inttcmask(&mut self) -> INTTCMASK_W<CTRL_SPEC> {
        INTTCMASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn interrmask(&mut self) -> INTERRMASK_W<CTRL_SPEC> {
        INTERRMASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intabtmask(&mut self) -> INTABTMASK_W<CTRL_SPEC> {
        INTABTMASK_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    #[inline(always)]
    #[must_use]
    pub fn dstreqsel(&mut self) -> DSTREQSEL_W<CTRL_SPEC> {
        DSTREQSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    #[inline(always)]
    #[must_use]
    pub fn srcreqsel(&mut self) -> SRCREQSEL_W<CTRL_SPEC> {
        SRCREQSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddrctrl(&mut self) -> DSTADDRCTRL_W<CTRL_SPEC> {
        DSTADDRCTRL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddrctrl(&mut self) -> SRCADDRCTRL_W<CTRL_SPEC> {
        SRCADDRCTRL_W::new(self, 14)
    }
    #[doc = "Bit 16 - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    #[inline(always)]
    #[must_use]
    pub fn dstmode(&mut self) -> DSTMODE_W<CTRL_SPEC> {
        DSTMODE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    #[inline(always)]
    #[must_use]
    pub fn srcmode(&mut self) -> SRCMODE_W<CTRL_SPEC> {
        SRCMODE_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
    #[inline(always)]
    #[must_use]
    pub fn dstwidth(&mut self) -> DSTWIDTH_W<CTRL_SPEC> {
        DSTWIDTH_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2"]
    #[inline(always)]
    #[must_use]
    pub fn srcwidth(&mut self) -> SRCWIDTH_W<CTRL_SPEC> {
        SRCWIDTH_W::new(self, 21)
    }
    #[doc = "Bits 24:27 - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7"]
    #[inline(always)]
    #[must_use]
    pub fn srcburstsize(&mut self) -> SRCBURSTSIZE_W<CTRL_SPEC> {
        SRCBURSTSIZE_W::new(self, 24)
    }
    #[doc = "Bit 29 - Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<CTRL_SPEC> {
        PRIORITY_W::new(self, 29)
    }
    #[doc = "Bit 30 - Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn dstbusinfidx(&mut self) -> DSTBUSINFIDX_W<CTRL_SPEC> {
        DSTBUSINFIDX_W::new(self, 30)
    }
    #[doc = "Bit 31 - Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn srcbusinfidx(&mut self) -> SRCBUSINFIDX_W<CTRL_SPEC> {
        SRCBUSINFIDX_W::new(self, 31)
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
#[doc = "Channel n Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Ctrl to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
