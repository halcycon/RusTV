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
        ]
    }

    /// Calculate the position and size for each view in the layout
    /// Returns (x, y, width, height) as fractions of the total area (0.0 to 1.0)
    pub fn calculate_view_rects(&self) -> Vec<(f32, f32, f32, f32)> {
        match self {
            Layout::Grid2x2 => {
                vec![
                    (0.0, 0.0, 0.5, 0.5),     // Top-left
                    (0.5, 0.0, 0.5, 0.5),     // Top-right
                    (0.0, 0.5, 0.5, 0.5),     // Bottom-left
                    (0.5, 0.5, 0.5, 0.5),     // Bottom-right
                ]
            }
            Layout::Grid3x3 => {
                let size = 1.0 / 3.0;
                (0..9)
                    .map(|i| {
                        let row = i / 3;
                        let col = i % 3;
                        (
                            col as f32 * size,
                            row as f32 * size,
                            size,
                            size,
                        )
                    })
                    .collect()
            }
            Layout::Grid4x4 => {
                let size = 0.25;
                (0..16)
                    .map(|i| {
                        let row = i / 4;
                        let col = i % 4;
                        (
                            col as f32 * size,
                            row as f32 * size,
                            size,
                            size,
                        )
                    })
                    .collect()
            }
            Layout::PiP => {
                vec![
                    (0.0, 0.0, 1.0, 1.0),           // Main view (full screen)
                    (0.7, 0.7, 0.25, 0.25),         // PiP view (bottom-right corner)
                ]
            }
            Layout::OneAndSeven => {
                let small_width = 0.25;
                let main_width = 0.75;
                let main_height = 1.0;
                
                let mut rects = vec![
                    (0.0, 0.0, main_width, main_height),  // Main view (left 75%)
                ];
                
                // 7 small views on the right side
                for i in 0..7 {
                    let y = i as f32 / 7.0;
                    let h = 1.0 / 7.0;
                    rects.push((0.75, y, small_width, h));
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
    }

    #[test]
    fn test_layout_rects() {
        let rects = Layout::Grid2x2.calculate_view_rects();
        assert_eq!(rects.len(), 4);
        
        let rects = Layout::PiP.calculate_view_rects();
        assert_eq!(rects.len(), 2);
        
        let rects = Layout::OneAndSeven.calculate_view_rects();
        assert_eq!(rects.len(), 8);
    }
}
