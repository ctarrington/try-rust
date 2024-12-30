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
    pub fn with_size(mut self, size: u32) -> Self {
        self.widget.size = size;
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.widget.color = color.parse().unwrap();
        self
    }

    pub fn make(self) -> Widget {
        self.widget
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{Widget, WidgetBuilder};

    #[test]
    fn fluent_widget_creation() {
        let widget: Widget = WidgetBuilder::default()
            .with_size(10)
            .with_color("blue")
            .make();
        assert_eq!(widget.color(), "blue");
        assert_eq!(widget.size(), 10);
    }

    #[test]
    fn non_fluent_widget_creation() {
        let builder = WidgetBuilder::default();
        // need to receive the moved builder
        let builder = builder.with_size(11);
        let builder = builder.with_color("red");
        let widget: Widget = builder.make();
        assert_eq!(widget.color(), "red");
        assert_eq!(widget.size(), 11);
    }
}
