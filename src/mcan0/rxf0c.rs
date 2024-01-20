#[doc = "Register `RXF0C` reader"]
pub type R = crate::R<RXF0C_SPEC>;
#[doc = "Register `RXF0C` writer"]
pub type W = crate::W<RXF0C_SPEC>;
#[doc = "Field `F0SA` reader - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)"]
pub type F0SA_R = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)"]
pub type F0SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F0S` reader - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
pub type F0S_R = crate::FieldReader;
#[doc = "Field `F0S` writer - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
pub type F0S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0WM` reader - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled"]
pub type F0WM_R = crate::FieldReader;
#[doc = "Field `F0WM` writer - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled"]
pub type F0WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<RXF0C_SPEC> {
        F0SA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<RXF0C_SPEC> {
        F0S_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<RXF0C_SPEC> {
        F0WM_W::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<RXF0C_SPEC> {
        F0OM_W::new(self, 31)
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
#[doc = "rx fifo 0 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0c::R`](R) reader structure"]
impl crate::Readable for RXF0C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf0c::W`](W) writer structure"]
impl crate::Writable for RXF0C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for RXF0C_SPEC {
    const RESET_VALUE: u32 = 0;
}
