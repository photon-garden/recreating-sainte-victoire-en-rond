use crate::prelude::*;
use derive_builder::Builder;

#[derive(Builder, Debug, Clone, Default)]
#[builder(default)]
pub struct Circle {}

impl Component for Circle {
    fn render(&self, params: &mut RenderParams) {
        let container = params.container;
        let draw = params.draw;

        let center = container.xy();
        let radius = container.lerp_w(0.965 / 2.0);
        let stroke_weight = container.lerp_w(0.002);

        draw.ellipse()
            .xy(center)
            .radius(radius)
            .color(soft_black())
            .stroke_weight(stroke_weight)
            .no_fill();
    }
}
