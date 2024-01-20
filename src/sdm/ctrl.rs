#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN` reader - Channel Enable"]
pub type CH_EN_R = crate::FieldReader;
#[doc = "Field `CH_EN` writer - Channel Enable"]
pub type CH_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYNC_MDAT` reader - Asserted to double sync the mdat input pin before its usage inside the module"]
pub type SYNC_MDAT_R = crate::FieldReader;
#[doc = "Field `SYNC_MDAT` writer - Asserted to double sync the mdat input pin before its usage inside the module"]
pub type SYNC_MDAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYNC_MCLK` reader - Asserted to double sync the mclk input pin before its usage inside the module"]
pub type SYNC_MCLK_R = crate::FieldReader;
#[doc = "Field `SYNC_MCLK` writer - Asserted to double sync the mclk input pin before its usage inside the module"]
pub type SYNC_MCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHMD` reader - Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined"]
pub type CHMD_R = crate::FieldReader<u16>;
#[doc = "Field `CHMD` writer - Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined"]
pub type CHMD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SFTRST` reader - software reset the module if asserted to be1’b1."]
pub type SFTRST_R = crate::BitReader;
#[doc = "Field `SFTRST` writer - software reset the module if asserted to be1’b1."]
pub type SFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Channel Enable"]
    #[inline(always)]
    pub fn ch_en(&self) -> CH_EN_R {
        CH_EN_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Asserted to double sync the mdat input pin before its usage inside the module"]
    #[inline(always)]
    pub fn sync_mdat(&self) -> SYNC_MDAT_R {
        SYNC_MDAT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Asserted to double sync the mclk input pin before its usage inside the module"]
    #[inline(always)]
    pub fn sync_mclk(&self) -> SYNC_MCLK_R {
        SYNC_MCLK_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:25 - Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined"]
    #[inline(always)]
    pub fn chmd(&self) -> CHMD_R {
        CHMD_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - software reset the module if asserted to be1’b1."]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CTRL_SPEC> {
        IE_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_en(&mut self) -> CH_EN_W<CTRL_SPEC> {
        CH_EN_W::new(self, 2)
    }
    #[doc = "Bits 6:9 - Asserted to double sync the mdat input pin before its usage inside the module"]
    #[inline(always)]
    #[must_use]
    pub fn sync_mdat(&mut self) -> SYNC_MDAT_W<CTRL_SPEC> {
        SYNC_MDAT_W::new(self, 6)
    }
    #[doc = "Bits 10:13 - Asserted to double sync the mclk input pin before its usage inside the module"]
    #[inline(always)]
    #[must_use]
    pub fn sync_mclk(&mut self) -> SYNC_MCLK_W<CTRL_SPEC> {
        SYNC_MCLK_W::new(self, 10)
    }
    #[doc = "Bits 14:25 - Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined"]
    #[inline(always)]
    #[must_use]
    pub fn chmd(&mut self) -> CHMD_W<CTRL_SPEC> {
        CHMD_W::new(self, 14)
    }
    #[doc = "Bit 31 - software reset the module if asserted to be1’b1."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<CTRL_SPEC> {
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
#[doc = "SDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
