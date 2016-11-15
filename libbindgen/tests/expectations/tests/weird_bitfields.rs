/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nsStyleSVGOpacitySource {
    eStyleSVGOpacitySource_Normal = 0,
    eStyleSVGOpacitySource_ContextFillOpacity = 1,
    eStyleSVGOpacitySource_ContextStrokeOpacity = 2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Weird {
    pub mStrokeDasharrayLength: ::std::os::raw::c_uint,
    pub _bitfield_1: u32,
    pub mClipRule: ::std::os::raw::c_uchar,
    pub mColorInterpolation: ::std::os::raw::c_uchar,
    pub mColorInterpolationFilters: ::std::os::raw::c_uchar,
    pub mFillRule: ::std::os::raw::c_uchar,
    pub mImageRendering: ::std::os::raw::c_uchar,
    pub mPaintOrder: ::std::os::raw::c_uchar,
    pub mShapeRendering: ::std::os::raw::c_uchar,
    pub mStrokeLinecap: ::std::os::raw::c_uchar,
    pub mStrokeLinejoin: ::std::os::raw::c_uchar,
    pub mTextAnchor: ::std::os::raw::c_uchar,
    pub mTextRendering: ::std::os::raw::c_uchar,
    pub _bitfield_2: u16,
}
#[test]
fn bindgen_test_layout_Weird() {
    assert_eq!(::std::mem::size_of::<Weird>() , 24usize);
    assert_eq!(::std::mem::align_of::<Weird>() , 4usize);
}
impl Clone for Weird {
    fn clone(&self) -> Self { *self }
}
impl Weird {
    #[inline]
    pub fn bitTest(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (65535usize as u32)) >>
                                       0u32) as u32)
        }
    }
    #[inline]
    pub fn set_bitTest(&mut self, val: ::std::os::raw::c_uint) {
        self._bitfield_1 &= !(65535usize as u32);
        self._bitfield_1 |=
            ((val as u32 as u32) << 0u32) & (65535usize as u32);
    }
    #[inline]
    pub fn bitTest2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 &
                                        (2147418112usize as u32)) >> 16u32) as
                                      u32)
        }
    }
    #[inline]
    pub fn set_bitTest2(&mut self, val: ::std::os::raw::c_uint) {
        self._bitfield_1 &= !(2147418112usize as u32);
        self._bitfield_1 |=
            ((val as u32 as u32) << 16u32) & (2147418112usize as u32);
    }
    #[inline]
    pub fn mFillOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(((self._bitfield_2 & (7usize as u16)) >>
                                       0u32) as u32)
        }
    }
    #[inline]
    pub fn set_mFillOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        self._bitfield_2 &= !(7usize as u16);
        self._bitfield_2 |= ((val as u32 as u16) << 0u32) & (7usize as u16);
    }
    #[inline]
    pub fn mStrokeOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(((self._bitfield_2 & (56usize as u16)) >>
                                       3u32) as u32)
        }
    }
    #[inline]
    pub fn set_mStrokeOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        self._bitfield_2 &= !(56usize as u16);
        self._bitfield_2 |= ((val as u32 as u16) << 3u32) & (56usize as u16);
    }
    #[inline]
    pub fn mStrokeDasharrayFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_2 & (64usize as u16)) >>
                                       6u32) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeDasharrayFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(64usize as u16);
        self._bitfield_2 |= ((val as u8 as u16) << 6u32) & (64usize as u16);
    }
    #[inline]
    pub fn mStrokeDashoffsetFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_2 & (128usize as u16)) >>
                                       7u32) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeDashoffsetFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(128usize as u16);
        self._bitfield_2 |= ((val as u8 as u16) << 7u32) & (128usize as u16);
    }
    #[inline]
    pub fn mStrokeWidthFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_2 & (256usize as u16)) >>
                                       8u32) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeWidthFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(256usize as u16);
        self._bitfield_2 |= ((val as u8 as u16) << 8u32) & (256usize as u16);
    }
}