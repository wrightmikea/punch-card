// PunchCard SVG Component

use punch_card_core::punch_card::{CardType, PunchCard as CorePunchCard};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PunchCardProps {
    pub card: CorePunchCard,
    pub current_column: Option<usize>,
}

#[function_component(PunchCard)]
pub fn punch_card(props: &PunchCardProps) -> Html {
    let card = &props.card;
    let current_col = props.current_column;

    // SVG dimensions - proper IBM card aspect ratio (7⅜" × 3¼")
    // Aspect ratio: 7.375 / 3.25 = 2.269
    // Narrower card to allow for side margins (1/4 viewport each side)
    let card_width = 800.0; // Width for card to fit in center 50% of viewport
    let row_height = 22.0; // Vertical spacing between rows
    let text_y = 18.0; // Y position for printed text - very close to top
    let grid_start_y = 30.0; // Start of punch grid - rows 12 and 11 in whitespace above 0
    // Calculate height: grid start + (12 rows * row_height) + space for bottom index row + margin
    let card_height = grid_start_y + (12.0 * row_height) + 15.0; // ~309px, includes bottom index row
    let col_width = card_width / 80.0; // 10px per column
    let punch_width = 6.0; // Punch hole width
    let punch_height = 15.0; // Rectangular (taller than wide)
    let guide_width = 5.0; // Guide holes
    let guide_height = 12.0;
    let corner_cut_size = 30.0; // Size of corner cut triangle

    // Row labels (12, 11, 0-9)
    let row_labels = [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    html! {
        <div class="punch-card-container">
            <svg class="punch-card" viewBox={format!("0 0 {} {}", card_width, card_height)} xmlns="http://www.w3.org/2000/svg">
                // Card background as polygon with corner cut - corner is truly transparent
                <polygon
                    points={format!("{},{} {},{} {},{} {},{} {},{}",
                        corner_cut_size, 0,  // Start after corner cut
                        card_width, 0,        // Top right
                        card_width, card_height,  // Bottom right
                        0, card_height,       // Bottom left
                        0, corner_cut_size)}  // Left side, up to corner cut
                    fill="#f4e8d0"
                    stroke="#999"
                    stroke-width="2" />

                // Column numbers (TOP row: ALL columns 1-80, BETWEEN rows 0 and 1)
                {
                    (0..80).map(|col| {
                        let x = col as f64 * col_width + col_width / 2.0;
                        // Position between row 0 (index 2) and row 1 (index 3)
                        // Pre-printed 0 is at 2.5 * row_height + 3.0
                        // Pre-printed 1 is at 3.5 * row_height + 3.0
                        // So column numbers should be at 3.0 * row_height (midpoint)
                        let y = grid_start_y + 3.0 * row_height;
                        html! {
                            <text x={x.to_string()} y={y.to_string()}
                                  text-anchor="middle" font-size="6" fill="#555"
                                  font-family="monospace" font-weight="bold">
                                { col + 1 }
                            </text>
                        }
                    }).collect::<Html>()
                }

                // Column numbers (BOTTOM row: ALL columns 1-80, BETWEEN row 9 and bottom edge)
                {
                    (0..80).map(|col| {
                        let x = col as f64 * col_width + col_width / 2.0;
                        // Position after row 9 (index 11), before bottom edge
                        let y = grid_start_y + 12.0 * row_height;
                        html! {
                            <text x={x.to_string()} y={y.to_string()}
                                  text-anchor="middle" font-size="6" fill="#555"
                                  font-family="monospace" font-weight="bold">
                                { col + 1 }
                            </text>
                        }
                    }).collect::<Html>()
                }

                // Printed characters (if text mode)
                {
                    if card.card_type() == CardType::Text {
                        (0..80).map(|col_idx| {
                            let x = col_idx as f64 * col_width + col_width / 2.0;
                            if let Some(column) = card.get_column(col_idx)
                                && let Some(ch) = column.printed_char
                            {
                                return html! {
                                    <text x={x.to_string()} y={text_y.to_string()}
                                          text-anchor="middle" font-size="12"
                                          font-family="Courier New, monospace" fill="#000">
                                        { ch }
                                    </text>
                                };
                            }
                            html! {}
                        }).collect::<Html>()
                    } else {
                        html! {}
                    }
                }

                // Column highlight (current position)
                {
                    if let Some(col) = current_col {
                        if col < 80 {
                            let x = col as f64 * col_width;
                            let highlight_height = card_height - grid_start_y;
                            html! {
                                <rect x={x.to_string()} y={grid_start_y.to_string()}
                                      width={col_width.to_string()}
                                      height={highlight_height.to_string()}
                                      fill="#4a90e2" fill-opacity="0.2" />
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                }

                // Guide holes (show all possible punch positions)
                {
                    (0..80).flat_map(|col_idx| {
                        (0..12).map(move |row_idx| {
                            let x = col_idx as f64 * col_width + col_width / 2.0;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0;

                            html! {
                                <ellipse cx={x.to_string()}
                                         cy={y.to_string()}
                                         rx={(guide_width / 2.0).to_string()}
                                         ry={(guide_height / 2.0).to_string()}
                                         fill="none"
                                         stroke="#ccc"
                                         stroke-width="0.5" />
                            }
                        })
                    }).collect::<Html>()
                }

                // Pre-printed digits 0-9 in each column (rows 0-9 are at indices 2-11)
                {
                    (0..80).flat_map(|col_idx| {
                        (0..10).map(move |digit| {
                            let x = col_idx as f64 * col_width + col_width / 2.0;
                            // Row index for digit: 12=0, 11=1, 0=2, 1=3, 2=4, ..., 9=11
                            // So digit 0 is at index 2, digit 1 at index 3, etc.
                            let row_idx = digit + 2;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0 + 3.0;

                            html! {
                                <text x={x.to_string()} y={y.to_string()}
                                      text-anchor="middle" font-size="10" fill="#bbb"
                                      font-family="'Courier New', monospace" font-weight="bold">
                                    { digit }
                                </text>
                            }
                        })
                    }).collect::<Html>()
                }

                // Actual punches (solid black rectangles over guide holes)
                {
                    (0..80).flat_map(|col_idx| {
                        (0..12).filter_map(move |row_idx| {
                            let x = col_idx as f64 * col_width + col_width / 2.0;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0;

                            if let Some(column) = card.get_column(col_idx) {
                                let punch_array = column.punches.as_array();
                                if punch_array[row_idx] {
                                    // Punched hole (rectangular - taller than wide, solid black)
                                    return Some(html! {
                                        <rect x={(x - punch_width / 2.0).to_string()}
                                              y={(y - punch_height / 2.0).to_string()}
                                              width={punch_width.to_string()}
                                              height={punch_height.to_string()}
                                              fill="#000" rx="1" />
                                    });
                                }
                            }
                            None
                        })
                    }).collect::<Html>()
                }

                // Row labels on the left
                {
                    row_labels.iter().enumerate().map(|(idx, &label)| {
                        let y = grid_start_y + idx as f64 * row_height + row_height / 2.0;
                        html! {
                            <text x="-10" y={(y + 5.0).to_string()} text-anchor="end"
                                  font-size="14" fill="#666">
                                { label }
                            </text>
                        }
                    }).collect::<Html>()
                }
            </svg>
        </div>
    }
}
