#[doc = "Register `SDPCR` reader"]
pub type R = crate::R<SDPCR_SPEC>;
#[doc = "Register `SDPCR` writer"]
pub type W = crate::W<SDPCR_SPEC>;
#[doc = "Field `INTEN` reader - Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSCEN` reader - when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)"]
pub type RDSCEN_R = crate::BitReader;
#[doc = "Field `RDSCEN` writer - when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)"]
pub type RDSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTPKT0IRQ` reader - Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
pub type TSTPKT0IRQ_R = crate::BitReader;
#[doc = "Field `TSTPKT0IRQ` writer - Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
pub type TSTPKT0IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRPDI` reader - Decryption Disable bit, Write to 1 to disable the decryption."]
pub type DCRPDI_R = crate::BitReader;
#[doc = "Field `DCRPDI` writer - Decryption Disable bit, Write to 1 to disable the decryption."]
pub type DCRPDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFEN` reader - Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
pub type CONFEN_R = crate::BitReader;
#[doc = "Field `CONFEN` writer - Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
pub type CONFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPEN` reader - Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
pub type MCPEN_R = crate::BitReader;
#[doc = "Field `MCPEN` writer - Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
pub type MCPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHEN` reader - HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
pub type HASHEN_R = crate::BitReader;
#[doc = "Field `HASHEN` writer - HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIPHEN` reader - Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
pub type CIPHEN_R = crate::BitReader;
#[doc = "Field `CIPHEN` writer - Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
pub type CIPHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASDIS` reader - HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip."]
pub type HASDIS_R = crate::BitReader;
#[doc = "Field `CIPDIS` reader - Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip."]
pub type CIPDIS_R = crate::BitReader;
#[doc = "Field `CLKGAT` reader - Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
pub type CLKGAT_R = crate::BitReader;
#[doc = "Field `CLKGAT` writer - Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
pub type CLKGAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRST` reader - soft reset. Write 1 then 0, to reset the SDP block."]
pub type SFTRST_R = crate::BitReader;
#[doc = "Field `SFTRST` writer - soft reset. Write 1 then 0, to reset the SDP block."]
pub type SFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)"]
    #[inline(always)]
    pub fn rdscen(&self) -> RDSCEN_R {
        RDSCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 17 - Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
    #[inline(always)]
    pub fn tstpkt0irq(&self) -> TSTPKT0IRQ_R {
        TSTPKT0IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Decryption Disable bit, Write to 1 to disable the decryption."]
    #[inline(always)]
    pub fn dcrpdi(&self) -> DCRPDI_R {
        DCRPDI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
    #[inline(always)]
    pub fn confen(&self) -> CONFEN_R {
        CONFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
    #[inline(always)]
    pub fn mcpen(&self) -> MCPEN_R {
        MCPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
    #[inline(always)]
    pub fn ciphen(&self) -> CIPHEN_R {
        CIPHEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip."]
    #[inline(always)]
    pub fn hasdis(&self) -> HASDIS_R {
        HASDIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip."]
    #[inline(always)]
    pub fn cipdis(&self) -> CIPDIS_R {
        CIPDIS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
    #[inline(always)]
    pub fn clkgat(&self) -> CLKGAT_R {
        CLKGAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - soft reset. Write 1 then 0, to reset the SDP block."]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<SDPCR_SPEC> {
        INTEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)"]
    #[inline(always)]
    #[must_use]
    pub fn rdscen(&mut self) -> RDSCEN_W<SDPCR_SPEC> {
        RDSCEN_W::new(self, 8)
    }
    #[doc = "Bit 17 - Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
    #[inline(always)]
    #[must_use]
    pub fn tstpkt0irq(&mut self) -> TSTPKT0IRQ_W<SDPCR_SPEC> {
        TSTPKT0IRQ_W::new(self, 17)
    }
    #[doc = "Bit 19 - Decryption Disable bit, Write to 1 to disable the decryption."]
    #[inline(always)]
    #[must_use]
    pub fn dcrpdi(&mut self) -> DCRPDI_W<SDPCR_SPEC> {
        DCRPDI_W::new(self, 19)
    }
    #[doc = "Bit 20 - Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn confen(&mut self) -> CONFEN_W<SDPCR_SPEC> {
        CONFEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcpen(&mut self) -> MCPEN_W<SDPCR_SPEC> {
        MCPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<SDPCR_SPEC> {
        HASHEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ciphen(&mut self) -> CIPHEN_W<SDPCR_SPEC> {
        CIPHEN_W::new(self, 23)
    }
    #[doc = "Bit 30 - Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
    #[inline(always)]
    #[must_use]
    pub fn clkgat(&mut self) -> CLKGAT_W<SDPCR_SPEC> {
        CLKGAT_W::new(self, 30)
    }
    #[doc = "Bit 31 - soft reset. Write 1 then 0, to reset the SDP block."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<SDPCR_SPEC> {
        SFTRST_W::new(self, 31)
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
#[doc = "SDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDPCR_SPEC;
impl crate::RegisterSpec for SDPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdpcr::R`](R) reader structure"]
impl crate::Readable for SDPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdpcr::W`](W) writer structure"]
impl crate::Writable for SDPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDPCR to value 0x3000_0000"]
impl crate::Resettable for SDPCR_SPEC {
    const RESET_VALUE: u32 = 0x3000_0000;
}
