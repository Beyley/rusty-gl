use gl;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTarget {
    ArrayBuffer = gl::ARRAY_BUFFER
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLUsage {
    StaticDraw = gl::STATIC_DRAW
}