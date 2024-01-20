#[doc = "Register `PKTCTL` reader"]
pub type R = crate::R<PKTCTL_SPEC>;
#[doc = "Register `PKTCTL` writer"]
pub type W = crate::W<PKTCTL_SPEC>;
#[doc = "Field `PKTINT` reader - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
pub type PKTINT_R = crate::BitReader;
#[doc = "Field `PKTINT` writer - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
pub type PKTINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRSEMA` reader - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
pub type DCRSEMA_R = crate::BitReader;
#[doc = "Field `DCRSEMA` writer - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
pub type DCRSEMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAIN` reader - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
pub type CHAIN_R = crate::BitReader;
#[doc = "Field `CHAIN` writer - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
pub type CHAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASINI` reader - Hash Initialization packat"]
pub type HASINI_R = crate::BitReader;
#[doc = "Field `HASINI` writer - Hash Initialization packat"]
pub type HASINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASFNL` reader - Hash Termination packet"]
pub type HASFNL_R = crate::BitReader;
#[doc = "Field `HASFNL` writer - Hash Termination packet"]
pub type HASFNL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIPHIV` reader - Load Initial Vector for the AES in this packet."]
pub type CIPHIV_R = crate::BitReader;
#[doc = "Field `CIPHIV` writer - Load Initial Vector for the AES in this packet."]
pub type CIPHIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTTAG` reader - packet tag"]
pub type PKTTAG_R = crate::FieldReader;
#[doc = "Field `PKTTAG` writer - packet tag"]
pub type PKTTAG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
    #[inline(always)]
    pub fn pktint(&self) -> PKTINT_R {
        PKTINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
    #[inline(always)]
    pub fn dcrsema(&self) -> DCRSEMA_R {
        DCRSEMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hash Initialization packat"]
    #[inline(always)]
    pub fn hasini(&self) -> HASINI_R {
        HASINI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash Termination packet"]
    #[inline(always)]
    pub fn hasfnl(&self) -> HASFNL_R {
        HASFNL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Load Initial Vector for the AES in this packet."]
    #[inline(always)]
    pub fn ciphiv(&self) -> CIPHIV_R {
        CIPHIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 24:31 - packet tag"]
    #[inline(always)]
    pub fn pkttag(&self) -> PKTTAG_R {
        PKTTAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
    #[inline(always)]
    #[must_use]
    pub fn pktint(&mut self) -> PKTINT_W<PKTCTL_SPEC> {
        PKTINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
    #[inline(always)]
    #[must_use]
    pub fn dcrsema(&mut self) -> DCRSEMA_W<PKTCTL_SPEC> {
        DCRSEMA_W::new(self, 2)
    }
    #[doc = "Bit 3 - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
    #[inline(always)]
    #[must_use]
    pub fn chain(&mut self) -> CHAIN_W<PKTCTL_SPEC> {
        CHAIN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Hash Initialization packat"]
    #[inline(always)]
    #[must_use]
    pub fn hasini(&mut self) -> HASINI_W<PKTCTL_SPEC> {
        HASINI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Hash Termination packet"]
    #[inline(always)]
    #[must_use]
    pub fn hasfnl(&mut self) -> HASFNL_W<PKTCTL_SPEC> {
        HASFNL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Load Initial Vector for the AES in this packet."]
    #[inline(always)]
    #[must_use]
    pub fn ciphiv(&mut self) -> CIPHIV_W<PKTCTL_SPEC> {
        CIPHIV_W::new(self, 6)
    }
    #[doc = "Bits 24:31 - packet tag"]
    #[inline(always)]
    #[must_use]
    pub fn pkttag(&mut self) -> PKTTAG_W<PKTCTL_SPEC> {
        PKTTAG_W::new(self, 24)
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
#[doc = "Packet Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKTCTL_SPEC;
impl crate::RegisterSpec for PKTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktctl::R`](R) reader structure"]
impl crate::Readable for PKTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pktctl::W`](W) writer structure"]
impl crate::Writable for PKTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTCTL to value 0"]
impl crate::Resettable for PKTCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
