#[doc = "Register `TransCtrl` reader"]
pub type R = crate::R<TRANS_CTRL_SPEC>;
#[doc = "Register `TransCtrl` writer"]
pub type W = crate::W<TRANS_CTRL_SPEC>;
#[doc = "Field `RDTRANCNT` reader - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
pub type RDTRANCNT_R = crate::FieldReader<u16>;
#[doc = "Field `RDTRANCNT` writer - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
pub type RDTRANCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DUMMYCNT` reader - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
pub type DUMMYCNT_R = crate::FieldReader;
#[doc = "Field `DUMMYCNT` writer - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
pub type DUMMYCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOKENVALUE` reader - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
pub type TOKENVALUE_R = crate::BitReader;
#[doc = "Field `TOKENVALUE` writer - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
pub type TOKENVALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTRANCNT` reader - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
pub type WRTRANCNT_R = crate::FieldReader<u16>;
#[doc = "Field `WRTRANCNT` writer - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
pub type WRTRANCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TOKENEN` reader - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
pub type TOKENEN_R = crate::BitReader;
#[doc = "Field `TOKENEN` writer - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
pub type TOKENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALQUAD` reader - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
pub type DUALQUAD_R = crate::FieldReader;
#[doc = "Field `DUALQUAD` writer - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
pub type DUALQUAD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRANSMODE` reader - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
pub type TRANSMODE_R = crate::FieldReader;
#[doc = "Field `TRANSMODE` writer - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
pub type TRANSMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRFMT` reader - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
pub type ADDRFMT_R = crate::BitReader;
#[doc = "Field `ADDRFMT` writer - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
pub type ADDRFMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDREN` reader - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
pub type ADDREN_R = crate::BitReader;
#[doc = "Field `ADDREN` writer - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
pub type ADDREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDEN` reader - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
pub type CMDEN_R = crate::BitReader;
#[doc = "Field `CMDEN` writer - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
pub type CMDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVDATAONLY` reader - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
pub type SLVDATAONLY_R = crate::BitReader;
#[doc = "Field `SLVDATAONLY` writer - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
pub type SLVDATAONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
    #[inline(always)]
    pub fn rdtrancnt(&self) -> RDTRANCNT_R {
        RDTRANCNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
    #[inline(always)]
    pub fn dummycnt(&self) -> DUMMYCNT_R {
        DUMMYCNT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
    #[inline(always)]
    pub fn tokenvalue(&self) -> TOKENVALUE_R {
        TOKENVALUE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
    #[inline(always)]
    pub fn wrtrancnt(&self) -> WRTRANCNT_R {
        WRTRANCNT_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
    #[inline(always)]
    pub fn tokenen(&self) -> TOKENEN_R {
        TOKENEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
    #[inline(always)]
    pub fn dualquad(&self) -> DUALQUAD_R {
        DUALQUAD_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
    #[inline(always)]
    pub fn transmode(&self) -> TRANSMODE_R {
        TRANSMODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
    #[inline(always)]
    pub fn addrfmt(&self) -> ADDRFMT_R {
        ADDRFMT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
    #[inline(always)]
    pub fn cmden(&self) -> CMDEN_R {
        CMDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
    #[inline(always)]
    pub fn slvdataonly(&self) -> SLVDATAONLY_R {
        SLVDATAONLY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
    #[inline(always)]
    #[must_use]
    pub fn rdtrancnt(&mut self) -> RDTRANCNT_W<TRANS_CTRL_SPEC> {
        RDTRANCNT_W::new(self, 0)
    }
    #[doc = "Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
    #[inline(always)]
    #[must_use]
    pub fn dummycnt(&mut self) -> DUMMYCNT_W<TRANS_CTRL_SPEC> {
        DUMMYCNT_W::new(self, 9)
    }
    #[doc = "Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
    #[inline(always)]
    #[must_use]
    pub fn tokenvalue(&mut self) -> TOKENVALUE_W<TRANS_CTRL_SPEC> {
        TOKENVALUE_W::new(self, 11)
    }
    #[doc = "Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
    #[inline(always)]
    #[must_use]
    pub fn wrtrancnt(&mut self) -> WRTRANCNT_W<TRANS_CTRL_SPEC> {
        WRTRANCNT_W::new(self, 12)
    }
    #[doc = "Bit 21 - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
    #[inline(always)]
    #[must_use]
    pub fn tokenen(&mut self) -> TOKENEN_W<TRANS_CTRL_SPEC> {
        TOKENEN_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dualquad(&mut self) -> DUALQUAD_W<TRANS_CTRL_SPEC> {
        DUALQUAD_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn transmode(&mut self) -> TRANSMODE_W<TRANS_CTRL_SPEC> {
        TRANSMODE_W::new(self, 24)
    }
    #[doc = "Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
    #[inline(always)]
    #[must_use]
    pub fn addrfmt(&mut self) -> ADDRFMT_W<TRANS_CTRL_SPEC> {
        ADDRFMT_W::new(self, 28)
    }
    #[doc = "Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<TRANS_CTRL_SPEC> {
        ADDREN_W::new(self, 29)
    }
    #[doc = "Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
    #[inline(always)]
    #[must_use]
    pub fn cmden(&mut self) -> CMDEN_W<TRANS_CTRL_SPEC> {
        CMDEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn slvdataonly(&mut self) -> SLVDATAONLY_W<TRANS_CTRL_SPEC> {
        SLVDATAONLY_W::new(self, 31)
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
#[doc = "Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trans_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trans_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANS_CTRL_SPEC;
impl crate::RegisterSpec for TRANS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trans_ctrl::R`](R) reader structure"]
impl crate::Readable for TRANS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trans_ctrl::W`](W) writer structure"]
impl crate::Writable for TRANS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TransCtrl to value 0"]
impl crate::Resettable for TRANS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
