#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    data: [DATA; 8],
}
impl KEY {
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &DATA {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &DATA> {
        self.data.iter()
    }
}
#[doc = "DATA (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "no description available"]
pub mod data;
