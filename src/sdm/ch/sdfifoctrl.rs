#[doc = "Register `SDFIFOCTRL` reader"]
pub type R = crate::R<SDFIFOCTRL_SPEC>;
#[doc = "Register `SDFIFOCTRL` writer"]
pub type W = crate::W<SDFIFOCTRL_SPEC>;
#[doc = "Field `D_RDY_INT_EN` reader - FIFO data ready interrupt enable"]
pub type D_RDY_INT_EN_R = crate::BitReader;
#[doc = "Field `D_RDY_INT_EN` writer - FIFO data ready interrupt enable"]
pub type D_RDY_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRSH` reader - FIFO threshold (0,..,16) (fillings > threshold, then gen int)"]
pub type THRSH_R = crate::FieldReader;
#[doc = "Field `THRSH` writer - FIFO threshold (0,..,16) (fillings > threshold, then gen int)"]
pub type THRSH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 2 - FIFO data ready interrupt enable"]
    #[inline(always)]
    pub fn d_rdy_int_en(&self) -> D_RDY_INT_EN_R {
        D_RDY_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:8 - FIFO threshold (0,..,16) (fillings > threshold, then gen int)"]
    #[inline(always)]
    pub fn thrsh(&self) -> THRSH_R {
        THRSH_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - FIFO data ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn d_rdy_int_en(&mut self) -> D_RDY_INT_EN_W<SDFIFOCTRL_SPEC> {
        D_RDY_INT_EN_W::new(self, 2)
    }
    #[doc = "Bits 4:8 - FIFO threshold (0,..,16) (fillings > threshold, then gen int)"]
    #[inline(always)]
    #[must_use]
    pub fn thrsh(&mut self) -> THRSH_W<SDFIFOCTRL_SPEC> {
        THRSH_W::new(self, 4)
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
#[doc = "Data FIFO Path Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdfifoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdfifoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDFIFOCTRL_SPEC;
impl crate::RegisterSpec for SDFIFOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdfifoctrl::R`](R) reader structure"]
impl crate::Readable for SDFIFOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdfifoctrl::W`](W) writer structure"]
impl crate::Writable for SDFIFOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDFIFOCTRL to value 0"]
impl crate::Resettable for SDFIFOCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
