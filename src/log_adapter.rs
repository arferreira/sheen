use crate::Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.enabled(metadata.level().into())
    }
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.level().into()) {
            return;
        }
        let msg = format!("{}", record.args());
        let target = record.target();
        self.log(
            record.level().into(),
            &msg,
            &[("target", &target as &dyn std::fmt::Debug)],
        );
    }
    fn flush(&self) {}
}
