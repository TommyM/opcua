// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    string::UAString,
    service_types::enums::MessageSecurityMode,
    extension_object::ExtensionObject,
    service_types::EndpointDescription,
    service_types::KeyValuePair,
    service_types::DataSetWriterDataType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WriterGroupDataType {
    pub name: UAString,
    pub enabled: bool,
    pub security_mode: MessageSecurityMode,
    pub security_group_id: UAString,
    pub security_key_services: Option<Vec<EndpointDescription>>,
    pub max_network_message_size: u32,
    pub group_properties: Option<Vec<KeyValuePair>>,
    pub writer_group_id: u16,
    pub publishing_interval: f64,
    pub keep_alive_time: f64,
    pub priority: u8,
    pub locale_ids: Option<Vec<UAString>>,
    pub header_layout_uri: UAString,
    pub transport_settings: ExtensionObject,
    pub message_settings: ExtensionObject,
    pub data_set_writers: Option<Vec<DataSetWriterDataType>>,
}

impl BinaryEncoder<WriterGroupDataType> for WriterGroupDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_group_id.byte_len();
        size += byte_len_array(&self.security_key_services);
        size += self.max_network_message_size.byte_len();
        size += byte_len_array(&self.group_properties);
        size += self.writer_group_id.byte_len();
        size += self.publishing_interval.byte_len();
        size += self.keep_alive_time.byte_len();
        size += self.priority.byte_len();
        size += byte_len_array(&self.locale_ids);
        size += self.header_layout_uri.byte_len();
        size += self.transport_settings.byte_len();
        size += self.message_settings.byte_len();
        size += byte_len_array(&self.data_set_writers);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_group_id.encode(stream)?;
        size += write_array(stream, &self.security_key_services)?;
        size += self.max_network_message_size.encode(stream)?;
        size += write_array(stream, &self.group_properties)?;
        size += self.writer_group_id.encode(stream)?;
        size += self.publishing_interval.encode(stream)?;
        size += self.keep_alive_time.encode(stream)?;
        size += self.priority.encode(stream)?;
        size += write_array(stream, &self.locale_ids)?;
        size += self.header_layout_uri.encode(stream)?;
        size += self.transport_settings.encode(stream)?;
        size += self.message_settings.encode(stream)?;
        size += write_array(stream, &self.data_set_writers)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let name = UAString::decode(stream, decoding_limits)?;
        let enabled = bool::decode(stream, decoding_limits)?;
        let security_mode = MessageSecurityMode::decode(stream, decoding_limits)?;
        let security_group_id = UAString::decode(stream, decoding_limits)?;
        let security_key_services: Option<Vec<EndpointDescription>> = read_array(stream, decoding_limits)?;
        let max_network_message_size = u32::decode(stream, decoding_limits)?;
        let group_properties: Option<Vec<KeyValuePair>> = read_array(stream, decoding_limits)?;
        let writer_group_id = u16::decode(stream, decoding_limits)?;
        let publishing_interval = f64::decode(stream, decoding_limits)?;
        let keep_alive_time = f64::decode(stream, decoding_limits)?;
        let priority = u8::decode(stream, decoding_limits)?;
        let locale_ids: Option<Vec<UAString>> = read_array(stream, decoding_limits)?;
        let header_layout_uri = UAString::decode(stream, decoding_limits)?;
        let transport_settings = ExtensionObject::decode(stream, decoding_limits)?;
        let message_settings = ExtensionObject::decode(stream, decoding_limits)?;
        let data_set_writers: Option<Vec<DataSetWriterDataType>> = read_array(stream, decoding_limits)?;
        Ok(WriterGroupDataType {
            name,
            enabled,
            security_mode,
            security_group_id,
            security_key_services,
            max_network_message_size,
            group_properties,
            writer_group_id,
            publishing_interval,
            keep_alive_time,
            priority,
            locale_ids,
            header_layout_uri,
            transport_settings,
            message_settings,
            data_set_writers,
        })
    }
}
