#[doc = "Register `BUF_ADDR[%s]` reader"]
pub type R = crate::R<BUF_ADDR_SPEC>;
#[doc = "Register `BUF_ADDR[%s]` writer"]
pub type W = crate::W<BUF_ADDR_SPEC>;
#[doc = "Field `BUF_STOP` reader - set to stop read point at end of bufffer0"]
pub type BUF_STOP_R = crate::BitReader;
#[doc = "Field `BUF_STOP` writer - set to stop read point at end of bufffer0"]
pub type BUF_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_START_ADDR` reader - buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
pub type BUF_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `BUF_START_ADDR` writer - buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
pub type BUF_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - set to stop read point at end of bufffer0"]
    #[inline(always)]
    pub fn buf_stop(&self) -> BUF_STOP_R {
        BUF_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
    #[inline(always)]
    pub fn buf_start_addr(&self) -> BUF_START_ADDR_R {
        BUF_START_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - set to stop read point at end of bufffer0"]
    #[inline(always)]
    #[must_use]
    pub fn buf_stop(&mut self) -> BUF_STOP_W<BUF_ADDR_SPEC> {
        BUF_STOP_W::new(self, 0)
    }
    #[doc = "Bits 2:31 - buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
    #[inline(always)]
    #[must_use]
    pub fn buf_start_addr(&mut self) -> BUF_START_ADDR_W<BUF_ADDR_SPEC> {
        BUF_START_ADDR_W::new(self, 2)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_ADDR_SPEC;
impl crate::RegisterSpec for BUF_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_addr::R`](R) reader structure"]
impl crate::Readable for BUF_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_addr::W`](W) writer structure"]
impl crate::Writable for BUF_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF_ADDR[%s]
to value 0"]
impl crate::Resettable for BUF_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
