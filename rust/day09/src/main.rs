// Saw this blog before and thought it is time to do GUIs
// https://fasterthanli.me/series/advent-of-code-2022/part-9#drawing-stuff-with-egui

use eframe;
use eframe::egui;
use eframe::egui::{Stroke, Color32, Sense};

mod knotts;

#[derive(Copy, Clone)]
struct GridPos {
    x: i32,
    y: i32,
}

struct MyApp {
    head: GridPos,
    tail: GridPos,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = egui::vec2(500.0, 500.0);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            const SIDE: f32 = 16.0;
            let to_panel_pos = |pos: GridPos| {
                (egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE) + center).to_pos2()
            };

            for x in -30..30 {
                for y in -20..20 {
                    let dot = GridPos { x, y };
                    let is_zero = dot.x == 0 && dot.y == 0;

                    let color = if is_zero {
                        Color32::DARK_RED
                    } else {
                        Color32::LIGHT_GRAY
                    };
                    painter.circle_stroke(to_panel_pos(dot), 1.0, Stroke::new(1.0, color));
                }
            }

            // paint the head
            let head_pos = to_panel_pos(self.head);
            painter.circle_stroke(head_pos, 6.0, Stroke::new(2.0, Color32::GREEN));

            // paint the tail
            let tail_pos = to_panel_pos(self.tail);
            painter.circle_stroke(tail_pos, 3.0, Stroke::new(2.0, Color32::YELLOW));

            // paint an arrow from head to tail
            painter.arrow(
                tail_pos,
                head_pos - tail_pos,
                Stroke::new(2.0, Color32::YELLOW),
            )
        });
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "AoC 2022 — Day 9",
        options,
        Box::new(|_cc| {
            Box::new(MyApp {
                head: GridPos { x: 0, y: 0 },
                tail: GridPos { x: 1, y: 1 },
            })
        }),
    );
}

