#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `LBCK` reader - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled"]
pub type LBCK_R = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled"]
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx"]
pub type TX_R = crate::FieldReader;
#[doc = "Field `TX` writer - Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - Receive Pin Monitors the actual value of pin m_can_rx 0= The CAN bus is dominant (m_can_rx = ‘0’) 1= The CAN bus is recessive (m_can_rx = ‘1’)"]
pub type RX_R = crate::BitReader;
#[doc = "Field `TXBNP` reader - Tx Buffer Number Prepared Tx Buffer number of message that is ready for transmission. Valid when PVAL is set.Valid values are 0 to 31."]
pub type TXBNP_R = crate::FieldReader;
#[doc = "Field `PVAL` reader - Prepared Valid 0= Value of TXBNP not valid 1= Value of TXBNP valid"]
pub type PVAL_R = crate::BitReader;
#[doc = "Field `TXBNS` reader - Tx Buffer Number Started Tx Buffer number of message whose transmission was started last. Valid when SVAL is set. Valid values are 0 to 31."]
pub type TXBNS_R = crate::FieldReader;
#[doc = "Field `SVAL` reader - Started Valid 0= Value of TXBNS not valid 1= Value of TXBNS valid"]
pub type SVAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin Monitors the actual value of pin m_can_rx 0= The CAN bus is dominant (m_can_rx = ‘0’) 1= The CAN bus is recessive (m_can_rx = ‘1’)"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Tx Buffer Number Prepared Tx Buffer number of message that is ready for transmission. Valid when PVAL is set.Valid values are 0 to 31."]
    #[inline(always)]
    pub fn txbnp(&self) -> TXBNP_R {
        TXBNP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Prepared Valid 0= Value of TXBNP not valid 1= Value of TXBNP valid"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Tx Buffer Number Started Tx Buffer number of message whose transmission was started last. Valid when SVAL is set. Valid values are 0 to 31."]
    #[inline(always)]
    pub fn txbns(&self) -> TXBNS_R {
        TXBNS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Started Valid 0= Value of TXBNS not valid 1= Value of TXBNS valid"]
    #[inline(always)]
    pub fn sval(&self) -> SVAL_R {
        SVAL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<TEST_SPEC> {
        LBCK_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TEST_SPEC> {
        TX_W::new(self, 5)
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
#[doc = "test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
