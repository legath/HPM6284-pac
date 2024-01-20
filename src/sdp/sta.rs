#[doc = "Register `STA` reader"]
pub type R = crate::R<STA_SPEC>;
#[doc = "Register `STA` writer"]
pub type W = crate::W<STA_SPEC>;
#[doc = "Field `ERRCHAIN` writer - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
pub type ERRCHAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRHAS` writer - Hashing Check Error"]
pub type ERRHAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRDST` writer - Destination Buffer Error"]
pub type ERRDST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSRC` writer - Source Buffer Access Error"]
pub type ERRSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRPKT` writer - Packet head access error, or status update error."]
pub type ERRPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSET` writer - Working mode setup error."]
pub type ERRSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDON` writer - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
pub type PKTDON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTCNT0` writer - Packet Counter registers reachs to ZERO now."]
pub type PKTCNT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASBSY` reader - Hashing Busy"]
pub type HASBSY_R = crate::BitReader;
#[doc = "Field `AESBSY` reader - AES Busy"]
pub type AESBSY_R = crate::BitReader;
#[doc = "Field `CHN1PKT0` writer - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
pub type CHN1PKT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ` writer - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
pub type IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAG` reader - packet tag."]
pub type TAG_R = crate::FieldReader;
impl R {
    #[doc = "Bit 18 - Hashing Busy"]
    #[inline(always)]
    pub fn hasbsy(&self) -> HASBSY_R {
        HASBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AES Busy"]
    #[inline(always)]
    pub fn aesbsy(&self) -> AESBSY_R {
        AESBSY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - packet tag."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
    #[inline(always)]
    #[must_use]
    pub fn errchain(&mut self) -> ERRCHAIN_W<STA_SPEC> {
        ERRCHAIN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hashing Check Error"]
    #[inline(always)]
    #[must_use]
    pub fn errhas(&mut self) -> ERRHAS_W<STA_SPEC> {
        ERRHAS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Destination Buffer Error"]
    #[inline(always)]
    #[must_use]
    pub fn errdst(&mut self) -> ERRDST_W<STA_SPEC> {
        ERRDST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source Buffer Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrc(&mut self) -> ERRSRC_W<STA_SPEC> {
        ERRSRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Packet head access error, or status update error."]
    #[inline(always)]
    #[must_use]
    pub fn errpkt(&mut self) -> ERRPKT_W<STA_SPEC> {
        ERRPKT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Working mode setup error."]
    #[inline(always)]
    #[must_use]
    pub fn errset(&mut self) -> ERRSET_W<STA_SPEC> {
        ERRSET_W::new(self, 5)
    }
    #[doc = "Bit 16 - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
    #[inline(always)]
    #[must_use]
    pub fn pktdon(&mut self) -> PKTDON_W<STA_SPEC> {
        PKTDON_W::new(self, 16)
    }
    #[doc = "Bit 17 - Packet Counter registers reachs to ZERO now."]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt0(&mut self) -> PKTCNT0_W<STA_SPEC> {
        PKTCNT0_W::new(self, 17)
    }
    #[doc = "Bit 20 - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
    #[inline(always)]
    #[must_use]
    pub fn chn1pkt0(&mut self) -> CHN1PKT0_W<STA_SPEC> {
        CHN1PKT0_W::new(self, 20)
    }
    #[doc = "Bit 23 - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<STA_SPEC> {
        IRQ_W::new(self, 23)
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
#[doc = "Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sta::W`](W) writer structure"]
impl crate::Writable for STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: u32 = 0;
}
