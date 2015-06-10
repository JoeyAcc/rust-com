#![macro_use]

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct IID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8]
}

pub trait HasIID {
    fn iid<'a>() -> &'a IID;
}

pub trait HasClsID {
    fn clsid<'a>() -> &'a IID;
}

#[macro_export]
macro_rules! impl_iid {
    ($target:ty, $data1:expr, $data2:expr, $data3:expr, $data4_1:expr, $data4_2:expr, $data4_3:expr, $data4_4:expr, $data4_5:expr, $data4_6:expr, $data4_7:expr, $data4_8:expr) => {
        static INTERFACE_ID: $crate::iid::IID = $crate::iid::IID { data1: $data1, data2: $data2, data3: $data3, data4: [ $data4_1, $data4_2, $data4_3, $data4_4, $data4_5, $data4_6, $data4_7, $data4_8 ] };

        impl $crate::iid::HasIID for $target {
            fn iid<'a>() -> &'a $crate::iid::IID {
                &INTERFACE_ID
            }
        }
    }
}

#[macro_export]
macro_rules! impl_clsid {
    ($target:ty, $data1:expr, $data2:expr, $data3:expr, $data4_1:expr, $data4_2:expr, $data4_3:expr, $data4_4:expr, $data4_5:expr, $data4_6:expr, $data4_7:expr, $data4_8:expr) => {
        static CLASS_ID: $crate::iid::IID = $crate::iid::IID { data1: $data1, data2: $data2, data3: $data3, data4: [ $data4_1, $data4_2, $data4_3, $data4_4, $data4_5, $data4_6, $data4_7, $data4_8 ] };

        impl $crate::iid::HasClsID for $target {
            fn clsid<'a>() -> &'a $crate::iid::IID {
                &CLASS_ID
            }
        }
    }
}