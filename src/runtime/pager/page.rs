use std::{num::NonZeroU32, ops::Deref};

#[derive(Debug)]
pub(super) struct Page<const N: usize> {
    pos: PagePosition,
    data: [u8; N],
    next: Option<PagePosition>,
}

#[derive(Debug)]
pub(super) struct PagePosition(u32);

impl Deref for PagePosition {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NonZeroU32> for PagePosition {
    fn from(value: NonZeroU32) -> Self {
        Self(value.get())
    }
}

pub(super) trait ValidPage {
    fn data_owned(&self) -> Vec<u8>;
}

impl ValidPage for Page<512> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<1024> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<2048> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<4096> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<8192> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<16384> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<32768> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}
impl ValidPage for Page<65536> {
    fn data_owned(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl<const N: usize> Page<N> {
    pub fn parse(pos: PagePosition, data: [u8; N]) -> Self {
        Self {
            pos,
            data,
            next: None,
        }
    }
}
