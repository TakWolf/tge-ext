use super::ResolutionPolicy;

pub trait ResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy;

    fn set_policy(&mut self, policy: ResolutionPolicy);

    fn begin_policy(&mut self);

    fn end_policy(&mut self);
}
