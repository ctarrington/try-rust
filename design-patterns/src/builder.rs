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

#[derive(Debug)]
pub enum WidgetBuilderError {
    InsufficientResources(String),
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

    pub fn make(self) -> Result<Widget, WidgetBuilderError> {
        if self.widget.color == "red" {
            return Err(WidgetBuilderError::InsufficientResources(
                "Sorry out of red".to_string(),
            ));
        }
        Ok(self.widget)
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{Widget, WidgetBuilder, WidgetBuilderError};

    #[test]
    fn fluent_widget_creation() {
        let widget = WidgetBuilder::default()
            .with_size(10)
            .with_color("blue")
            .make()
            .unwrap();
        assert_eq!(widget.color(), "blue");
        assert_eq!(widget.size(), 10);
    }

    #[test]
    fn non_fluent_widget_creation() {
        let builder = WidgetBuilder::default();
        // need to receive the moved builder
        let builder = builder.with_size(11);
        let builder = builder.with_color("green");
        let widget: Widget = builder.make().unwrap();
        assert_eq!(widget.color(), "green");
        assert_eq!(widget.size(), 11);
    }

    #[test]
    fn out_of_red() {
        let result = WidgetBuilder::default()
            .with_size(10)
            .with_color("red")
            .make();

        assert!(matches!(
            result,
            Err(WidgetBuilderError::InsufficientResources(_))
        ));
    }
}
