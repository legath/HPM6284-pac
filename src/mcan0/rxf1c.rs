#[doc = "Register `RXF1C` reader"]
pub type R = crate::R<RXF1C_SPEC>;
#[doc = "Register `RXF1C` writer"]
pub type W = crate::W<RXF1C_SPEC>;
#[doc = "Field `F1SA` reader - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)"]
pub type F1SA_R = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)"]
pub type F1SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F1S` reader - Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
pub type F1S_R = crate::FieldReader;
#[doc = "Field `F1S` writer - Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
pub type F1S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1WM` reader - Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled"]
pub type F1WM_R = crate::FieldReader;
#[doc = "Field `F1WM` writer - Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled"]
pub type F1WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1OM` reader - FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1SA_W<RXF1C_SPEC> {
        F1SA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1S_W<RXF1C_SPEC> {
        F1S_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1WM_W<RXF1C_SPEC> {
        F1WM_W::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<RXF1C_SPEC> {
        F1OM_W::new(self, 31)
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
#[doc = "rx fifo1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1C_SPEC;
impl crate::RegisterSpec for RXF1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1c::R`](R) reader structure"]
impl crate::Readable for RXF1C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf1c::W`](W) writer structure"]
impl crate::Writable for RXF1C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1C to value 0"]
impl crate::Resettable for RXF1C_SPEC {
    const RESET_VALUE: u32 = 0;
}
