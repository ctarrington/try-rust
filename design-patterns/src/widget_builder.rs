use crate::builder::{Buildable, Builder};
use crate::{with_primitive, with_str};

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
    with_primitive!(widget, size, with_size, u32);
    with_str!(widget, color, with_color);
}

impl Builder<Widget, WidgetBuilderError> for WidgetBuilder {
    fn build(self) -> Result<Widget, WidgetBuilderError> {
        if self.widget.color == "red" {
            return Err(WidgetBuilderError::InsufficientResources(
                "Sorry out of red".to_string(),
            ));
        }
        Ok(self.widget)
    }
}

impl Buildable<Widget, WidgetBuilderError, WidgetBuilder> for Widget {
    fn builder() -> WidgetBuilder {
        WidgetBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{Buildable, Builder};
    use crate::widget_builder::{Widget, WidgetBuilderError};

    #[test]
    fn fluent_widget_creation() {
        let widget = Widget::builder()
            .with_size(10)
            .with_color("blue")
            .build()
            .unwrap();
        assert_eq!(widget.color(), "blue");
        assert_eq!(widget.size(), 10);
    }

    #[test]
    fn non_fluent_widget_creation() {
        let builder = Widget::builder();
        // need to receive the moved builder
        let builder = builder.with_size(11);
        let builder = builder.with_color("green");
        let widget: Widget = builder.build().unwrap();
        assert_eq!(widget.color(), "green");
        assert_eq!(widget.size(), 11);
    }

    #[test]
    fn out_of_red() {
        let result = Widget::builder().with_size(10).with_color("red").build();

        assert!(matches!(
            result,
            Err(WidgetBuilderError::InsufficientResources(_))
        ));
    }
}
