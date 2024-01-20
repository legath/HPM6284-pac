#[doc = "Register `KEYADDR` reader"]
pub type R = crate::R<KEYADDR_SPEC>;
#[doc = "Register `KEYADDR` writer"]
pub type W = crate::W<KEYADDR_SPEC>;
#[doc = "Field `SUBWRD` reader - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
pub type SUBWRD_R = crate::FieldReader;
#[doc = "Field `SUBWRD` writer - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
pub type SUBWRD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INDEX` reader - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
pub type INDEX_R = crate::FieldReader;
#[doc = "Field `INDEX` writer - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
    #[inline(always)]
    pub fn subwrd(&self) -> SUBWRD_R {
        SUBWRD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:23 - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
    #[inline(always)]
    #[must_use]
    pub fn subwrd(&mut self) -> SUBWRD_W<KEYADDR_SPEC> {
        SUBWRD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<KEYADDR_SPEC> {
        INDEX_W::new(self, 16)
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
#[doc = "Key Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYADDR_SPEC;
impl crate::RegisterSpec for KEYADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyaddr::R`](R) reader structure"]
impl crate::Readable for KEYADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keyaddr::W`](W) writer structure"]
impl crate::Writable for KEYADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYADDR to value 0x40"]
impl crate::Resettable for KEYADDR_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
