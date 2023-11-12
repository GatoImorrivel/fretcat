use nih_plug::vizia::{prelude::*, vg::{Path, Paint}};

use crate::common::map_value;

pub struct Graph {
    x: Vec<i32>,
    y: Vec<i32>
}

impl Graph {
    pub fn new<X, Y>(cx: &mut Context, x: X, y: Y) -> Handle<Self> 
    where 
        X: Lens<Target = Vec<i32>>,
        Y: Lens<Target = Vec<i32>>,
    {
        Self {
            x: x.get(cx),
            y: y.get(cx),
        }.build(cx, |cx| {

        }).color(Color::white()).background_color(Color::red())
    }
}

impl View for Graph {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bg = cx.background_color();
        let line_color = cx.font_color();

        let bounds = cx.bounds();

        let mut graph_line = Path::new();

        let min_x = *self.x.iter().min().unwrap_or_else(|| &0) as f32;
        let max_x = *self.x.iter().max().unwrap_or_else(|| &0) as f32;

        let min_y = *self.y.iter().min().unwrap_or_else(|| &0) as f32;
        let max_y = *self.y.iter().max().unwrap_or_else(|| &0) as f32;

        let mut is_first = true;
        self.x.iter().zip(self.y.iter()).for_each(|(x,y)| {
            let x = map_value(*x as f32, min_x, max_x, bounds.x, bounds.right());
            let mapped_y = map_value(*y as f32, min_y, max_y, bounds.y, bounds.bottom()) as f64;
            let y = mapped_y - bounds.bottom() as f64;
            let y = (y / 100.0).abs();
            let y = bounds.y as f64 + bounds.w as f64 * y;
            if is_first {
                graph_line.move_to(x, y as f32);
                is_first = false;
            } else {
                graph_line.line_to(x, y as f32);
            }
        });

        let mut bg_path = Path::new();
        bg_path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.fill_path(&bg_path, &Paint::color(bg.into()));

        let mut paint = Paint::color(line_color.into());
        paint.set_line_width(2.0);
        paint.set_line_cap(nih_plug::vizia::vg::LineCap::Square);
        canvas.stroke_path(&graph_line, &paint);
    }
}