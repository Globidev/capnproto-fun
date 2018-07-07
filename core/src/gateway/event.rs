extern crate serde;
extern crate serde_json;
extern crate serde_value;

use self::serde::{de, Serialize};
use self::serde_value::Value as ASTValue;
use self::serde_json::{
    from_str as decode,
    to_string as encode,
};

use std::fmt;

pub mod op {
    #[derive(Debug, Default, Serialize)]
    pub struct Identify {
        pub token: String,
        pub properties: IdentifyProperties,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub compress: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub large_threshold: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub shard: Option<(u32, u32)>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub presence: Option<StatusUpdate>
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct Hello {
        pub heartbeat_interval: u64,

        #[serde(rename = "_trace")]
        trace: Vec<String>
    }


    #[derive(Debug, Deserialize)]
    pub enum DispatchEvent {
        SomeEvent
    }



    #[derive(Debug, Default, Serialize)]
    pub struct IdentifyProperties {
        #[serde(rename = "$os")]
        pub os: String,

        #[serde(rename = "$browser")]
        pub browser: String,

        #[serde(rename = "$device")]
        pub device: String
    }

    #[derive(Debug, Default, Serialize)]
    pub struct StatusUpdate {
        pub since: Option<u64>,
        pub game: Option<Activity>,
        pub status: String,
        pub afk: bool
    }

    #[derive(Debug, Default, Serialize)]
    pub struct Activity {

    }
}

#[derive(Debug)]
pub enum Event {
    SomeEvent
}

#[derive(Debug)]
pub enum MessageIn {
    Dispatch(Event),
    Hello(op::Hello),
    HeartbeatAck,
}

#[derive(Debug)]
pub enum MessageOut {
    Identify(op::Identify),
    Heartbeat(Option<u32>),
}

pub fn from_raw_payload(raw_data: &str) -> Result<MessageIn, PayloadError> {
    decode(raw_data)
        .map_err(PayloadError::JsonError)
}

pub fn to_raw_payload(event: MessageOut) -> Result<String, PayloadError> {
    use self::MessageOut::*;

    match event {
        Heartbeat(heartbeat) => encode_event(1, heartbeat),
        Identify(identify)   => encode_event(2, identify),
    }
}

fn encode_event(op_code: u32, data: impl Serialize)
    -> Result<String, PayloadError>
{
    encode(&PayloadOut { op_code, data })
        .map_err(PayloadError::JsonError)
}

#[derive(Debug, Serialize)]
pub struct PayloadOut<T: Serialize> {
    #[serde(rename = "op")]
    op_code: u32,

    #[serde(rename = "d")]
    data: T,
}

impl<'de> de::Deserialize<'de> for MessageIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(PayloadVisitor)
    }
}

struct PayloadVisitor;

impl<'de> de::Visitor<'de> for PayloadVisitor {
    type Value = MessageIn;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Well formed payload")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        use self::PayloadField::*;

        let mut builder = PayloadBuilder::new();

        while let Some(key) = map.next_key()? {
            builder = match key {
                OpCode         => builder.with_op_code(map.next_value()?),
                Data           => builder.with_data(map.next_value()?),
                SequenceNumber => builder.with_sequence_number(map.next_value()?),
                EventName      => builder.with_event_name(map.next_value()?)
            }
        }

        builder.finish()
            .map_err(de::Error::custom)
    }
}

#[derive(Default)]
struct PayloadBuilder {
    op_code: Option<u32>,
    data: Option<ASTValue>,
    sequence_number: Option<u32>,
    event_name: Option<String>
}

impl PayloadBuilder {
    fn new() -> Self { Self::default() }

    fn with_op_code(self, op_code: u32) -> Self {
        Self { op_code: Some(op_code), ..self }
    }

    fn with_data(self, data: ASTValue) -> Self {
        Self { data: Some(data), ..self }
    }

    fn with_sequence_number(self, sequence_number: Option<u32>) -> Self {
        Self { sequence_number: sequence_number, ..self }
    }

    fn with_event_name(self, event_name: Option<String>) -> Self {
        Self { event_name: event_name, ..self }
    }

    fn finish(self) -> Result<MessageIn, PayloadError> {
        use self::MessageIn::*;
        use self::PayloadError::*;
        use self::PayloadField::*;

        let op_code = self.op_code
            .ok_or_else(|| MissingField(OpCode))?;
        let data = self.data
            .ok_or_else(|| MissingField(Data))?;

        match op_code {
            0 => {
                let sequence_number = self.sequence_number
                    .ok_or_else(|| MissingField(SequenceNumber))?;
                let event_name = self.event_name
                    .ok_or_else(|| MissingField(EventName))?;
                // TODO: de event
                Ok(Dispatch(Event::SomeEvent))
            },

            10 => Ok(Hello(data.deserialize_into().map_err(DataFormatError)?)),
            11 => Ok(HeartbeatAck),

            unknown => Err(Unimplemented { op_code: unknown, data })
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum PayloadField {
    #[serde(rename = "op")]
    OpCode,

    #[serde(rename = "d")]
    Data,

    #[serde(rename = "s")]
    SequenceNumber,

    #[serde(rename = "t")]
    EventName
}

#[derive(Debug)]
pub enum PayloadError {
    JsonError(self::serde_json::Error),
    DataFormatError(self::serde_value::DeserializerError),
    MissingField(PayloadField),
    Unimplemented { op_code: u32, data: ASTValue },
}

impl fmt::Display for PayloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn op10() {
        let d = "{\"t\":null,\"s\":null,\"op\":10,\"d\":{\"heartbeat_interval\":41250,\"_trace\":[\"gateway-prd-main-8v6p\"]}}";

        println!("{:?}", super::from_raw_payload(d).unwrap());
    }

    #[test]
    fn op11() {
        let d = "{\"op\":11,\"d\":null}";

        println!("{:?}", super::from_raw_payload(d).unwrap());
    }
}
