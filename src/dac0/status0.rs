#[doc = "Register `status0` reader"]
pub type R = crate::R<STATUS0_SPEC>;
#[doc = "Register `status0` writer"]
pub type W = crate::W<STATUS0_SPEC>;
#[doc = "Field `CUR_BUF_INDEX` reader - No description avaiable"]
pub type CUR_BUF_INDEX_R = crate::BitReader;
#[doc = "Field `CUR_BUF_INDEX` writer - No description avaiable"]
pub type CUR_BUF_INDEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_BUF_OFFSET` reader - No description avaiable"]
pub type CUR_BUF_OFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `CUR_BUF_OFFSET` writer - No description avaiable"]
pub type CUR_BUF_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    pub fn cur_buf_index(&self) -> CUR_BUF_INDEX_R {
        CUR_BUF_INDEX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:23 - No description avaiable"]
    #[inline(always)]
    pub fn cur_buf_offset(&self) -> CUR_BUF_OFFSET_R {
        CUR_BUF_OFFSET_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 7 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn cur_buf_index(&mut self) -> CUR_BUF_INDEX_W<STATUS0_SPEC> {
        CUR_BUF_INDEX_W::new(self, 7)
    }
    #[doc = "Bits 8:23 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn cur_buf_offset(&mut self) -> CUR_BUF_OFFSET_W<STATUS0_SPEC> {
        CUR_BUF_OFFSET_W::new(self, 8)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS0_SPEC;
impl crate::RegisterSpec for STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for STATUS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status0::W`](W) writer structure"]
impl crate::Writable for STATUS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status0 to value 0"]
impl crate::Resettable for STATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
