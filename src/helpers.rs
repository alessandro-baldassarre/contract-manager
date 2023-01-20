use cosmwasm_std::{Event, Reply, StdError, StdResult};
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

/// Returns the value from attribute specified
pub fn value_from_attr_key(msg: Reply, key: &str) -> StdResult<String> {
    let value = event_from_attr(msg.clone(), key)?
        .attributes
        .iter()
        .find(|&attr| attr.key == key)
        .cloned()
        .ok_or_else(|| StdError::generic_err(format!("unable to find attribute: {} ", key)))?
        .value;
    Ok(value)
}

// Determine if a Reply message contain an event with attribute from specified key-value pair and
// returns it
fn event_from_attr(msg: Reply, key: &str) -> StdResult<Event> {
    let event = msg
        .result
        .into_result()
        .map_err(StdError::generic_err)?
        .events
        .iter()
        .find(|&e| event_contains_attr(e, key))
        .cloned()
        .ok_or_else(|| {
            StdError::generic_err(format!("unable to find event with attribute {}  ", key))
        })?;

    Ok(event)
}

fn event_contains_attr(event: &Event, key: &str) -> bool {
    event.attributes.iter().any(|attr| attr.key == key)
}
