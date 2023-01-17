use prost::EncodeError;

pub trait MessageExt: prost::Message {
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}
impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}
