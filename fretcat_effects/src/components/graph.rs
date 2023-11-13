use nih_plug::vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use crate::common::map_value;

#[derive(Debug, Clone, Copy, Data, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn get_y_axis(points: &[Point]) -> Vec<i32> {
    let y_axis = points.iter().map(|point| point.y).collect::<Vec<_>>();
    y_axis
}

pub fn get_x_axis(points: &[Point]) -> Vec<i32> {
    let x_axis = points.iter().map(|point| point.x).collect::<Vec<_>>();
    x_axis
}

pub struct Graph<L: Lens<Target = Vec<Point>>> {
    points: L,
}

impl<L: Lens<Target = Vec<Point>>> Graph<L> {
    pub fn new(cx: &mut Context, points: L) -> Handle<Self> {
        Self {
            points,
        }
        .build(cx, |_| {})
    }
}

impl<L: Lens<Target = Vec<Point>>> View for Graph<L> {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bg = cx.background_color();
        let line_color = cx.font_color();

        let bounds = cx.bounds();

        let mut graph_line = Path::new();

        let points = self.points.get(cx);
        let x_axis = get_x_axis(&points);
        let min_x = *x_axis.iter().min().unwrap_or_else(|| &0);
        let max_x = *x_axis.iter().max().unwrap_or_else(|| &0);

        let y_axis = get_y_axis(&points);
        let min_y = *y_axis.iter().min().unwrap_or_else(|| &0);
        let max_y = *y_axis.iter().max().unwrap_or_else(|| &0);

        let mut is_first = true;
        points.iter().for_each(|Point { x, y }| {
            let mapped_x = map_value(
                *x as f32,
                min_x as f32,
                max_x as f32,
                bounds.x,
                bounds.right(),
            );
            let mapped_y = map_value(
                *y as f32,
                min_y as f32,
                max_y as f32,
                bounds.y,
                bounds.bottom(),
            );
            let y = bounds.bottom() - (mapped_y - bounds.top());
            if is_first {
                graph_line.move_to(mapped_x as f32, y as f32);
                is_first = false;
            } else {
                graph_line.line_to(mapped_x as f32, y as f32);
            }
        });

        let mut bg_path = Path::new();
        bg_path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.fill_path(&bg_path, &Paint::color(bg.into()));

        let mut paint = Paint::color(line_color.into());
        paint.set_line_width(1.0);
        paint.set_line_cap(nih_plug::vizia::vg::LineCap::Square);
        canvas.stroke_path(&graph_line, &paint);
    }
}
