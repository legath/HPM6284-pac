#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TXBC_SPEC>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TXBC_SPEC>;
#[doc = "Field `TBSA` reader - Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
pub type TBSA_R = crate::FieldReader<u16>;
#[doc = "Field `TBSA` writer - Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
pub type TBSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NDTB` reader - Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32"]
pub type NDTB_R = crate::FieldReader;
#[doc = "Field `NDTB` writer - Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32"]
pub type NDTB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQS` reader - Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32"]
pub type TFQS_R = crate::FieldReader;
#[doc = "Field `TFQS` writer - Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32"]
pub type TFQS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQM` reader - Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
pub type TFQM_R = crate::BitReader;
#[doc = "Field `TFQM` writer - Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TBSA_W<TXBC_SPEC> {
        TBSA_W::new(self, 2)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NDTB_W<TXBC_SPEC> {
        NDTB_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TFQS_W<TXBC_SPEC> {
        TFQS_W::new(self, 24)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<TXBC_SPEC> {
        TFQM_W::new(self, 30)
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
#[doc = "tx buffer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TXBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TXBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TXBC_SPEC {
    const RESET_VALUE: u32 = 0;
}
