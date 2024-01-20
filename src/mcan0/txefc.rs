#[doc = "Register `TXEFC` reader"]
pub type R = crate::R<TXEFC_SPEC>;
#[doc = "Register `TXEFC` writer"]
pub type W = crate::W<TXEFC_SPEC>;
#[doc = "Field `EFSA` reader - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)"]
pub type EFSA_R = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)"]
pub type EFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EFS` reader - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1"]
pub type EFS_R = crate::FieldReader;
#[doc = "Field `EFS` writer - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1"]
pub type EFS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFWM` reader - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled"]
pub type EFWM_R = crate::FieldReader;
#[doc = "Field `EFWM` writer - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled"]
pub type EFWM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 2:15 - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)"]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1"]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EFSA_W<TXEFC_SPEC> {
        EFSA_W::new(self, 2)
    }
    #[doc = "Bits 16:21 - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EFS_W<TXEFC_SPEC> {
        EFS_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EFWM_W<TXEFC_SPEC> {
        EFWM_W::new(self, 24)
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
#[doc = "tx event fifo configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFC_SPEC;
impl crate::RegisterSpec for TXEFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefc::R`](R) reader structure"]
impl crate::Readable for TXEFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txefc::W`](W) writer structure"]
impl crate::Writable for TXEFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFC to value 0"]
impl crate::Resettable for TXEFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
