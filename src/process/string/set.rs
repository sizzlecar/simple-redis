use crate::{process::Parameter, Encoder, Processor};

#[derive(Debug)]
#[allow(unused)]
pub struct SetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    para: Parameter,
}

impl SetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for SetCommandPara {
    fn process(self) -> Result<Box<dyn Encoder>, anyhow::Error> {
        todo!()
    }
}
