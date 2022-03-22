// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `rsio.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct StepCallRequest {
    // message fields
    pub msgid: ::std::string::String,
    pub content: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StepCallRequest {
    fn default() -> &'a StepCallRequest {
        <StepCallRequest as ::protobuf::Message>::default_instance()
    }
}

impl StepCallRequest {
    pub fn new() -> StepCallRequest {
        ::std::default::Default::default()
    }

    // string msgid = 1;


    pub fn get_msgid(&self) -> &str {
        &self.msgid
    }
    pub fn clear_msgid(&mut self) {
        self.msgid.clear();
    }

    // Param is passed by value, moved
    pub fn set_msgid(&mut self, v: ::std::string::String) {
        self.msgid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msgid(&mut self) -> &mut ::std::string::String {
        &mut self.msgid
    }

    // Take field
    pub fn take_msgid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msgid, ::std::string::String::new())
    }

    // bytes content = 2;


    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for StepCallRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msgid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.msgid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.msgid);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.msgid.is_empty() {
            os.write_string(1, &self.msgid)?;
        }
        if !self.content.is_empty() {
            os.write_bytes(2, &self.content)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> StepCallRequest {
        StepCallRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msgid",
                |m: &StepCallRequest| { &m.msgid },
                |m: &mut StepCallRequest| { &mut m.msgid },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "content",
                |m: &StepCallRequest| { &m.content },
                |m: &mut StepCallRequest| { &mut m.content },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StepCallRequest>(
                "StepCallRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StepCallRequest {
        static instance: ::protobuf::rt::LazyV2<StepCallRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StepCallRequest::new)
    }
}

impl ::protobuf::Clear for StepCallRequest {
    fn clear(&mut self) {
        self.msgid.clear();
        self.content.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StepCallRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StepCallRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StepCallResponse {
    // message fields
    pub status: i32,
    pub message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StepCallResponse {
    fn default() -> &'a StepCallResponse {
        <StepCallResponse as ::protobuf::Message>::default_instance()
    }
}

impl StepCallResponse {
    pub fn new() -> StepCallResponse {
        ::std::default::Default::default()
    }

    // int32 status = 1;


    pub fn get_status(&self) -> i32 {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = 0;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: i32) {
        self.status = v;
    }

    // string message = 2;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for StepCallResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != 0 {
            my_size += ::protobuf::rt::value_size(1, self.status, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.status != 0 {
            os.write_int32(1, self.status)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> StepCallResponse {
        StepCallResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "status",
                |m: &StepCallResponse| { &m.status },
                |m: &mut StepCallResponse| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "message",
                |m: &StepCallResponse| { &m.message },
                |m: &mut StepCallResponse| { &mut m.message },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StepCallResponse>(
                "StepCallResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StepCallResponse {
        static instance: ::protobuf::rt::LazyV2<StepCallResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StepCallResponse::new)
    }
}

impl ::protobuf::Clear for StepCallResponse {
    fn clear(&mut self) {
        self.status = 0;
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StepCallResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StepCallResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nrsio.proto\x12\x04rsio\"A\n\x0fStepCallRequest\x12\x14\n\x05msgid\
    \x18\x01\x20\x01(\tR\x05msgid\x12\x18\n\x07content\x18\x02\x20\x01(\x0cR\
    \x07content\"D\n\x10StepCallResponse\x12\x16\n\x06status\x18\x01\x20\x01\
    (\x05R\x06status\x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message2J\n\
    \x0bNodeService\x12;\n\x08StepCall\x12\x15.rsio.StepCallRequest\x1a\x16.\
    rsio.StepCallResponse\"\0J\xcd\x03\n\x06\x12\x04\0\0\x0f\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0c\n\n\n\x02\x06\0\
    \x12\x04\x03\0\x05\x01\n\n\n\x03\x06\0\x01\x12\x03\x03\x08\x13\n\x0b\n\
    \x04\x06\0\x02\0\x12\x03\x04\x04?\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x04\x08\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x04\x11\x20\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03\x04+;\n\n\n\x02\x04\0\x12\x04\x07\0\n\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x07\x08\x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\
    \x02\x13\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x08\x02\x07\x19\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x08\t\x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08\x11\x12\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\t\x02\x14\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\t\
    \x02\x08\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\t\x02\x07\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\t\x08\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\t\x12\x13\n\n\n\x02\x04\x01\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\x01\
    \x01\x12\x03\x0c\x08\x18\n%\n\x04\x04\x01\x02\0\x12\x03\r\x04\x15\"\x18\
    \x200:\x20success\x20other:\x20fail\n\n\r\n\x05\x04\x01\x02\0\x04\x12\
    \x04\r\x04\x0c\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\x04\t\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\r\n\x10\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\r\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0e\x04\x17\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\x0e\x04\r\x15\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0e\x0b\
    \x12\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0e\x15\x16b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}