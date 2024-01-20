#[doc = "Register `IRQ_EN` reader"]
pub type R = crate::R<IRQ_EN_SPEC>;
#[doc = "Register `IRQ_EN` writer"]
pub type W = crate::W<IRQ_EN_SPEC>;
#[doc = "Field `IRQ_EN` reader - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
pub type IRQ_EN_R = crate::FieldReader<u16>;
#[doc = "Field `IRQ_EN` writer - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
pub type IRQ_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LOCK` reader - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn irq_en(&mut self) -> IRQ_EN_W<IRQ_EN_SPEC> {
        IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 31 - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<IRQ_EN_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "Tamper interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_EN_SPEC;
impl crate::RegisterSpec for IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_en::R`](R) reader structure"]
impl crate::Readable for IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_en::W`](W) writer structure"]
impl crate::Writable for IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IRQ_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
