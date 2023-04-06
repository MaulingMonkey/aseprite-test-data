use std::fmt::{self, Debug, Formatter};

/// A basic <code>0x<span style="color: red">RR</span><span style="color: green">GG</span><span style="color: blue">BB</span>AA</code> color
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RGBA(pub u32);
impl Debug for RGBA {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "#{:08x}", self.0)
    }
}

/// An .aseprite file format color
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AseColorFormat {
    Rgba,
    Greyscale,
    Indexed,
}

/// Expected .png color profile
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum PngColorProfile {
    None,
    SRGB,
    Other,
}

/// File data
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct FileSet {
    pub version: &'static str,
    pub name: &'static str,

    pub src_aseprite: &'static [u8],
    pub array_json: &'static [u8],
    pub basic_json: &'static [u8],
    pub hash_json: &'static [u8],
    pub array_png: &'static [u8],
    pub basic_png: &'static [u8],
    pub hash_png: &'static [u8],

    pub ase_color_format: AseColorFormat, // not necessairly the format the exported png is
    pub png_color_profile: PngColorProfile,
    pub size: [u32; 2],
    pub pixel: [u8; 2],
    pub pixels: Option<&'static [RGBA]>,
    pub palette: &'static [RGBA],

    pub n_frames: usize,
    pub n_layers: usize,
    pub n_slices: usize,
    pub n_frametags: usize,
}

impl Default for FileSet {
    fn default() -> Self {
        Self {
            version: "",
            name: "",

            src_aseprite: &[],
            array_json: &[],
            basic_json: &[],
            hash_json: &[],
            array_png: &[],
            basic_png: &[],
            hash_png: &[],

            ase_color_format: AseColorFormat::Rgba,
            png_color_profile: PngColorProfile::SRGB,
            size: [1, 1],
            pixel: [1, 1],
            pixels: Some(&[RGBA(0x00000000)]),
            palette: ARNE32,

            n_frames: 1,
            n_layers: 1,
            n_slices: 0,
            n_frametags: 0,
        }
    }
}

macro_rules! fileset {
    ($ver:literal, $name:literal, $($tt:tt)*) => {
        FileSet {
            version:        $ver,
            name:           $name,

            src_aseprite:   include_bytes!(concat!("data/", $ver, "/_src/",  $name)),

            array_json:     include_bytes!(concat!("data/", $ver, "/array/", $name, ".json")),
            basic_json:     include_bytes!(concat!("data/", $ver, "/basic/", $name, ".json")),
            hash_json:      include_bytes!(concat!("data/", $ver, "/hash/",  $name, ".json")),

            array_png:      include_bytes!(concat!("data/", $ver, "/array/", $name, ".png")),
            basic_png:      include_bytes!(concat!("data/", $ver, "/basic/", $name, ".png")),
            hash_png:       include_bytes!(concat!("data/", $ver, "/hash/",  $name, ".png")),

            $($tt)*,

            .. FileSet::default()
        }
    };
}

impl FileSet {
    pub fn complex_1_2_25() -> Self {
        fileset! { "1.2.25", "complex.aseprite", n_frames: 9, n_layers: 25, n_slices: 3, n_frametags: 6, size: [8,8], pixels: None }
    }

    pub fn list() -> impl Iterator<Item = FileSet> + 'static {
        vec![
            fileset! { "1.2.25", "basic-grey-1x1-black.aseprite",               ase_color_format: AseColorFormat::Greyscale, pixels: Some(&[RGBA(0x000000FF)]) },
            fileset! { "1.2.25", "basic-grey-1x1-white.aseprite",               ase_color_format: AseColorFormat::Greyscale, pixels: Some(&[RGBA(0xFFFFFFFF)]) },

            fileset! { "1.2.25", "basic-indexed-1x1-transparent.aseprite",      ase_color_format: AseColorFormat::Indexed, pixels: Some(&[RGBA(0x00000000)]) },
            fileset! { "1.2.25", "basic-indexed-1x1-green.aseprite",            ase_color_format: AseColorFormat::Indexed, pixels: Some(&[RGBA(0xA3CE27FF)]) },

            fileset! { "1.2.25", "basic-rgba-1x1-empty.aseprite",               pixels: Some(&[RGBA(0x00000000)]) },
            fileset! { "1.2.25", "basic-rgba-1x1-green-none.aseprite",          pixels: Some(&[RGBA(0xA3CE27FF)]), png_color_profile: PngColorProfile::SRGB }, // Setting color profile to "None" doesn't seem to stick
            fileset! { "1.2.25", "basic-rgba-1x1-green-srgb.aseprite",          pixels: Some(&[RGBA(0xA3CE27FF)]), png_color_profile: PngColorProfile::SRGB },
            fileset! { "1.2.25", "basic-rgba-1x1-green-monitor1.aseprite",      pixels: Some(&[RGBA(0xA3CE27FF)]), png_color_profile: PngColorProfile::Other },
            fileset! { "1.2.25", "basic-rgba-1x1-transparent-square.aseprite",  pixel: [1,1] },
            fileset! { "1.2.25", "basic-rgba-1x1-transparent-tall.aseprite",    pixel: [1,2] },
            fileset! { "1.2.25", "basic-rgba-1x1-transparent-wide.aseprite",    pixel: [2,1] },
            fileset! { "1.2.25", "basic-rgba-1x1-transparent.aseprite",         pixel: [1,1] },

            Self::complex_1_2_25(),
        ].into_iter()
    }
}

const ARNE32: &[RGBA] = &[
    RGBA(0x00000000),
    RGBA(0x9D9D9DFF),
    RGBA(0xFFFFFFFF),
    RGBA(0xBE2633FF),
    RGBA(0xE06F8BFF),
    RGBA(0x493C2BFF),
    RGBA(0xA46422FF),
    RGBA(0xEB8931FF),
    RGBA(0xF7E26BFF),
    RGBA(0x2F484EFF),
    RGBA(0x44891AFF),
    RGBA(0xA3CE27FF),
    RGBA(0x1B2632FF),
    RGBA(0x005784FF),
    RGBA(0x31A2F2FF),
    RGBA(0xB2DCEFFF),
    RGBA(0x342A97FF),
    RGBA(0x656D71FF),
    RGBA(0xCCCCCCFF),
    RGBA(0x732930FF),
    RGBA(0xCB43A7FF),
    RGBA(0x524F40FF),
    RGBA(0xAD9D33FF),
    RGBA(0xEC4700FF),
    RGBA(0xFAB40BFF),
    RGBA(0x115E33FF),
    RGBA(0x14807EFF),
    RGBA(0x15C2A5FF),
    RGBA(0x225AF6FF),
    RGBA(0x9964F9FF),
    RGBA(0xF78ED6FF),
    RGBA(0xF4B990FF),
];
