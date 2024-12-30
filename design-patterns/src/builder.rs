#[derive(Default)]
pub struct Widget {
    size: u32,
    color: String,
}

impl Widget {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn color(&self) -> &String {
        &self.color
    }
}

#[derive(Default)]
pub struct WidgetBuilder {
    widget: Widget,
}

impl WidgetBuilder {
    pub fn with_size(&mut self, size: u32) {
        self.widget.size = size;
    }

    pub fn with_color(&mut self, color: &str) {
        self.widget.color = color.parse().unwrap();
    }

    pub fn make(self) -> Widget {
        self.widget
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{Widget, WidgetBuilder};

    #[test]
    fn create_widget() {
        let mut builder = WidgetBuilder::default();
        builder.with_size(10);
        builder.with_color("blue");
        let widget: Widget = builder.make();
        assert_eq!(widget.color(), "blue");
        assert_eq!(widget.size(), 10);
    }
}
