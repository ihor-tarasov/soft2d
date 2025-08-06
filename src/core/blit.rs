use crate::core::*;

pub fn blit_same_size<A, B>(dst: &mut A, src: &B, src_pos: IVec2, dst_pos: IVec2, size: IVec2)
where
    A: Surface,
    B: Surface,
{
    let dst_size = dst.size();
    let src_size = src.size();
    for y in 0..size.y {
        let dst_offset_y = dst_pos.y + y;
        if dst_offset_y < 0 || dst_offset_y >= dst_size.y {
            continue;
        }
        let src_offset_y = src_pos.y + y;
        if src_offset_y < 0 || src_offset_y >= src_size.y {
            continue;
        }
        for x in 0..size.x {
            let dst_offset_x = dst_pos.x + x;
            if dst_offset_x < 0 || dst_offset_x >= dst_size.x {
                continue;
            }
            let src_offset_x = src_pos.x + x;
            if src_offset_x < 0 || src_offset_x >= src_size.x {
                continue;
            }
            let src_color = src.get_pixel(ivec2(src_offset_x, src_offset_y));
            if src_color.a() != 0x00 {
                dst.set_pixel(ivec2(dst_offset_x, dst_offset_y), src_color);
            }
        }
    }
}

pub fn blit_scale<A, B>(
    dst: &mut A,
    src: &B,
    src_pos: IVec2,
    src_size: IVec2,
    dst_pos: IVec2,
    dst_size: IVec2,
) where
    A: Surface,
    B: Surface,
{
    let base_size = dst.size();
    let step_x = src_size.x as f32 / dst_size.x as f32;
    let step_y = src_size.y as f32 / dst_size.y as f32;
    for y in 0..dst_size.y {
        let dst_offset_y = dst_pos.y + y;
        if dst_offset_y < 0 || dst_offset_y >= base_size.y {
            continue;
        }
        let src_offset_y = src_pos.y + (y as f32 * step_y) as i32;
        for x in 0..dst_size.x {
            let dst_offset_x = dst_pos.x + x;
            if dst_offset_x < 0 || dst_offset_x >= base_size.x {
                continue;
            }
            let src_offset_x = src_pos.x + (x as f32 * step_x) as i32;
            let src_color = src.get_pixel(ivec2(src_offset_x, src_offset_y));
            if src_color.a() != 0x00 {
                dst.set_pixel(ivec2(dst_offset_x, dst_offset_y), src_color);
            }
        }
    }
}
