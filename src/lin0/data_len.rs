#[doc = "Register `data_len` reader"]
pub type R = crate::R<DATA_LEN_SPEC>;
#[doc = "Register `data_len` writer"]
pub type W = crate::W<DATA_LEN_SPEC>;
#[doc = "Field `DATA_LENGTH` reader - data length"]
pub type DATA_LENGTH_R = crate::FieldReader;
#[doc = "Field `DATA_LENGTH` writer - data length"]
pub type DATA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENH_CHECK` reader - 1:enhence check mode"]
pub type ENH_CHECK_R = crate::BitReader;
#[doc = "Field `ENH_CHECK` writer - 1:enhence check mode"]
pub type ENH_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - data length"]
    #[inline(always)]
    pub fn data_length(&self) -> DATA_LENGTH_R {
        DATA_LENGTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 1:enhence check mode"]
    #[inline(always)]
    pub fn enh_check(&self) -> ENH_CHECK_R {
        ENH_CHECK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - data length"]
    #[inline(always)]
    #[must_use]
    pub fn data_length(&mut self) -> DATA_LENGTH_W<DATA_LEN_SPEC> {
        DATA_LENGTH_W::new(self, 0)
    }
    #[doc = "Bit 7 - 1:enhence check mode"]
    #[inline(always)]
    #[must_use]
    pub fn enh_check(&mut self) -> ENH_CHECK_W<DATA_LEN_SPEC> {
        ENH_CHECK_W::new(self, 7)
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
#[doc = "data lenth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_LEN_SPEC;
impl crate::RegisterSpec for DATA_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_len::R`](R) reader structure"]
impl crate::Readable for DATA_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_len::W`](W) writer structure"]
impl crate::Writable for DATA_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_len to value 0"]
impl crate::Resettable for DATA_LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
