#[doc = "Register `TRGOCFG[%s]` reader"]
pub type R = crate::R<TRGOCFG_SPEC>;
#[doc = "Register `TRGOCFG[%s]` writer"]
pub type W = crate::W<TRGOCFG_SPEC>;
#[doc = "Field `TRIGOSEL` reader - This bitfield selects one of the TRGM inputs as output."]
pub type TRIGOSEL_R = crate::FieldReader;
#[doc = "Field `TRIGOSEL` writer - This bitfield selects one of the TRGM inputs as output."]
pub type TRIGOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REDG2PEN` reader - 1- The selected input signal rising edge will be convert to an pulse on output."]
pub type REDG2PEN_R = crate::BitReader;
#[doc = "Field `REDG2PEN` writer - 1- The selected input signal rising edge will be convert to an pulse on output."]
pub type REDG2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEDG2PEN` reader - 1- The selected input signal falling edge will be convert to an pulse on output."]
pub type FEDG2PEN_R = crate::BitReader;
#[doc = "Field `FEDG2PEN` writer - 1- The selected input signal falling edge will be convert to an pulse on output."]
pub type FEDG2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTINV` reader - 1- Invert the output"]
pub type OUTINV_R = crate::BitReader;
#[doc = "Field `OUTINV` writer - 1- Invert the output"]
pub type OUTINV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - This bitfield selects one of the TRGM inputs as output."]
    #[inline(always)]
    pub fn trigosel(&self) -> TRIGOSEL_R {
        TRIGOSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 1- The selected input signal rising edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn redg2pen(&self) -> REDG2PEN_R {
        REDG2PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1- The selected input signal falling edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn fedg2pen(&self) -> FEDG2PEN_R {
        FEDG2PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1- Invert the output"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This bitfield selects one of the TRGM inputs as output."]
    #[inline(always)]
    #[must_use]
    pub fn trigosel(&mut self) -> TRIGOSEL_W<TRGOCFG_SPEC> {
        TRIGOSEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - 1- The selected input signal rising edge will be convert to an pulse on output."]
    #[inline(always)]
    #[must_use]
    pub fn redg2pen(&mut self) -> REDG2PEN_W<TRGOCFG_SPEC> {
        REDG2PEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1- The selected input signal falling edge will be convert to an pulse on output."]
    #[inline(always)]
    #[must_use]
    pub fn fedg2pen(&mut self) -> FEDG2PEN_W<TRGOCFG_SPEC> {
        FEDG2PEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1- Invert the output"]
    #[inline(always)]
    #[must_use]
    pub fn outinv(&mut self) -> OUTINV_W<TRGOCFG_SPEC> {
        OUTINV_W::new(self, 9)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trgocfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trgocfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRGOCFG_SPEC;
impl crate::RegisterSpec for TRGOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trgocfg::R`](R) reader structure"]
impl crate::Readable for TRGOCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trgocfg::W`](W) writer structure"]
impl crate::Writable for TRGOCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRGOCFG[%s]
to value 0"]
impl crate::Resettable for TRGOCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
