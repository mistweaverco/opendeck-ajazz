pub(crate) struct ButtonRect {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

const BUTTON_SIZE: f32 = 0.231_884_06; // From width

const FIRST_COLUMN_X: f32 = 0.0;
const SECOND_COLUMN_X: f32 = 0.384_057_97;
const THIRD_COLUMN_X: f32 = 0.768_115_94;
const FIRST_ROW_Y: f32 = 0.0;
const SECOND_ROW_Y: f32 = 0.625_731;

fn get_offset_ratios(index: usize) -> (f32, f32) {
    match index {
        0 => (FIRST_COLUMN_X, FIRST_ROW_Y),
        1 => (SECOND_COLUMN_X, FIRST_ROW_Y),
        2 => (THIRD_COLUMN_X, FIRST_ROW_Y),
        3 => (FIRST_COLUMN_X, SECOND_ROW_Y),
        4 => (SECOND_COLUMN_X, SECOND_ROW_Y),
        5 => (THIRD_COLUMN_X, SECOND_ROW_Y),
        _ => panic!("Invalid index"),
    }
}

pub(crate) fn calculate_button_rect(width: u32, height: u32) -> (u32, Vec<ButtonRect>) {
    let button_size = (BUTTON_SIZE * width as f32) as u32;

    let button_rects = (0..6)
        .map(|index| {
            let (x_offset, y_offset) = get_offset_ratios(index);
            let x = (x_offset * (width as f32)) as u32;
            let y = (y_offset * (height as f32)) as u32;

            ButtonRect { x, y }
        })
        .collect();

    (button_size, button_rects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_button_rect() {
        let (button_size, button_rects) = calculate_button_rect(640, 480);

        assert_eq!(button_size, 148);
        assert_eq!(button_rects.len(), 6);

        assert_eq!(button_rects[0].x, 0);
        assert_eq!(button_rects[0].y, 0);
        assert_eq!(button_rects[1].x, 245);
        assert_eq!(button_rects[1].y, 0);
        assert_eq!(button_rects[2].x, 491);
        assert_eq!(button_rects[2].y, 0);
        assert_eq!(button_rects[3].x, 0);
        assert_eq!(button_rects[3].y, 300);
        assert_eq!(button_rects[4].x, 245);
        assert_eq!(button_rects[4].y, 300);
        assert_eq!(button_rects[5].x, 491);
        assert_eq!(button_rects[5].y, 300);
    }
}
