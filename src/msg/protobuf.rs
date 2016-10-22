// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Identify {
    // message fields
    server_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Identify {}

impl Identify {
    pub fn new() -> Identify {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Identify {
        static mut instance: ::protobuf::lazy::Lazy<Identify> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Identify,
        };
        unsafe {
            instance.get(|| {
                Identify {
                    server_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 server_id = 1;

    pub fn clear_server_id(&mut self) {
        self.server_id = ::std::option::Option::None;
    }

    pub fn has_server_id(&self) -> bool {
        self.server_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_id(&mut self, v: u64) {
        self.server_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_id(&self) -> u64 {
        self.server_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for Identify {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.server_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.server_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_id {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Identify>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Identify {
    fn new() -> Identify {
        Identify::new()
    }

    fn descriptor_static(_: ::std::option::Option<Identify>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "server_id",
                    Identify::has_server_id,
                    Identify::get_server_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Identify>(
                    "Identify",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Identify {
    fn clear(&mut self) {
        self.clear_server_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Identify {
    fn eq(&self, other: &Identify) -> bool {
        self.server_id == other.server_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Identify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RPCRequest {
    // message fields
    field_type: ::std::option::Option<ExchangeType>,
    server_id: ::std::option::Option<u64>,
    request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPCRequest {}

impl RPCRequest {
    pub fn new() -> RPCRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPCRequest {
        static mut instance: ::protobuf::lazy::Lazy<RPCRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPCRequest,
        };
        unsafe {
            instance.get(|| {
                RPCRequest {
                    field_type: ::std::option::Option::None,
                    server_id: ::std::option::Option::None,
                    request: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .RaftRPC.ExchangeType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ExchangeType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ExchangeType {
        self.field_type.unwrap_or(ExchangeType::APPENDENTRIES)
    }

    // optional uint64 server_id = 2;

    pub fn clear_server_id(&mut self) {
        self.server_id = ::std::option::Option::None;
    }

    pub fn has_server_id(&self) -> bool {
        self.server_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_id(&mut self, v: u64) {
        self.server_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_id(&self) -> u64 {
        self.server_id.unwrap_or(0)
    }

    // optional bytes request = 3;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> ::std::vec::Vec<u8> {
        self.request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request(&self) -> &[u8] {
        match self.request.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RPCRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.server_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.request));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.server_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.server_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.request.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RPCRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RPCRequest {
    fn new() -> RPCRequest {
        RPCRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPCRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    RPCRequest::has_field_type,
                    RPCRequest::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "server_id",
                    RPCRequest::has_server_id,
                    RPCRequest::get_server_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "request",
                    RPCRequest::has_request,
                    RPCRequest::get_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPCRequest>(
                    "RPCRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPCRequest {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_server_id();
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RPCRequest {
    fn eq(&self, other: &RPCRequest) -> bool {
        self.field_type == other.field_type &&
        self.server_id == other.server_id &&
        self.request == other.request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RPCRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RPCReply {
    // message fields
    field_type: ::std::option::Option<ExchangeType>,
    server_id: ::std::option::Option<u64>,
    reply: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPCReply {}

impl RPCReply {
    pub fn new() -> RPCReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPCReply {
        static mut instance: ::protobuf::lazy::Lazy<RPCReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPCReply,
        };
        unsafe {
            instance.get(|| {
                RPCReply {
                    field_type: ::std::option::Option::None,
                    server_id: ::std::option::Option::None,
                    reply: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .RaftRPC.ExchangeType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ExchangeType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ExchangeType {
        self.field_type.unwrap_or(ExchangeType::APPENDENTRIES)
    }

    // optional uint64 server_id = 2;

    pub fn clear_server_id(&mut self) {
        self.server_id = ::std::option::Option::None;
    }

    pub fn has_server_id(&self) -> bool {
        self.server_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_id(&mut self, v: u64) {
        self.server_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_id(&self) -> u64 {
        self.server_id.unwrap_or(0)
    }

    // optional bytes reply = 3;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    pub fn has_reply(&self) -> bool {
        self.reply.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::std::vec::Vec<u8>) {
        self.reply = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reply(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.reply.is_none() {
            self.reply.set_default();
        };
        self.reply.as_mut().unwrap()
    }

    // Take field
    pub fn take_reply(&mut self) -> ::std::vec::Vec<u8> {
        self.reply.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_reply(&self) -> &[u8] {
        match self.reply.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RPCReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.server_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.reply));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.server_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.reply {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.server_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.reply.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RPCReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RPCReply {
    fn new() -> RPCReply {
        RPCReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPCReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    RPCReply::has_field_type,
                    RPCReply::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "server_id",
                    RPCReply::has_server_id,
                    RPCReply::get_server_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "reply",
                    RPCReply::has_reply,
                    RPCReply::get_reply,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPCReply>(
                    "RPCReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPCReply {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_server_id();
        self.clear_reply();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RPCReply {
    fn eq(&self, other: &RPCReply) -> bool {
        self.field_type == other.field_type &&
        self.server_id == other.server_id &&
        self.reply == other.reply &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RPCReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LogEntry {
    // message fields
    index: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogEntry {}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry,
        };
        unsafe {
            instance.get(|| {
                LogEntry {
                    index: ::std::option::Option::None,
                    term: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    // optional uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional string key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes value = 4;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for LogEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.term {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogEntry {
    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "index",
                    LogEntry::has_index,
                    LogEntry::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    LogEntry::has_term,
                    LogEntry::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    LogEntry::has_key,
                    LogEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    LogEntry::has_value,
                    LogEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry>(
                    "LogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_term();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.index == other.index &&
        self.term == other.term &&
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendEntriesRequest {
    // message fields
    term: ::std::option::Option<u64>,
    leaderId: ::std::option::Option<u64>,
    prevLogIndex: ::std::option::Option<u64>,
    prevLogTerm: ::std::option::Option<u64>,
    entry: ::protobuf::RepeatedField<LogEntry>,
    leaderCommit: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesRequest {}

impl AppendEntriesRequest {
    pub fn new() -> AppendEntriesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesRequest {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesRequest,
        };
        unsafe {
            instance.get(|| {
                AppendEntriesRequest {
                    term: ::std::option::Option::None,
                    leaderId: ::std::option::Option::None,
                    prevLogIndex: ::std::option::Option::None,
                    prevLogTerm: ::std::option::Option::None,
                    entry: ::protobuf::RepeatedField::new(),
                    leaderCommit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional uint64 leaderId = 2;

    pub fn clear_leaderId(&mut self) {
        self.leaderId = ::std::option::Option::None;
    }

    pub fn has_leaderId(&self) -> bool {
        self.leaderId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaderId(&mut self, v: u64) {
        self.leaderId = ::std::option::Option::Some(v);
    }

    pub fn get_leaderId(&self) -> u64 {
        self.leaderId.unwrap_or(0)
    }

    // optional uint64 prevLogIndex = 3;

    pub fn clear_prevLogIndex(&mut self) {
        self.prevLogIndex = ::std::option::Option::None;
    }

    pub fn has_prevLogIndex(&self) -> bool {
        self.prevLogIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prevLogIndex(&mut self, v: u64) {
        self.prevLogIndex = ::std::option::Option::Some(v);
    }

    pub fn get_prevLogIndex(&self) -> u64 {
        self.prevLogIndex.unwrap_or(0)
    }

    // optional uint64 prevLogTerm = 4;

    pub fn clear_prevLogTerm(&mut self) {
        self.prevLogTerm = ::std::option::Option::None;
    }

    pub fn has_prevLogTerm(&self) -> bool {
        self.prevLogTerm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prevLogTerm(&mut self, v: u64) {
        self.prevLogTerm = ::std::option::Option::Some(v);
    }

    pub fn get_prevLogTerm(&self) -> u64 {
        self.prevLogTerm.unwrap_or(0)
    }

    // repeated .RaftRPC.LogEntry entry = 5;

    pub fn clear_entry(&mut self) {
        self.entry.clear();
    }

    // Param is passed by value, moved
    pub fn set_entry(&mut self, v: ::protobuf::RepeatedField<LogEntry>) {
        self.entry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entry(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.entry
    }

    // Take field
    pub fn take_entry(&mut self) -> ::protobuf::RepeatedField<LogEntry> {
        ::std::mem::replace(&mut self.entry, ::protobuf::RepeatedField::new())
    }

    pub fn get_entry(&self) -> &[LogEntry] {
        &self.entry
    }

    // optional uint64 leaderCommit = 6;

    pub fn clear_leaderCommit(&mut self) {
        self.leaderCommit = ::std::option::Option::None;
    }

    pub fn has_leaderCommit(&self) -> bool {
        self.leaderCommit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaderCommit(&mut self, v: u64) {
        self.leaderCommit = ::std::option::Option::Some(v);
    }

    pub fn get_leaderCommit(&self) -> u64 {
        self.leaderCommit.unwrap_or(0)
    }
}

impl ::protobuf::Message for AppendEntriesRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.leaderId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.prevLogIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.prevLogTerm = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entry));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.leaderCommit = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.leaderId {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prevLogIndex {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prevLogTerm {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.leaderCommit {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.leaderId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.prevLogIndex {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.prevLogTerm {
            try!(os.write_uint64(4, v));
        };
        for v in &self.entry {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.leaderCommit {
            try!(os.write_uint64(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AppendEntriesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntriesRequest {
    fn new() -> AppendEntriesRequest {
        AppendEntriesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    AppendEntriesRequest::has_term,
                    AppendEntriesRequest::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "leaderId",
                    AppendEntriesRequest::has_leaderId,
                    AppendEntriesRequest::get_leaderId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "prevLogIndex",
                    AppendEntriesRequest::has_prevLogIndex,
                    AppendEntriesRequest::get_prevLogIndex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "prevLogTerm",
                    AppendEntriesRequest::has_prevLogTerm,
                    AppendEntriesRequest::get_prevLogTerm,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entry",
                    AppendEntriesRequest::get_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "leaderCommit",
                    AppendEntriesRequest::has_leaderCommit,
                    AppendEntriesRequest::get_leaderCommit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesRequest>(
                    "AppendEntriesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_leaderId();
        self.clear_prevLogIndex();
        self.clear_prevLogTerm();
        self.clear_entry();
        self.clear_leaderCommit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendEntriesRequest {
    fn eq(&self, other: &AppendEntriesRequest) -> bool {
        self.term == other.term &&
        self.leaderId == other.leaderId &&
        self.prevLogIndex == other.prevLogIndex &&
        self.prevLogTerm == other.prevLogTerm &&
        self.entry == other.entry &&
        self.leaderCommit == other.leaderCommit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendEntriesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendEntriesReply {
    // message fields
    term: ::std::option::Option<u64>,
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesReply {}

impl AppendEntriesReply {
    pub fn new() -> AppendEntriesReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesReply {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesReply,
        };
        unsafe {
            instance.get(|| {
                AppendEntriesReply {
                    term: ::std::option::Option::None,
                    success: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional bool success = 2;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }
}

impl ::protobuf::Message for AppendEntriesReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.success.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.success {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AppendEntriesReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntriesReply {
    fn new() -> AppendEntriesReply {
        AppendEntriesReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    AppendEntriesReply::has_term,
                    AppendEntriesReply::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    AppendEntriesReply::has_success,
                    AppendEntriesReply::get_success,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesReply>(
                    "AppendEntriesReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesReply {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendEntriesReply {
    fn eq(&self, other: &AppendEntriesReply) -> bool {
        self.term == other.term &&
        self.success == other.success &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendEntriesReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestVoteRequest {
    // message fields
    term: ::std::option::Option<u64>,
    candidateId: ::std::option::Option<u64>,
    lastLogIndex: ::std::option::Option<u64>,
    lastLogTerm: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteRequest {}

impl RequestVoteRequest {
    pub fn new() -> RequestVoteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteRequest {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteRequest,
        };
        unsafe {
            instance.get(|| {
                RequestVoteRequest {
                    term: ::std::option::Option::None,
                    candidateId: ::std::option::Option::None,
                    lastLogIndex: ::std::option::Option::None,
                    lastLogTerm: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional uint64 candidateId = 2;

    pub fn clear_candidateId(&mut self) {
        self.candidateId = ::std::option::Option::None;
    }

    pub fn has_candidateId(&self) -> bool {
        self.candidateId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candidateId(&mut self, v: u64) {
        self.candidateId = ::std::option::Option::Some(v);
    }

    pub fn get_candidateId(&self) -> u64 {
        self.candidateId.unwrap_or(0)
    }

    // optional uint64 lastLogIndex = 3;

    pub fn clear_lastLogIndex(&mut self) {
        self.lastLogIndex = ::std::option::Option::None;
    }

    pub fn has_lastLogIndex(&self) -> bool {
        self.lastLogIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogIndex(&mut self, v: u64) {
        self.lastLogIndex = ::std::option::Option::Some(v);
    }

    pub fn get_lastLogIndex(&self) -> u64 {
        self.lastLogIndex.unwrap_or(0)
    }

    // optional uint64 lastLogTerm = 4;

    pub fn clear_lastLogTerm(&mut self) {
        self.lastLogTerm = ::std::option::Option::None;
    }

    pub fn has_lastLogTerm(&self) -> bool {
        self.lastLogTerm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogTerm(&mut self, v: u64) {
        self.lastLogTerm = ::std::option::Option::Some(v);
    }

    pub fn get_lastLogTerm(&self) -> u64 {
        self.lastLogTerm.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestVoteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.candidateId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastLogIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastLogTerm = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.candidateId {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lastLogIndex {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lastLogTerm {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.candidateId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.lastLogIndex {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.lastLogTerm {
            try!(os.write_uint64(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RequestVoteRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestVoteRequest {
    fn new() -> RequestVoteRequest {
        RequestVoteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    RequestVoteRequest::has_term,
                    RequestVoteRequest::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "candidateId",
                    RequestVoteRequest::has_candidateId,
                    RequestVoteRequest::get_candidateId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastLogIndex",
                    RequestVoteRequest::has_lastLogIndex,
                    RequestVoteRequest::get_lastLogIndex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastLogTerm",
                    RequestVoteRequest::has_lastLogTerm,
                    RequestVoteRequest::get_lastLogTerm,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteRequest>(
                    "RequestVoteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_candidateId();
        self.clear_lastLogIndex();
        self.clear_lastLogTerm();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestVoteRequest {
    fn eq(&self, other: &RequestVoteRequest) -> bool {
        self.term == other.term &&
        self.candidateId == other.candidateId &&
        self.lastLogIndex == other.lastLogIndex &&
        self.lastLogTerm == other.lastLogTerm &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestVoteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestVoteReply {
    // message fields
    term: ::std::option::Option<u64>,
    voteGranted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteReply {}

impl RequestVoteReply {
    pub fn new() -> RequestVoteReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteReply {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteReply,
        };
        unsafe {
            instance.get(|| {
                RequestVoteReply {
                    term: ::std::option::Option::None,
                    voteGranted: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional bool voteGranted = 2;

    pub fn clear_voteGranted(&mut self) {
        self.voteGranted = ::std::option::Option::None;
    }

    pub fn has_voteGranted(&self) -> bool {
        self.voteGranted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voteGranted(&mut self, v: bool) {
        self.voteGranted = ::std::option::Option::Some(v);
    }

    pub fn get_voteGranted(&self) -> bool {
        self.voteGranted.unwrap_or(false)
    }
}

impl ::protobuf::Message for RequestVoteReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.voteGranted = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.voteGranted.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.voteGranted {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RequestVoteReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestVoteReply {
    fn new() -> RequestVoteReply {
        RequestVoteReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    RequestVoteReply::has_term,
                    RequestVoteReply::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "voteGranted",
                    RequestVoteReply::has_voteGranted,
                    RequestVoteReply::get_voteGranted,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteReply>(
                    "RequestVoteReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteReply {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_voteGranted();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestVoteReply {
    fn eq(&self, other: &RequestVoteReply) -> bool {
        self.term == other.term &&
        self.voteGranted == other.voteGranted &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestVoteReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InstallSnapshotRequest {
    // message fields
    term: ::std::option::Option<u64>,
    leaderId: ::std::option::Option<u64>,
    lastIncludedIndex: ::std::option::Option<u64>,
    lastIncludedTerm: ::std::option::Option<u64>,
    offset: ::std::option::Option<u64>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InstallSnapshotRequest {}

impl InstallSnapshotRequest {
    pub fn new() -> InstallSnapshotRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InstallSnapshotRequest {
        static mut instance: ::protobuf::lazy::Lazy<InstallSnapshotRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InstallSnapshotRequest,
        };
        unsafe {
            instance.get(|| {
                InstallSnapshotRequest {
                    term: ::std::option::Option::None,
                    leaderId: ::std::option::Option::None,
                    lastIncludedIndex: ::std::option::Option::None,
                    lastIncludedTerm: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional uint64 leaderId = 2;

    pub fn clear_leaderId(&mut self) {
        self.leaderId = ::std::option::Option::None;
    }

    pub fn has_leaderId(&self) -> bool {
        self.leaderId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaderId(&mut self, v: u64) {
        self.leaderId = ::std::option::Option::Some(v);
    }

    pub fn get_leaderId(&self) -> u64 {
        self.leaderId.unwrap_or(0)
    }

    // optional uint64 lastIncludedIndex = 3;

    pub fn clear_lastIncludedIndex(&mut self) {
        self.lastIncludedIndex = ::std::option::Option::None;
    }

    pub fn has_lastIncludedIndex(&self) -> bool {
        self.lastIncludedIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastIncludedIndex(&mut self, v: u64) {
        self.lastIncludedIndex = ::std::option::Option::Some(v);
    }

    pub fn get_lastIncludedIndex(&self) -> u64 {
        self.lastIncludedIndex.unwrap_or(0)
    }

    // optional uint64 lastIncludedTerm = 4;

    pub fn clear_lastIncludedTerm(&mut self) {
        self.lastIncludedTerm = ::std::option::Option::None;
    }

    pub fn has_lastIncludedTerm(&self) -> bool {
        self.lastIncludedTerm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastIncludedTerm(&mut self, v: u64) {
        self.lastIncludedTerm = ::std::option::Option::Some(v);
    }

    pub fn get_lastIncludedTerm(&self) -> u64 {
        self.lastIncludedTerm.unwrap_or(0)
    }

    // optional uint64 offset = 5;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    // optional bytes data = 6;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool done = 7;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for InstallSnapshotRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.leaderId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastIncludedIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastIncludedTerm = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.offset = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.leaderId {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lastIncludedIndex {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lastIncludedTerm {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.offset {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.leaderId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.lastIncludedIndex {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.lastIncludedTerm {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.offset {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.done {
            try!(os.write_bool(7, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<InstallSnapshotRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InstallSnapshotRequest {
    fn new() -> InstallSnapshotRequest {
        InstallSnapshotRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InstallSnapshotRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    InstallSnapshotRequest::has_term,
                    InstallSnapshotRequest::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "leaderId",
                    InstallSnapshotRequest::has_leaderId,
                    InstallSnapshotRequest::get_leaderId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastIncludedIndex",
                    InstallSnapshotRequest::has_lastIncludedIndex,
                    InstallSnapshotRequest::get_lastIncludedIndex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastIncludedTerm",
                    InstallSnapshotRequest::has_lastIncludedTerm,
                    InstallSnapshotRequest::get_lastIncludedTerm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "offset",
                    InstallSnapshotRequest::has_offset,
                    InstallSnapshotRequest::get_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    InstallSnapshotRequest::has_data,
                    InstallSnapshotRequest::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    InstallSnapshotRequest::has_done,
                    InstallSnapshotRequest::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InstallSnapshotRequest>(
                    "InstallSnapshotRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InstallSnapshotRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_leaderId();
        self.clear_lastIncludedIndex();
        self.clear_lastIncludedTerm();
        self.clear_offset();
        self.clear_data();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InstallSnapshotRequest {
    fn eq(&self, other: &InstallSnapshotRequest) -> bool {
        self.term == other.term &&
        self.leaderId == other.leaderId &&
        self.lastIncludedIndex == other.lastIncludedIndex &&
        self.lastIncludedTerm == other.lastIncludedTerm &&
        self.offset == other.offset &&
        self.data == other.data &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InstallSnapshotRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InstallSnapshotReply {
    // message fields
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InstallSnapshotReply {}

impl InstallSnapshotReply {
    pub fn new() -> InstallSnapshotReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InstallSnapshotReply {
        static mut instance: ::protobuf::lazy::Lazy<InstallSnapshotReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InstallSnapshotReply,
        };
        unsafe {
            instance.get(|| {
                InstallSnapshotReply {
                    term: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }
}

impl ::protobuf::Message for InstallSnapshotReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<InstallSnapshotReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InstallSnapshotReply {
    fn new() -> InstallSnapshotReply {
        InstallSnapshotReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<InstallSnapshotReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    InstallSnapshotReply::has_term,
                    InstallSnapshotReply::get_term,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InstallSnapshotReply>(
                    "InstallSnapshotReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InstallSnapshotReply {
    fn clear(&mut self) {
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InstallSnapshotReply {
    fn eq(&self, other: &InstallSnapshotReply) -> bool {
        self.term == other.term &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InstallSnapshotReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExchangeType {
    APPENDENTRIES = 0,
    REQUESTVOTE = 1,
    INSTALLSNAPSHOT = 2,
}

impl ::protobuf::ProtobufEnum for ExchangeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExchangeType> {
        match value {
            0 => ::std::option::Option::Some(ExchangeType::APPENDENTRIES),
            1 => ::std::option::Option::Some(ExchangeType::REQUESTVOTE),
            2 => ::std::option::Option::Some(ExchangeType::INSTALLSNAPSHOT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExchangeType] = &[
            ExchangeType::APPENDENTRIES,
            ExchangeType::REQUESTVOTE,
            ExchangeType::INSTALLSNAPSHOT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ExchangeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExchangeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExchangeType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x65, 0x74, 0x63, 0x2f, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x07, 0x52, 0x61, 0x66, 0x74, 0x52, 0x50, 0x43, 0x22, 0x27, 0x0a, 0x08, 0x49, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x66, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x49, 0x64, 0x22, 0x6e, 0x0a, 0x0a, 0x52, 0x50, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x29, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15,
    0x2e, 0x52, 0x61, 0x66, 0x74, 0x52, 0x50, 0x43, 0x2e, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x22, 0x68, 0x0a, 0x08, 0x52, 0x50, 0x43, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x29,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x52,
    0x61, 0x66, 0x74, 0x52, 0x50, 0x43, 0x2e, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x5c, 0x0a, 0x08,
    0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12,
    0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x74, 0x65,
    0x72, 0x6d, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xd9, 0x01, 0x0a, 0x14, 0x41,
    0x70, 0x70, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x49, 0x64, 0x12, 0x22, 0x0a, 0x0c, 0x70, 0x72, 0x65, 0x76, 0x4c, 0x6f, 0x67, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x70, 0x72, 0x65, 0x76, 0x4c,
    0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x20, 0x0a, 0x0b, 0x70, 0x72, 0x65, 0x76, 0x4c,
    0x6f, 0x67, 0x54, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x70, 0x72,
    0x65, 0x76, 0x4c, 0x6f, 0x67, 0x54, 0x65, 0x72, 0x6d, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x6e, 0x74,
    0x72, 0x79, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x52,
    0x50, 0x43, 0x2e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x05, 0x65, 0x6e, 0x74,
    0x72, 0x79, 0x12, 0x22, 0x0a, 0x0c, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x22, 0x42, 0x0a, 0x12, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64,
    0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x12, 0x0a, 0x04,
    0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x74, 0x65, 0x72, 0x6d,
    0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0x90, 0x01, 0x0a, 0x12, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x04, 0x74, 0x65, 0x72, 0x6d, 0x12, 0x20, 0x0a, 0x0b, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x63, 0x61, 0x6e, 0x64,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x49, 0x64, 0x12, 0x22, 0x0a, 0x0c, 0x6c, 0x61, 0x73, 0x74, 0x4c,
    0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x6c,
    0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x20, 0x0a, 0x0b, 0x6c,
    0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x54, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0b, 0x6c, 0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x54, 0x65, 0x72, 0x6d, 0x22, 0x48, 0x0a,
    0x10, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x04, 0x74, 0x65, 0x72, 0x6d, 0x12, 0x20, 0x0a, 0x0b, 0x76, 0x6f, 0x74, 0x65, 0x47, 0x72, 0x61,
    0x6e, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x76, 0x6f, 0x74, 0x65,
    0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x22, 0xe2, 0x01, 0x0a, 0x16, 0x49, 0x6e, 0x73, 0x74,
    0x61, 0x6c, 0x6c, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x2c, 0x0a, 0x11, 0x6c, 0x61, 0x73, 0x74, 0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64,
    0x65, 0x64, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x11, 0x6c,
    0x61, 0x73, 0x74, 0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x2a, 0x0a, 0x10, 0x6c, 0x61, 0x73, 0x74, 0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64,
    0x54, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x10, 0x6c, 0x61, 0x73, 0x74,
    0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x54, 0x65, 0x72, 0x6d, 0x12, 0x16, 0x0a, 0x06,
    0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6f, 0x66,
    0x66, 0x73, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x22, 0x2a, 0x0a, 0x14,
    0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x2a, 0x47, 0x0a, 0x0c, 0x45, 0x78, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x50, 0x50, 0x45,
    0x4e, 0x44, 0x45, 0x4e, 0x54, 0x52, 0x49, 0x45, 0x53, 0x10, 0x00, 0x12, 0x0f, 0x0a, 0x0b, 0x52,
    0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x56, 0x4f, 0x54, 0x45, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f,
    0x49, 0x4e, 0x53, 0x54, 0x41, 0x4c, 0x4c, 0x53, 0x4e, 0x41, 0x50, 0x53, 0x48, 0x4f, 0x54, 0x10,
    0x02, 0x4a, 0x97, 0x1c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x45, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0f,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x03, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x03, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x04, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x05, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x06, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x06, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x0a, 0x02, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x15,
    0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x0e, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x0e, 0x02, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x0e, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x0f,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x16, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0f, 0x02, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0f, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10,
    0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x10, 0x02, 0x0f,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x12, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x02, 0x12, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x02, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x14, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x14, 0x02, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x09,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x15, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x15, 0x02, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x15, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x1d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x19, 0x02, 0x18, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x19, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x19, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1a, 0x02, 0x19, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x02, 0x12, 0x03, 0x1b, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x1b, 0x02, 0x1a, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x1b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x09,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x0f, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1c, 0x02, 0x1b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1c, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1f, 0x00, 0x26,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1c, 0x0a, 0x20, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x12, 0x22, 0x13, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x27, 0x73, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x20, 0x02, 0x1f, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x10, 0x11, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x21, 0x02, 0x16, 0x22, 0x22, 0x20, 0x73, 0x6f, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f,
    0x77, 0x65, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x21, 0x02, 0x20, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x21, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x21, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21,
    0x14, 0x15, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x22, 0x02, 0x1a, 0x22,
    0x33, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79,
    0x20, 0x70, 0x72, 0x65, 0x63, 0x65, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6f,
    0x6e, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x22,
    0x02, 0x21, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x09, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x22, 0x18, 0x19, 0x0a, 0x28, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x23, 0x02, 0x19, 0x22, 0x1b, 0x74, 0x65, 0x72, 0x6d,
    0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x65, 0x76, 0x4c, 0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x23, 0x02, 0x22, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23,
    0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x23, 0x17, 0x18,
    0x0a, 0x23, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x24, 0x02, 0x1e, 0x22, 0x16, 0x20,
    0x6c, 0x6f, 0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x24, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x24, 0x14, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24, 0x1c, 0x1d, 0x0a, 0x49, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x25, 0x02, 0x1a, 0x22, 0x3c, 0x20, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x68, 0x69, 0x67, 0x68, 0x65, 0x73, 0x74, 0x20, 0x6c, 0x6f,
    0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x25, 0x02, 0x24, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x25, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x25, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x25, 0x18,
    0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x28, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x28, 0x08, 0x1a, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x29, 0x02, 0x12, 0x22, 0x2a, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x54, 0x65, 0x72, 0x6d, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c,
    0x66, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x29, 0x02, 0x28,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x10, 0x11, 0x0a, 0x56, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x13, 0x22, 0x49, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20,
    0x69, 0x66, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x72, 0x65, 0x76, 0x4c, 0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x72, 0x65, 0x76, 0x4c, 0x6f, 0x67, 0x54, 0x65, 0x72,
    0x6d, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2a, 0x02,
    0x29, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x07, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x2d, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x2d, 0x08, 0x1a, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02,
    0x12, 0x22, 0x12, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x27, 0x73, 0x20,
    0x74, 0x65, 0x72, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x2e, 0x02, 0x2d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x10, 0x11, 0x0a, 0x28,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x02, 0x19, 0x22, 0x1b, 0x20, 0x63, 0x61,
    0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x76, 0x6f, 0x74, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x2f, 0x02, 0x2e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x2f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x2f, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x17,
    0x18, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x30, 0x02, 0x1a, 0x22, 0x25,
    0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x27, 0x73, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x30, 0x02, 0x2f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x30,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x09, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30, 0x18, 0x19, 0x0a, 0x31,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x19, 0x22, 0x24, 0x20, 0x74, 0x65,
    0x72, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x27,
    0x73, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x31, 0x02, 0x30, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x31, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x34, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x34,
    0x08, 0x18, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x35, 0x02, 0x12, 0x22,
    0x2d, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x65, 0x72, 0x6d, 0x2c, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x35, 0x02, 0x34, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x35, 0x10, 0x11, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x36, 0x02, 0x17, 0x22, 0x28, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x6d, 0x65, 0x61, 0x6e,
    0x73, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x63, 0x65,
    0x69, 0x76, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x6f, 0x74, 0x65, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x36, 0x02, 0x35, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x36, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x36, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x39,
    0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x39, 0x08, 0x1e, 0x0a,
    0x1c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x02, 0x12, 0x22, 0x0f, 0x20, 0x6c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x27, 0x73, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3a, 0x02, 0x39, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3a, 0x10, 0x11, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03,
    0x3b, 0x02, 0x16, 0x22, 0x22, 0x20, 0x73, 0x6f, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65,
    0x72, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x3b, 0x02, 0x3a, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3b,
    0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3b, 0x14, 0x15,
    0x0a, 0x42, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x3c, 0x02, 0x1f, 0x22, 0x35, 0x20,
    0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65,
    0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x72, 0x65, 0x73, 0x20, 0x75, 0x70,
    0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x3c,
    0x02, 0x3b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x09, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x1d, 0x1e, 0x0a, 0x28, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x3d, 0x02, 0x1e, 0x22, 0x1b, 0x20, 0x74, 0x65, 0x72,
    0x6d, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65,
    0x64, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x3d, 0x02, 0x3c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x3d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3d,
    0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3d, 0x1c, 0x1d,
    0x0a, 0x45, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x14, 0x22, 0x38, 0x20,
    0x62, 0x79, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x77, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x3e, 0x02, 0x3d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x3e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3e,
    0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3e, 0x12, 0x13,
    0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x11, 0x22, 0x31, 0x20,
    0x72, 0x61, 0x77, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x2c, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x74, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x04, 0x3f, 0x02, 0x3e, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3f, 0x0f, 0x10, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x06, 0x12, 0x03, 0x40, 0x02, 0x10, 0x22, 0x20, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73,
    0x74, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06,
    0x04, 0x12, 0x04, 0x40, 0x02, 0x3f, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x40, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x40, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x40, 0x0e,
    0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x43, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1c, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x44, 0x02, 0x12, 0x22, 0x2a, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x54, 0x65, 0x72, 0x6d, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c,
    0x66, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x44, 0x02, 0x43,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x10, 0x11, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
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
