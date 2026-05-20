use heapless::spsc::Producer;

use crate::action::Action;
use crate::report::Report;

pub trait HandleAction {
    fn handle(&mut self, action: Action) -> Result<(), Report>;
}

pub struct ActionHandler<'b> {
    layer: usize,
    producer: Producer<'b, Report>,
}

impl<'b> ActionHandler<'b> {
    pub fn new(producer: Producer<Report>) -> ActionHandler {
        ActionHandler { layer: 0, producer }
    }
}

impl<'b> HandleAction for ActionHandler<'b> {
    fn handle(&mut self, action: Action) -> Result<(), Report> {
        match action {
            Action::Report(report) => self.producer.enqueue(report)?,
            Action::Layer(layer) => self.layer = layer as usize,
        }
        Ok(())
    }
}

impl<'b> ActionHandler<'b> {
    pub fn get_layer(&self) -> usize {
        self.layer
    }
}
