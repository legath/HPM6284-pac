#[doc = "Register `INT_FLAG` reader"]
pub type R = crate::R<INT_FLAG_SPEC>;
#[doc = "Register `INT_FLAG` writer"]
pub type W = crate::W<INT_FLAG_SPEC>;
#[doc = "Field `LOAD` reader - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
pub type LOAD_R = crate::BitReader;
#[doc = "Field `LOAD` writer - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ` reader - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
pub type READ_R = crate::BitReader;
#[doc = "Field `READ` writer - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<INT_FLAG_SPEC> {
        LOAD_W::new(self, 0)
    }
    #[doc = "Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<INT_FLAG_SPEC> {
        READ_W::new(self, 1)
    }
    #[doc = "Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<INT_FLAG_SPEC> {
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
#[doc = "interrupt flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FLAG_SPEC;
impl crate::RegisterSpec for INT_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_flag::R`](R) reader structure"]
impl crate::Readable for INT_FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_flag::W`](W) writer structure"]
impl crate::Writable for INT_FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FLAG to value 0"]
impl crate::Resettable for INT_FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
