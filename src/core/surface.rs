use crate::core::*;

pub trait Surface {
    fn get_pixel(&self, pos: IVec2) -> Color;
    fn set_pixel(&mut self, pos: IVec2, color: Color);
    fn size(&self) -> IVec2;

    fn index(pos: IVec2, width: i32) -> i32 {
        pos.y * width + pos.x
    }

    fn clear(&mut self, color: Color) {
        let size = self.size();
        for y in 0..size.y {
            for x in 0..size.x {
                self.set_pixel(ivec2(x, y), color);
            }
        }
    }

    fn blit<S>(
        &mut self,
        src: &S,
        src_pos: Option<IVec2>,
        src_size: Option<IVec2>,
        dst_pos: Option<IVec2>,
        dst_size: Option<IVec2>,
    ) where
        S: Surface,
        Self: Sized,
    {
        let src_pos = src_pos.unwrap_or(IVec2::ZERO);
        let src_size = src_size.unwrap_or_else(|| src.size());
        let dst_pos = dst_pos.unwrap_or(IVec2::ZERO);
        if let Some(dst_size) = dst_size {
            if dst_size == src_size {
                blit::blit_same_size(self, src, src_pos, dst_pos, dst_size);
            } else {
                blit::blit_scale(self, src, src_pos, src_size, dst_pos, dst_size);
            }
        } else {
            blit::blit_same_size(self, src, src_pos, dst_pos, src_size);
        }
    }
}
