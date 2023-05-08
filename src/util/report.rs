use super::*;

pub trait Report: Sized {
    type Item;
    type Error: Display;
    fn into_result(self) -> Result<Self::Item, Self::Error>;
    fn report_err(self) {
        if let Err(error) = self.into_result() {
            log::error!("{error}");
        }
    }
    fn report_warn(self) {
        if let Err(error) = self.into_result() {
            log::warn!("{error}");
        }
    }
}

impl<T, E: Display> Report for Result<T, E> {
    type Item = T;
    type Error = E;
    fn into_result(self) -> Result<Self::Item, Self::Error> {
        self
    }
}
