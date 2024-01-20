#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    databyte: [DATABYTE; 8],
    control: CONTROL,
    state: STATE,
    error: ERROR,
    data_len: DATA_LEN,
    baudrate_ctl_low: BAUDRATE_CTL_LOW,
    bardrate_ctl_high: BARDRATE_CTL_HIGH,
    id: ID,
    tv: TV,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub const fn databyte(&self, n: usize) -> &DATABYTE {
        &self.databyte[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub fn databyte_iter(&self) -> impl Iterator<Item = &DATABYTE> {
        self.databyte.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte0(&self) -> &DATABYTE {
        self.databyte(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte1(&self) -> &DATABYTE {
        self.databyte(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte2(&self) -> &DATABYTE {
        self.databyte(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte3(&self) -> &DATABYTE {
        self.databyte(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte4(&self) -> &DATABYTE {
        self.databyte(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte5(&self) -> &DATABYTE {
        self.databyte(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte6(&self) -> &DATABYTE {
        self.databyte(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn databytedata_byte7(&self) -> &DATABYTE {
        self.databyte(7)
    }
    #[doc = "0x20 - control register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x24 - state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x28 - error register"]
    #[inline(always)]
    pub const fn error(&self) -> &ERROR {
        &self.error
    }
    #[doc = "0x2c - data lenth register"]
    #[inline(always)]
    pub const fn data_len(&self) -> &DATA_LEN {
        &self.data_len
    }
    #[doc = "0x30 - baudrate control low register"]
    #[inline(always)]
    pub const fn baudrate_ctl_low(&self) -> &BAUDRATE_CTL_LOW {
        &self.baudrate_ctl_low
    }
    #[doc = "0x34 - baudrate control high register"]
    #[inline(always)]
    pub const fn bardrate_ctl_high(&self) -> &BARDRATE_CTL_HIGH {
        &self.bardrate_ctl_high
    }
    #[doc = "0x38 - id register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x3c - timeout control register"]
    #[inline(always)]
    pub const fn tv(&self) -> &TV {
        &self.tv
    }
}
#[doc = "DATABYTE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databyte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`databyte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databyte`]
module"]
pub type DATABYTE = crate::Reg<databyte::DATABYTE_SPEC>;
#[doc = "no description available"]
pub mod databyte;
#[doc = "control (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "control register"]
pub mod control;
#[doc = "state (rw) register accessor: state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "state register"]
pub mod state;
#[doc = "error (rw) register accessor: error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error`]
module"]
pub type ERROR = crate::Reg<error::ERROR_SPEC>;
#[doc = "error register"]
pub mod error;
#[doc = "data_len (rw) register accessor: data lenth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_len`]
module"]
pub type DATA_LEN = crate::Reg<data_len::DATA_LEN_SPEC>;
#[doc = "data lenth register"]
pub mod data_len;
#[doc = "baudrate_ctl_low (rw) register accessor: baudrate control low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudrate_ctl_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudrate_ctl_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudrate_ctl_low`]
module"]
pub type BAUDRATE_CTL_LOW = crate::Reg<baudrate_ctl_low::BAUDRATE_CTL_LOW_SPEC>;
#[doc = "baudrate control low register"]
pub mod baudrate_ctl_low;
#[doc = "bardrate_ctl_high (rw) register accessor: baudrate control high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bardrate_ctl_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bardrate_ctl_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bardrate_ctl_high`]
module"]
pub type BARDRATE_CTL_HIGH = crate::Reg<bardrate_ctl_high::BARDRATE_CTL_HIGH_SPEC>;
#[doc = "baudrate control high register"]
pub mod bardrate_ctl_high;
#[doc = "id (rw) register accessor: id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "id register"]
pub mod id;
#[doc = "tv (rw) register accessor: timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv`]
module"]
pub type TV = crate::Reg<tv::TV_SPEC>;
#[doc = "timeout control register"]
pub mod tv;
