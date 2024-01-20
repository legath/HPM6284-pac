#[doc = "Register `misc_setting` reader"]
pub type R = crate::R<MISC_SETTING_SPEC>;
#[doc = "Register `misc_setting` writer"]
pub type W = crate::W<MISC_SETTING_SPEC>;
#[doc = "Field `POLY_WIDTH` reader - crc data length"]
pub type POLY_WIDTH_R = crate::FieldReader;
#[doc = "Field `POLY_WIDTH` writer - crc data length"]
pub type POLY_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REV_IN` reader - 0: no wrap input bit order 1: wrap input bit order"]
pub type REV_IN_R = crate::BitReader;
#[doc = "Field `REV_IN` writer - 0: no wrap input bit order 1: wrap input bit order"]
pub type REV_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV_OUT` reader - 0: no wrap output bit order 1: wrap output bit order"]
pub type REV_OUT_R = crate::BitReader;
#[doc = "Field `REV_OUT` writer - 0: no wrap output bit order 1: wrap output bit order"]
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_REV` reader - 0: no wrap input byte order 1: wrap input byte order"]
pub type BYTE_REV_R = crate::BitReader;
#[doc = "Field `BYTE_REV` writer - 0: no wrap input byte order 1: wrap input byte order"]
pub type BYTE_REV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - crc data length"]
    #[inline(always)]
    pub fn poly_width(&self) -> POLY_WIDTH_R {
        POLY_WIDTH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 0: no wrap input bit order 1: wrap input bit order"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: no wrap output bit order 1: wrap output bit order"]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: no wrap input byte order 1: wrap input byte order"]
    #[inline(always)]
    pub fn byte_rev(&self) -> BYTE_REV_R {
        BYTE_REV_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - crc data length"]
    #[inline(always)]
    #[must_use]
    pub fn poly_width(&mut self) -> POLY_WIDTH_W<MISC_SETTING_SPEC> {
        POLY_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 8 - 0: no wrap input bit order 1: wrap input bit order"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<MISC_SETTING_SPEC> {
        REV_IN_W::new(self, 8)
    }
    #[doc = "Bit 16 - 0: no wrap output bit order 1: wrap output bit order"]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<MISC_SETTING_SPEC> {
        REV_OUT_W::new(self, 16)
    }
    #[doc = "Bit 24 - 0: no wrap input byte order 1: wrap input byte order"]
    #[inline(always)]
    #[must_use]
    pub fn byte_rev(&mut self) -> BYTE_REV_W<MISC_SETTING_SPEC> {
        BYTE_REV_W::new(self, 24)
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
#[doc = "chn&amp;index0 misc_setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_setting::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_setting::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SETTING_SPEC;
impl crate::RegisterSpec for MISC_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_setting::R`](R) reader structure"]
impl crate::Readable for MISC_SETTING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc_setting::W`](W) writer structure"]
impl crate::Writable for MISC_SETTING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets misc_setting to value 0"]
impl crate::Resettable for MISC_SETTING_SPEC {
    const RESET_VALUE: u32 = 0;
}
