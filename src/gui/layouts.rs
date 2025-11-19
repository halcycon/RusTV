use serde::{Deserialize, Serialize};

/// Represents different matrix view layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Layout {
    /// 2x2 grid (4 views)
    #[default]
    Grid2x2,
    /// 3x3 grid (9 views)
    Grid3x3,
    /// 4x4 grid (16 views)
    Grid4x4,
    /// Picture in Picture (1 main + 1 small)
    PiP,
    /// 1 main + 7 small views
    OneAndSeven,
    /// 1 main + 9 small views
    OneAndNine,
}

impl Layout {
    /// Get the number of views for this layout
    pub fn view_count(&self) -> usize {
        match self {
            Layout::Grid2x2 => 4,
            Layout::Grid3x3 => 9,
            Layout::Grid4x4 => 16,
            Layout::PiP => 2,
            Layout::OneAndSeven => 8,
            Layout::OneAndNine => 10,
        }
    }

    /// Get a human-readable name for the layout
    pub fn name(&self) -> &'static str {
        match self {
            Layout::Grid2x2 => "2x2 Grid",
            Layout::Grid3x3 => "3x3 Grid",
            Layout::Grid4x4 => "4x4 Grid",
            Layout::PiP => "Picture in Picture",
            Layout::OneAndSeven => "1+7 Layout",
            Layout::OneAndNine => "1+9 Layout",
        }
    }

    /// Get all available layouts
    pub fn all() -> Vec<Layout> {
        vec![
            Layout::Grid2x2,
            Layout::Grid3x3,
            Layout::Grid4x4,
            Layout::PiP,
            Layout::OneAndSeven,
            Layout::OneAndNine,
        ]
    }

    /// Calculate the position and size for each view in the layout
    /// Returns (x, y, width, height) as fractions of the total area (0.0 to 1.0)
    pub fn calculate_view_rects(&self) -> Vec<(f32, f32, f32, f32)> {
        match self {
            Layout::Grid2x2 => {
                vec![
                    (0.0, 0.0, 0.5, 0.5), // Top-left
                    (0.5, 0.0, 0.5, 0.5), // Top-right
                    (0.0, 0.5, 0.5, 0.5), // Bottom-left
                    (0.5, 0.5, 0.5, 0.5), // Bottom-right
                ]
            }
            Layout::Grid3x3 => {
                let size = 1.0 / 3.0;
                (0..9)
                    .map(|i| {
                        let row = i / 3;
                        let col = i % 3;
                        (col as f32 * size, row as f32 * size, size, size)
                    })
                    .collect()
            }
            Layout::Grid4x4 => {
                let size = 0.25;
                (0..16)
                    .map(|i| {
                        let row = i / 4;
                        let col = i % 4;
                        (col as f32 * size, row as f32 * size, size, size)
                    })
                    .collect()
            }
            Layout::PiP => {
                vec![
                    (0.0, 0.0, 1.0, 1.0),   // Main view (full screen)
                    (0.7, 0.7, 0.25, 0.25), // PiP view (bottom-right corner)
                ]
            }
            Layout::OneAndSeven => {
                // Main view in top-left corner: 75% width, 75% height
                // 4 small views along the right edge (25% width each, 18.75% height)
                // 3 small views along the bottom edge (25% width each, 25% height)
                let main_width = 0.75;
                let main_height = 0.75;
                let small_width_right = 0.25;
                let small_width_bottom = 0.25;
                let small_height_right = main_height / 4.0;
                let small_height_bottom = 0.25;

                let mut rects = vec![
                    (0.0, 0.0, main_width, main_height), // Main view (top-left, 75% x 75%)
                ];

                // 4 small views on the right edge
                for i in 0..4 {
                    let y = i as f32 * small_height_right;
                    rects.push((main_width, y, small_width_right, small_height_right));
                }

                // 3 small views along the bottom edge
                for i in 0..3 {
                    let x = i as f32 * small_width_bottom;
                    rects.push((x, main_height, small_width_bottom, small_height_bottom));
                }

                rects
            }
            Layout::OneAndNine => {
                // Main view in top-left corner: 75% width, 75% height
                // 6 small views along the right edge (25% width each, 12.5% height)
                // 3 small views along the bottom edge (25% width each, 25% height)
                let main_width = 0.75;
                let main_height = 0.75;
                let small_width_right = 0.25;
                let small_width_bottom = 0.25;
                let small_height_right = main_height / 6.0;
                let small_height_bottom = 0.25;

                let mut rects = vec![
                    (0.0, 0.0, main_width, main_height), // Main view (top-left, 75% x 75%)
                ];

                // 6 small views on the right edge
                for i in 0..6 {
                    let y = i as f32 * small_height_right;
                    rects.push((main_width, y, small_width_right, small_height_right));
                }

                // 3 small views along the bottom edge
                for i in 0..3 {
                    let x = i as f32 * small_width_bottom;
                    rects.push((x, main_height, small_width_bottom, small_height_bottom));
                }

                rects
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_view_counts() {
        assert_eq!(Layout::Grid2x2.view_count(), 4);
        assert_eq!(Layout::Grid3x3.view_count(), 9);
        assert_eq!(Layout::Grid4x4.view_count(), 16);
        assert_eq!(Layout::PiP.view_count(), 2);
        assert_eq!(Layout::OneAndSeven.view_count(), 8);
        assert_eq!(Layout::OneAndNine.view_count(), 10);
    }

    #[test]
    fn test_layout_rects() {
        let rects = Layout::Grid2x2.calculate_view_rects();
        assert_eq!(rects.len(), 4);

        let rects = Layout::PiP.calculate_view_rects();
        assert_eq!(rects.len(), 2);

        let rects = Layout::OneAndSeven.calculate_view_rects();
        assert_eq!(rects.len(), 8);

        let rects = Layout::OneAndNine.calculate_view_rects();
        assert_eq!(rects.len(), 10);
    }

    #[test]
    fn test_one_and_seven_layout_positioning() {
        let rects = Layout::OneAndSeven.calculate_view_rects();
        // Main view should be at top-left corner
        assert_eq!(rects[0], (0.0, 0.0, 0.75, 0.75));
        // Small views should be on the right and bottom edges
        assert!(rects[1].0 >= 0.75); // Right edge views have x >= 0.75
        assert!(rects[5].1 >= 0.75); // Bottom edge views have y >= 0.75
    }

    #[test]
    fn test_one_and_nine_layout_positioning() {
        let rects = Layout::OneAndNine.calculate_view_rects();
        // Main view should be at top-left corner
        assert_eq!(rects[0], (0.0, 0.0, 0.75, 0.75));
        // Small views should be on the right and bottom edges
        assert!(rects[1].0 >= 0.75); // Right edge views have x >= 0.75
        assert!(rects[7].1 >= 0.75); // Bottom edge views have y >= 0.75
    }
}
