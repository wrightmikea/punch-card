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
    let card_width = 1200.0;
    let card_height = 529.0; // Maintains proper aspect ratio
    let col_width = card_width / 80.0; // 15px per column
    let row_height = 30.0; // Increased for better spacing
    let punch_width = 10.0;
    let punch_height = 22.0; // Rectangular (taller than wide)
    let text_y = 35.0; // Y position for printed text
    let grid_start_y = 60.0; // Start of punch grid

    // Row labels (12, 11, 0-9)
    let row_labels = [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    html! {
        <div class="punch-card-container">
            <svg class="punch-card" viewBox={format!("0 0 {} {}", card_width, card_height)} xmlns="http://www.w3.org/2000/svg">
                // Card background with corner cut
                <defs>
                    <clipPath id="card-clip">
                        <path d={format!("M 0 0 L {} 0 L {} {} L 0 {} Z M 0 0 L 30 0 L 0 30 Z",
                            card_width, card_width, card_height, card_height)} />
                    </clipPath>
                </defs>
                <rect x="0" y="0" width={card_width.to_string()} height={card_height.to_string()}
                      fill="#f4e8d0" stroke="#999" stroke-width="2" clip-path="url(#card-clip)" />
                // Corner cut line
                <line x1="0" y1="30" x2="30" y2="0" stroke="#999" stroke-width="2" />

                // Column numbers (every 5th column)
                {
                    (0..80).step_by(5).map(|col| {
                        let x = col as f64 * col_width + col_width / 2.0;
                        html! {
                            <text x={x.to_string()} y="15" text-anchor="middle" font-size="8" fill="#666">
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

                // Punches grid
                {
                    (0..80).flat_map(|col_idx| {
                        (0..12).filter_map(move |row_idx| {
                            let x = col_idx as f64 * col_width + col_width / 2.0;
                            let y = grid_start_y + row_idx as f64 * row_height + row_height / 2.0;

                            if let Some(column) = card.get_column(col_idx) {
                                let punch_array = column.punches.as_array();
                                if punch_array[row_idx] {
                                    // Punched hole (rectangular - taller than wide)
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
