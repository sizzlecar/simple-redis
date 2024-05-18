use crate::{process::Parameter, Encoder, Processor};

#[derive(Debug)]
#[allow(unused)]
pub struct GetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    para: Parameter,
}

impl GetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for GetCommandPara {
    fn process(self) -> Result<Box<dyn Encoder>, anyhow::Error> {
        todo!()
    }
}
