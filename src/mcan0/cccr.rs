#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCR_SPEC>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCR_SPEC>;
#[doc = "Field `INIT` reader - Initialization 0= Normal Operation 1= Initialization is started"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization 0= Normal Operation 1= Initialization is started"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
pub type ASM_R = crate::BitReader;
#[doc = "Field `ASM` writer - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_CAN may be set in power down by stopping m_can_hclk and m_can_cclk"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSR` reader - Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
pub type MON_R = crate::BitReader;
#[doc = "Field `MON` writer - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
pub type DAR_R = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
pub type TEST_R = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
pub type FDOE_R = crate::BitReader;
#[doc = "Field `FDOE` writer - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSE` reader - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
pub type BRSE_R = crate::BitReader;
#[doc = "Field `BRSE` writer - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTSU` reader - Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
pub type UTSU_R = crate::BitReader;
#[doc = "Field `UTSU` writer - Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
pub type UTSU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMM` reader - Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO"]
pub type WMM_R = crate::BitReader;
#[doc = "Field `WMM` writer - Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO"]
pub type WMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
pub type PXHD_R = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
pub type EFBI_R = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
pub type TXP_R = crate::BitReader;
#[doc = "Field `TXP` writer - Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
pub type NISO_R = crate::BitReader;
#[doc = "Field `NISO` writer - Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_CAN may be set in power down by stopping m_can_hclk and m_can_cclk"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
    #[inline(always)]
    pub fn utsu(&self) -> UTSU_R {
        UTSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO"]
    #[inline(always)]
    pub fn wmm(&self) -> WMM_R {
        WMM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<CCCR_SPEC> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<CCCR_SPEC> {
        CCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<CCCR_SPEC> {
        ASM_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<CCCR_SPEC> {
        CSR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<CCCR_SPEC> {
        MON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<CCCR_SPEC> {
        DAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<CCCR_SPEC> {
        TEST_W::new(self, 7)
    }
    #[doc = "Bit 8 - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<CCCR_SPEC> {
        FDOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<CCCR_SPEC> {
        BRSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
    #[inline(always)]
    #[must_use]
    pub fn utsu(&mut self) -> UTSU_W<CCCR_SPEC> {
        UTSU_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn wmm(&mut self) -> WMM_W<CCCR_SPEC> {
        WMM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<CCCR_SPEC> {
        PXHD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<CCCR_SPEC> {
        EFBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<CCCR_SPEC> {
        TXP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<CCCR_SPEC> {
        NISO_W::new(self, 15)
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
#[doc = "CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
