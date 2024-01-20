#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `LOAD` reader - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
pub type LOAD_R = crate::BitReader;
#[doc = "Field `LOAD` writer - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ` reader - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
pub type READ_R = crate::BitReader;
#[doc = "Field `READ` writer - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<INT_EN_SPEC> {
        LOAD_W::new(self, 0)
    }
    #[doc = "Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<INT_EN_SPEC> {
        READ_W::new(self, 1)
    }
    #[doc = "Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<INT_EN_SPEC> {
        WRITE_W::new(self, 2)
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
#[doc = "interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
