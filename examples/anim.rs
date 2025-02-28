// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of an animating widget.

use std::f64::consts::PI;

use druid::kurbo::{Line, Point, Size, Vec2};
use druid::piet::{Color, RenderContext};
use druid::shell::{runloop, WindowBuilder};
use druid::{
    Action, BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};
use druid::{UiMain, UiState};

struct AnimWidget {
    t: f64,
}

impl Widget<u32> for AnimWidget {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base_state: &BaseState,
        _data: &u32,
        _env: &Env,
    ) {
        let center = Point::new(50.0, 50.0);
        let ambit = center + 45.0 * Vec2::from_angle((0.75 + self.t) * 2.0 * PI);
        paint_ctx.stroke(Line::new(center, ambit), &Color::WHITE, 1.0);
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u32,
        _env: &Env,
    ) -> Size {
        bc.constrain((100.0, 100.0))
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        _data: &mut u32,
        _env: &Env,
    ) -> Option<Action> {
        match event {
            Event::MouseDown(_) => {
                self.t = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                self.t += (*interval as f64) * 1e-9;
                if self.t < 1.0 {
                    ctx.request_anim_frame();
                }
                // When we do fine-grained invalidation,
                // no doubt this will be required:
                //ctx.invalidate();
            }
            _ => (),
        }
        None
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: Option<&u32>, _data: &u32, _env: &Env) {}
}

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let root = AnimWidget { t: 0.0 };
    let state = UiState::new(root, 0u32);
    builder.set_title("Animation example");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
