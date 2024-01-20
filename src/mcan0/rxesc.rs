#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RXESC_SPEC>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RXESC_SPEC>;
#[doc = "Field `F0DS` reader - Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
pub type F0DS_R = crate::FieldReader;
#[doc = "Field `F0DS` writer - Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
pub type F0DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `F1DS` reader - Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
pub type F1DS_R = crate::FieldReader;
#[doc = "Field `F1DS` writer - Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
pub type F1DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
pub type RBDS_R = crate::FieldReader;
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
pub type RBDS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<RXESC_SPEC> {
        F0DS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<RXESC_SPEC> {
        F1DS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<RXESC_SPEC> {
        RBDS_W::new(self, 8)
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
#[doc = "rx buffer/fifo element size configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXESC_SPEC;
impl crate::RegisterSpec for RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RXESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RXESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESC_SPEC {
    const RESET_VALUE: u32 = 0;
}
