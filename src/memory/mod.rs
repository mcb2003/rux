mod frame;
pub use frame::Frame;
mod simple_allocator;
pub use simple_allocator::SimpleFrameAllocator;

pub const FRAME_SIZE: usize = 4096;

pub trait FrameAllocator: Iterator<Item = Frame> {
    fn dealloc(&mut self, frame: Frame);
}
