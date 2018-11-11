use ui::*;

pub struct Text;

impl Element for Text {
    fn inflate(&mut self, base: &mut Base) {
        base.primitives().text("Žiauriai gerai");
    }

    fn resize(&mut self, base: &mut Base) {
        let box_size = base.box_size();
        base.resolve_size(match box_size {
            BoxSize::Hidden => None,
            BoxSize::Auto => Some(ResolvedSize { w: 100, h: 60 }),
            BoxSize::Fixed { w, h } => Some(ResolvedSize { w, h }),
        })
    }

    fn update(&mut self, _base: &mut Base, _delta: f32) {
        trace!("update Text");
    }
}

pub struct Button {
    margin: i32,
    step: i32,
    delta_acc: f32,
}

impl Button {
    pub fn new() -> Button {
        Button {
            margin: 10,
            step: 1,
            delta_acc: 0.0,
        }
    }
}

impl Element for Button {
    fn inflate(&mut self, base: &mut Base) {
        base.add(Text);
        base.add(Text);
        base.enable_update(true);
    }

    fn resize(&mut self, base: &mut Base) {
        base.layout_vertical(self.margin)
    }

    fn update(&mut self, base: &mut Base, delta: f32) {
        self.delta_acc += delta;
        if self.delta_acc > 0.05 {
            self.margin += self.step;
            if self.margin > 20 || self.margin < 1 {
                self.step = -self.step;
            }
            base.invalidate_size();
            self.delta_acc = 0.0;
        }
    }
}

pub struct Fill {
}

impl Fill {
    pub fn new() -> Fill {
        Fill { }
    }
}

impl Element for Fill {
    fn inflate(&mut self, base: &mut Base) {
        base.add(Button::new());
        base.add(Text);
        base.add(Text);
        base.add(Button::new());
        base.add(Button::new());
    }

    fn resize(&mut self, base: &mut Base) {
        base.layout_horizontal(5)
    }
}