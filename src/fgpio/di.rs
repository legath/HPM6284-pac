#[doc = r"Register block"]
#[repr(C)]
pub struct DI {
    value: VALUE,
}
impl DI {
    #[doc = "0x00 - GPIO input value"]
    #[inline(always)]
    pub const fn value(&self) -> &VALUE {
        &self.value
    }
}
#[doc = "VALUE (rw) register accessor: GPIO input value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "GPIO input value"]
pub mod value;
