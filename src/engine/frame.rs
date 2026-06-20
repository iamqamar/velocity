/// Raw RGBA frame extracted from the GStreamer pipeline.
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    /// Packed RGBA pixel data, length = width * height * 4
    pub data: Vec<u8>,
}
