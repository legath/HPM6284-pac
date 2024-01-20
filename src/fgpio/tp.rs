#[doc = r"Register block"]
#[repr(C)]
pub struct TP {
    value: VALUE,
    set: SET,
    clear: CLEAR,
    toggle: TOGGLE,
}
impl TP {
    #[doc = "0x00 - GPIO interrupt type value"]
    #[inline(always)]
    pub const fn value(&self) -> &VALUE {
        &self.value
    }
    #[doc = "0x04 - GPIO interrupt type set"]
    #[inline(always)]
    pub const fn set(&self) -> &SET {
        &self.set
    }
    #[doc = "0x08 - GPIO interrupt type clear"]
    #[inline(always)]
    pub const fn clear(&self) -> &CLEAR {
        &self.clear
    }
    #[doc = "0x0c - GPIO interrupt type toggle"]
    #[inline(always)]
    pub const fn toggle(&self) -> &TOGGLE {
        &self.toggle
    }
}
#[doc = "VALUE (rw) register accessor: GPIO interrupt type value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "GPIO interrupt type value"]
pub mod value;
#[doc = "SET (rw) register accessor: GPIO interrupt type set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod set;
#[doc = "CLEAR (rw) register accessor: GPIO interrupt type clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod clear;
#[doc = "TOGGLE (rw) register accessor: GPIO interrupt type toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`toggle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toggle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toggle`]
module"]
pub type TOGGLE = crate::Reg<toggle::TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod toggle;
