pub trait StringProcessor {
    fn process(&self) -> String;
}

pub struct StringHolder {
    value: String,
}

impl StringProcessor for StringHolder {
    fn process(&self) -> String {
        self.value.clone()
    }
}

pub struct UpperProcessor<'a> {
    processor: &'a dyn StringProcessor,
}

impl StringProcessor for UpperProcessor<'_> {
    fn process(&self) -> String {
        let value = self.processor.process();
        value.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::decorator::{StringHolder, StringProcessor, UpperProcessor};

    #[test]
    fn just_string() {
        let string_holder = StringHolder {
            value: "Hi there".to_string(),
        };
        let string_processor: &dyn StringProcessor = &string_holder;
        assert_eq!("Hi there", string_processor.process());
    }

    #[test]
    fn decorator_to_upper() {
        let string_holder = StringHolder {
            value: "Hi there".to_string(),
        };
        let string_processor_ref: &dyn StringProcessor = &string_holder;

        let to_upper_processor = UpperProcessor {
            processor: string_processor_ref,
        };

        let to_upper_processor_ref: &dyn StringProcessor = &to_upper_processor;
        assert_eq!("HI THERE", to_upper_processor_ref.process());
    }
}
