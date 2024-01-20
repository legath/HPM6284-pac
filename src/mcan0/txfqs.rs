#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TXFQS_SPEC>;
#[doc = "Register `TXFQS` writer"]
pub type W = crate::W<TXFQS_SPEC>;
#[doc = "Field `TFFL` reader - Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’) Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’)."]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
pub type TFQF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’) Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’)."]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
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
#[doc = "tx fifo/queue status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfqs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfqs::W`](W) writer structure"]
impl crate::Writable for TXFQS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: u32 = 0;
}
