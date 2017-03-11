// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct DatHeader {
    // message fields
    services: ::std::option::Option<u64>,
    service_config: ::std::option::Option<u64>,
    service_files: ::std::option::Option<u64>,
    elections: ::std::option::Option<u64>,
    updates: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatHeader {}

impl DatHeader {
    pub fn new() -> DatHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatHeader {
        static mut instance: ::protobuf::lazy::Lazy<DatHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatHeader,
        };
        unsafe {
            instance.get(DatHeader::new)
        }
    }

    // optional uint64 services = 1;

    pub fn clear_services(&mut self) {
        self.services = ::std::option::Option::None;
    }

    pub fn has_services(&self) -> bool {
        self.services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_services(&mut self, v: u64) {
        self.services = ::std::option::Option::Some(v);
    }

    pub fn get_services(&self) -> u64 {
        self.services.unwrap_or(0)
    }

    fn get_services_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.services
    }

    fn mut_services_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.services
    }

    // optional uint64 service_config = 2;

    pub fn clear_service_config(&mut self) {
        self.service_config = ::std::option::Option::None;
    }

    pub fn has_service_config(&self) -> bool {
        self.service_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_config(&mut self, v: u64) {
        self.service_config = ::std::option::Option::Some(v);
    }

    pub fn get_service_config(&self) -> u64 {
        self.service_config.unwrap_or(0)
    }

    fn get_service_config_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.service_config
    }

    fn mut_service_config_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.service_config
    }

    // optional uint64 service_files = 3;

    pub fn clear_service_files(&mut self) {
        self.service_files = ::std::option::Option::None;
    }

    pub fn has_service_files(&self) -> bool {
        self.service_files.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_files(&mut self, v: u64) {
        self.service_files = ::std::option::Option::Some(v);
    }

    pub fn get_service_files(&self) -> u64 {
        self.service_files.unwrap_or(0)
    }

    fn get_service_files_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.service_files
    }

    fn mut_service_files_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.service_files
    }

    // optional uint64 elections = 4;

    pub fn clear_elections(&mut self) {
        self.elections = ::std::option::Option::None;
    }

    pub fn has_elections(&self) -> bool {
        self.elections.is_some()
    }

    // Param is passed by value, moved
    pub fn set_elections(&mut self, v: u64) {
        self.elections = ::std::option::Option::Some(v);
    }

    pub fn get_elections(&self) -> u64 {
        self.elections.unwrap_or(0)
    }

    fn get_elections_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.elections
    }

    fn mut_elections_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.elections
    }

    // optional uint64 updates = 5;

    pub fn clear_updates(&mut self) {
        self.updates = ::std::option::Option::None;
    }

    pub fn has_updates(&self) -> bool {
        self.updates.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: u64) {
        self.updates = ::std::option::Option::Some(v);
    }

    pub fn get_updates(&self) -> u64 {
        self.updates.unwrap_or(0)
    }

    fn get_updates_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.updates
    }

    fn mut_updates_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.updates
    }
}

impl ::protobuf::Message for DatHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.services = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.service_config = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.service_files = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.elections = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.updates = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.services {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.service_config {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.service_files {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.elections {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.updates {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.services {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.service_config {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.service_files {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.elections {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.updates {
            os.write_uint64(5, v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatHeader {
    fn new() -> DatHeader {
        DatHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "services",
                    DatHeader::get_services_for_reflect,
                    DatHeader::mut_services_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "service_config",
                    DatHeader::get_service_config_for_reflect,
                    DatHeader::mut_service_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "service_files",
                    DatHeader::get_service_files_for_reflect,
                    DatHeader::mut_service_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "elections",
                    DatHeader::get_elections_for_reflect,
                    DatHeader::mut_elections_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "updates",
                    DatHeader::get_updates_for_reflect,
                    DatHeader::mut_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatHeader>(
                    "DatHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatHeader {
    fn clear(&mut self) {
        self.clear_services();
        self.clear_service_config();
        self.clear_service_files();
        self.clear_elections();
        self.clear_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x64, 0x61, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xab, 0x01, 0x0a, 0x09, 0x44, 0x61, 0x74, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x12,
    0x25, 0x0a, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x73, 0x4a, 0x83, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x02, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x03, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x03, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x05, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x06, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x06, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x07, 0x1c, 0x1d,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
