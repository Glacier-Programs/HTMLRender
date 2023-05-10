// different collision techniques

pub fn point_on_rect(rect_tlc: [f32; 2], width: f32, height: f32, point: [f32; 2]) -> bool{
    let x_check = rect_tlc[0] < point[0] && rect_tlc[0] + width > point[0];
    let y_check = rect_tlc[1] < point[1] && rect_tlc[1] + height > point[1];
    x_check && y_check
}