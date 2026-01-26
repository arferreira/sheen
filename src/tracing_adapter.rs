use crate::Logger;

pub struct SheenLayer {
    logger: Logger,
}

impl SheenLayer {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }
}

struct FieldVisitor {
    message: String,
    fields: Vec<(String, String)>,
}

impl FieldVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
            fields: Vec::new(),
        }
    }
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        } else {
            self.fields
                .push((field.name().to_string(), format!("{:?}", value)));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for SheenLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let level = (*event.metadata().level()).into();
        let target = event.metadata().target();

        let mut extra: Vec<(&str, &dyn std::fmt::Debug)> =
            vec![("target", &target as &dyn std::fmt::Debug)];
        for (k, v) in &visitor.fields {
            extra.push((k.as_str(), v as &dyn std::fmt::Debug));
        }

        self.logger.log(level, &visitor.message, &extra);
    }
}
