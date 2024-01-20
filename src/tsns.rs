#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    t: T,
    tmax: TMAX,
    tmin: TMIN,
    age: AGE,
    status: STATUS,
    config: CONFIG,
    validity: VALIDITY,
    flag: FLAG,
    upper_lim_irq: UPPER_LIM_IRQ,
    lower_lim_irq: LOWER_LIM_IRQ,
    upper_lim_rst: UPPER_LIM_RST,
    lower_lim_rst: LOWER_LIM_RST,
    async_: ASYNC,
    _reserved13: [u8; 0x04],
    advan: ADVAN,
}
impl RegisterBlock {
    #[doc = "0x00 - Temperature"]
    #[inline(always)]
    pub const fn t(&self) -> &T {
        &self.t
    }
    #[doc = "0x04 - Maximum Temperature"]
    #[inline(always)]
    pub const fn tmax(&self) -> &TMAX {
        &self.tmax
    }
    #[doc = "0x08 - Minimum Temperature"]
    #[inline(always)]
    pub const fn tmin(&self) -> &TMIN {
        &self.tmin
    }
    #[doc = "0x0c - Sample age"]
    #[inline(always)]
    pub const fn age(&self) -> &AGE {
        &self.age
    }
    #[doc = "0x10 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x14 - Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x18 - Sample validity"]
    #[inline(always)]
    pub const fn validity(&self) -> &VALIDITY {
        &self.validity
    }
    #[doc = "0x1c - Temperature flag"]
    #[inline(always)]
    pub const fn flag(&self) -> &FLAG {
        &self.flag
    }
    #[doc = "0x20 - Maximum temperature to interrupt"]
    #[inline(always)]
    pub const fn upper_lim_irq(&self) -> &UPPER_LIM_IRQ {
        &self.upper_lim_irq
    }
    #[doc = "0x24 - Minimum temperature to interrupt"]
    #[inline(always)]
    pub const fn lower_lim_irq(&self) -> &LOWER_LIM_IRQ {
        &self.lower_lim_irq
    }
    #[doc = "0x28 - Maximum temperature to reset"]
    #[inline(always)]
    pub const fn upper_lim_rst(&self) -> &UPPER_LIM_RST {
        &self.upper_lim_rst
    }
    #[doc = "0x2c - Minimum temperature to reset"]
    #[inline(always)]
    pub const fn lower_lim_rst(&self) -> &LOWER_LIM_RST {
        &self.lower_lim_rst
    }
    #[doc = "0x30 - Configuration in asynchronous mode"]
    #[inline(always)]
    pub const fn async_(&self) -> &ASYNC {
        &self.async_
    }
    #[doc = "0x38 - Advance configuration"]
    #[inline(always)]
    pub const fn advan(&self) -> &ADVAN {
        &self.advan
    }
}
#[doc = "T (rw) register accessor: Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t`]
module"]
pub type T = crate::Reg<t::T_SPEC>;
#[doc = "Temperature"]
pub mod t;
#[doc = "TMAX (rw) register accessor: Maximum Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmax::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmax::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmax`]
module"]
pub type TMAX = crate::Reg<tmax::TMAX_SPEC>;
#[doc = "Maximum Temperature"]
pub mod tmax;
#[doc = "TMIN (rw) register accessor: Minimum Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmin`]
module"]
pub type TMIN = crate::Reg<tmin::TMIN_SPEC>;
#[doc = "Minimum Temperature"]
pub mod tmin;
#[doc = "AGE (rw) register accessor: Sample age\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`age::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`age::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@age`]
module"]
pub type AGE = crate::Reg<age::AGE_SPEC>;
#[doc = "Sample age"]
pub mod age;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "CONFIG (rw) register accessor: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration"]
pub mod config;
#[doc = "VALIDITY (rw) register accessor: Sample validity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`validity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`validity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@validity`]
module"]
pub type VALIDITY = crate::Reg<validity::VALIDITY_SPEC>;
#[doc = "Sample validity"]
pub mod validity;
#[doc = "FLAG (rw) register accessor: Temperature flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flag`]
module"]
pub type FLAG = crate::Reg<flag::FLAG_SPEC>;
#[doc = "Temperature flag"]
pub mod flag;
#[doc = "UPPER_LIM_IRQ (rw) register accessor: Maximum temperature to interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upper_lim_irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upper_lim_irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upper_lim_irq`]
module"]
pub type UPPER_LIM_IRQ = crate::Reg<upper_lim_irq::UPPER_LIM_IRQ_SPEC>;
#[doc = "Maximum temperature to interrupt"]
pub mod upper_lim_irq;
#[doc = "LOWER_LIM_IRQ (rw) register accessor: Minimum temperature to interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lower_lim_irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lower_lim_irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lower_lim_irq`]
module"]
pub type LOWER_LIM_IRQ = crate::Reg<lower_lim_irq::LOWER_LIM_IRQ_SPEC>;
#[doc = "Minimum temperature to interrupt"]
pub mod lower_lim_irq;
#[doc = "UPPER_LIM_RST (rw) register accessor: Maximum temperature to reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upper_lim_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upper_lim_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upper_lim_rst`]
module"]
pub type UPPER_LIM_RST = crate::Reg<upper_lim_rst::UPPER_LIM_RST_SPEC>;
#[doc = "Maximum temperature to reset"]
pub mod upper_lim_rst;
#[doc = "LOWER_LIM_RST (rw) register accessor: Minimum temperature to reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lower_lim_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lower_lim_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lower_lim_rst`]
module"]
pub type LOWER_LIM_RST = crate::Reg<lower_lim_rst::LOWER_LIM_RST_SPEC>;
#[doc = "Minimum temperature to reset"]
pub mod lower_lim_rst;
#[doc = "ASYNC (rw) register accessor: Configuration in asynchronous mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`async_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`async_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_`]
module"]
pub type ASYNC = crate::Reg<async_::ASYNC_SPEC>;
#[doc = "Configuration in asynchronous mode"]
pub mod async_;
#[doc = "ADVAN (rw) register accessor: Advance configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`advan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advan`]
module"]
pub type ADVAN = crate::Reg<advan::ADVAN_SPEC>;
#[doc = "Advance configuration"]
pub mod advan;
