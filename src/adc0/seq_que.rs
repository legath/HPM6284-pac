#[doc = "Register `SEQ_QUE[%s]` reader"]
pub type R = crate::R<SEQ_QUE_SPEC>;
#[doc = "Register `SEQ_QUE[%s]` writer"]
pub type W = crate::W<SEQ_QUE_SPEC>;
#[doc = "Field `CHAN_NUM_4_0` reader - channel number for current conversion"]
pub type CHAN_NUM_4_0_R = crate::FieldReader;
#[doc = "Field `CHAN_NUM_4_0` writer - channel number for current conversion"]
pub type CHAN_NUM_4_0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SEQ_INT_EN` reader - interrupt enable for current conversion"]
pub type SEQ_INT_EN_R = crate::BitReader;
#[doc = "Field `SEQ_INT_EN` writer - interrupt enable for current conversion"]
pub type SEQ_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - channel number for current conversion"]
    #[inline(always)]
    pub fn chan_num_4_0(&self) -> CHAN_NUM_4_0_R {
        CHAN_NUM_4_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - interrupt enable for current conversion"]
    #[inline(always)]
    pub fn seq_int_en(&self) -> SEQ_INT_EN_R {
        SEQ_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel number for current conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan_num_4_0(&mut self) -> CHAN_NUM_4_0_W<SEQ_QUE_SPEC> {
        CHAN_NUM_4_0_W::new(self, 0)
    }
    #[doc = "Bit 5 - interrupt enable for current conversion"]
    #[inline(always)]
    #[must_use]
    pub fn seq_int_en(&mut self) -> SEQ_INT_EN_W<SEQ_QUE_SPEC> {
        SEQ_INT_EN_W::new(self, 5)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_que::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_que::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_QUE_SPEC;
impl crate::RegisterSpec for SEQ_QUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_que::R`](R) reader structure"]
impl crate::Readable for SEQ_QUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_que::W`](W) writer structure"]
impl crate::Writable for SEQ_QUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_QUE[%s]
to value 0"]
impl crate::Resettable for SEQ_QUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
