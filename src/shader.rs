use ash::util::read_spv;
use once_cell::sync::Lazy;
use std::io::Cursor;

static VERT_SPV: &[u8] = include_bytes!("../shaders/vert.spv");

static FRAG_SPV: &[u8] = include_bytes!("../shaders/frag.spv");

pub static VERT_SHADER: Lazy<Vec<u32>> =
    Lazy::new(|| read_spv(&mut Cursor::new(VERT_SPV)).unwrap());

pub static FRAG_SHADER: Lazy<Vec<u32>> =
    Lazy::new(|| read_spv(&mut Cursor::new(FRAG_SPV)).unwrap());
