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
pub struct CmdGetRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetRequest {}

impl CmdGetRequest {
    pub fn new() -> CmdGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetRequest,
        };
        unsafe { instance.get(CmdGetRequest::new) }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for CmdGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

impl ::protobuf::MessageStatic for CmdGetRequest {
    fn new() -> CmdGetRequest {
        CmdGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetRequest>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdGetRequest::get_key_for_reflect,
                    CmdGetRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetRequest>(
                    "CmdGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdGetResponse {
    // message fields
    pub result: ::std::vec::Vec<u8>,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetResponse {}

impl CmdGetResponse {
    pub fn new() -> CmdGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetResponse,
        };
        unsafe { instance.get(CmdGetResponse::new) }
    }

    // bytes result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::std::vec::Vec<u8>) {
        self.result = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.result, ::std::vec::Vec::new())
    }

    pub fn get_result(&self) -> &[u8] {
        &self.result
    }

    fn get_result_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.result
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.result)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.error)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.result.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.result);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if !self.result.is_empty() {
            os.write_bytes(1, &self.result)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for CmdGetResponse {
    fn new() -> CmdGetResponse {
        CmdGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetResponse>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "result",
                    CmdGetResponse::get_result_for_reflect,
                    CmdGetResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CmdGetResponse::get_error_for_reflect,
                    CmdGetResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetResponse>(
                    "CmdGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdSetRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdSetRequest {}

impl CmdSetRequest {
    pub fn new() -> CmdSetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdSetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdSetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdSetRequest,
        };
        unsafe { instance.get(CmdSetRequest::new) }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for CmdSetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.value)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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

impl ::protobuf::MessageStatic for CmdSetRequest {
    fn new() -> CmdSetRequest {
        CmdSetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdSetRequest>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdSetRequest::get_key_for_reflect,
                    CmdSetRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CmdSetRequest::get_value_for_reflect,
                    CmdSetRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdSetRequest>(
                    "CmdSetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdSetRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdSetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdSetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdSetResponse {
    // message fields
    pub result: bool,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdSetResponse {}

impl CmdSetResponse {
    pub fn new() -> CmdSetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdSetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdSetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdSetResponse,
        };
        unsafe { instance.get(CmdSetResponse::new) }
    }

    // bool result = 1;

    pub fn clear_result(&mut self) {
        self.result = false;
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: bool) {
        self.result = v;
    }

    pub fn get_result(&self) -> bool {
        self.result
    }

    fn get_result_for_reflect(&self) -> &bool {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut bool {
        &mut self.result
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdSetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.result = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.error)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.result != false {
            my_size += 2;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.result != false {
            os.write_bool(1, self.result)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for CmdSetResponse {
    fn new() -> CmdSetResponse {
        CmdSetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdSetResponse>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "result",
                    CmdSetResponse::get_result_for_reflect,
                    CmdSetResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CmdSetResponse::get_error_for_reflect,
                    CmdSetResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdSetResponse>(
                    "CmdSetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdSetResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdSetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdSetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdPingRequest {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPingRequest {}

impl CmdPingRequest {
    pub fn new() -> CmdPingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPingRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdPingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPingRequest,
        };
        unsafe { instance.get(CmdPingRequest::new) }
    }

    // string message = 1;

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

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for CmdPingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.message)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

impl ::protobuf::MessageStatic for CmdPingRequest {
    fn new() -> CmdPingRequest {
        CmdPingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPingRequest>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CmdPingRequest::get_message_for_reflect,
                    CmdPingRequest::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPingRequest>(
                    "CmdPingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPingRequest {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdPingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdPingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdPingResponse {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPingResponse {}

impl CmdPingResponse {
    pub fn new() -> CmdPingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPingResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdPingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPingResponse,
        };
        unsafe { instance.get(CmdPingResponse::new) }
    }

    // string message = 1;

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

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for CmdPingResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.message)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

impl ::protobuf::MessageStatic for CmdPingResponse {
    fn new() -> CmdPingResponse {
        CmdPingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPingResponse>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CmdPingResponse::get_message_for_reflect,
                    CmdPingResponse::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPingResponse>(
                    "CmdPingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPingResponse {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdPingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdPingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub field_type: MessageType,
    // message oneof groups
    body: ::std::option::Option<Request_oneof_body>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_body {
    cmd_ping_req(CmdPingRequest),
    cmd_get_req(CmdGetRequest),
    cmd_set_req(CmdSetRequest),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe { instance.get(Request::new) }
    }

    // .rpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = MessageType::CmdPing;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &MessageType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut MessageType {
        &mut self.field_type
    }

    // .rpcpb.CmdPingRequest cmd_ping_req = 2;

    pub fn clear_cmd_ping_req(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_ping_req(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_ping_req(&mut self, v: CmdPingRequest) {
        self.body = ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_ping_req(&mut self) -> &mut CmdPingRequest {
        if let ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(_)) = self.body {
        } else {
            self.body = ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(CmdPingRequest::new()));
        }
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_ping_req(&mut self) -> CmdPingRequest {
        if self.has_cmd_ping_req() {
            match self.body.take() {
                ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdPingRequest::new()
        }
    }

    pub fn get_cmd_ping_req(&self) -> &CmdPingRequest {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(ref v)) => v,
            _ => CmdPingRequest::default_instance(),
        }
    }

    // .rpcpb.CmdGetRequest cmd_get_req = 3;

    pub fn clear_cmd_get_req(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_get_req(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_get_req(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_req(&mut self, v: CmdGetRequest) {
        self.body = ::std::option::Option::Some(Request_oneof_body::cmd_get_req(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_get_req(&mut self) -> &mut CmdGetRequest {
        if let ::std::option::Option::Some(Request_oneof_body::cmd_get_req(_)) = self.body {
        } else {
            self.body =
                ::std::option::Option::Some(Request_oneof_body::cmd_get_req(CmdGetRequest::new()));
        }
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_get_req(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_get_req(&mut self) -> CmdGetRequest {
        if self.has_cmd_get_req() {
            match self.body.take() {
                ::std::option::Option::Some(Request_oneof_body::cmd_get_req(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdGetRequest::new()
        }
    }

    pub fn get_cmd_get_req(&self) -> &CmdGetRequest {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_get_req(ref v)) => v,
            _ => CmdGetRequest::default_instance(),
        }
    }

    // .rpcpb.CmdSetRequest cmd_set_req = 4;

    pub fn clear_cmd_set_req(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_set_req(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_set_req(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_set_req(&mut self, v: CmdSetRequest) {
        self.body = ::std::option::Option::Some(Request_oneof_body::cmd_set_req(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_set_req(&mut self) -> &mut CmdSetRequest {
        if let ::std::option::Option::Some(Request_oneof_body::cmd_set_req(_)) = self.body {
        } else {
            self.body =
                ::std::option::Option::Some(Request_oneof_body::cmd_set_req(CmdSetRequest::new()));
        }
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_set_req(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_set_req(&mut self) -> CmdSetRequest {
        if self.has_cmd_set_req() {
            match self.body.take() {
                ::std::option::Option::Some(Request_oneof_body::cmd_set_req(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdSetRequest::new()
        }
    }

    pub fn get_cmd_set_req(&self) -> &CmdSetRequest {
        match self.body {
            ::std::option::Option::Some(Request_oneof_body::cmd_set_req(ref v)) => v,
            _ => CmdSetRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_body::cmd_ping_req(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_body::cmd_get_req(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_body::cmd_set_req(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Request_oneof_body::cmd_ping_req(is.read_message()?));
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Request_oneof_body::cmd_get_req(is.read_message()?));
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Request_oneof_body::cmd_set_req(is.read_message()?));
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != MessageType::CmdPing {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let ::std::option::Option::Some(ref v) = self.body {
            match v {
                &Request_oneof_body::cmd_ping_req(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
                &Request_oneof_body::cmd_get_req(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
                &Request_oneof_body::cmd_set_req(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.field_type != MessageType::CmdPing {
            os.write_enum(1, self.field_type.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.body {
            match v {
                &Request_oneof_body::cmd_ping_req(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
                &Request_oneof_body::cmd_get_req(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
                &Request_oneof_body::cmd_set_req(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
            };
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

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Request::get_field_type_for_reflect,
                    Request::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdPingRequest>(
                    "cmd_ping_req",
                    Request::has_cmd_ping_req,
                    Request::get_cmd_ping_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdGetRequest>(
                    "cmd_get_req",
                    Request::has_cmd_get_req,
                    Request::get_cmd_get_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdSetRequest>(
                    "cmd_set_req",
                    Request::has_cmd_set_req,
                    Request::get_cmd_set_req,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_cmd_ping_req();
        self.clear_cmd_get_req();
        self.clear_cmd_set_req();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub field_type: MessageType,
    // message oneof groups
    body: ::std::option::Option<Response_oneof_body>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_body {
    cmd_ping_res(CmdPingResponse),
    cmd_get_res(CmdGetResponse),
    cmd_set_res(CmdSetResponse),
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe { instance.get(Response::new) }
    }

    // .rpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = MessageType::CmdPing;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &MessageType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut MessageType {
        &mut self.field_type
    }

    // .rpcpb.CmdPingResponse cmd_ping_res = 2;

    pub fn clear_cmd_ping_res(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_ping_res(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_ping_res(&mut self, v: CmdPingResponse) {
        self.body = ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_ping_res(&mut self) -> &mut CmdPingResponse {
        if let ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(_)) = self.body {
        } else {
            self.body = ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(CmdPingResponse::new()));
        }
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_ping_res(&mut self) -> CmdPingResponse {
        if self.has_cmd_ping_res() {
            match self.body.take() {
                ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdPingResponse::new()
        }
    }

    pub fn get_cmd_ping_res(&self) -> &CmdPingResponse {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(ref v)) => v,
            _ => CmdPingResponse::default_instance(),
        }
    }

    // .rpcpb.CmdGetResponse cmd_get_res = 3;

    pub fn clear_cmd_get_res(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_get_res(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_get_res(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_res(&mut self, v: CmdGetResponse) {
        self.body = ::std::option::Option::Some(Response_oneof_body::cmd_get_res(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_get_res(&mut self) -> &mut CmdGetResponse {
        if let ::std::option::Option::Some(Response_oneof_body::cmd_get_res(_)) = self.body {
        } else {
            self.body = ::std::option::Option::Some(Response_oneof_body::cmd_get_res(CmdGetResponse::new()));
        }
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_get_res(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_get_res(&mut self) -> CmdGetResponse {
        if self.has_cmd_get_res() {
            match self.body.take() {
                ::std::option::Option::Some(Response_oneof_body::cmd_get_res(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdGetResponse::new()
        }
    }

    pub fn get_cmd_get_res(&self) -> &CmdGetResponse {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_get_res(ref v)) => v,
            _ => CmdGetResponse::default_instance(),
        }
    }

    // .rpcpb.CmdSetResponse cmd_set_res = 4;

    pub fn clear_cmd_set_res(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_cmd_set_res(&self) -> bool {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_set_res(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cmd_set_res(&mut self, v: CmdSetResponse) {
        self.body = ::std::option::Option::Some(Response_oneof_body::cmd_set_res(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cmd_set_res(&mut self) -> &mut CmdSetResponse {
        if let ::std::option::Option::Some(Response_oneof_body::cmd_set_res(_)) = self.body {
        } else {
            self.body = ::std::option::Option::Some(Response_oneof_body::cmd_set_res(CmdSetResponse::new()));
        }
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_set_res(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cmd_set_res(&mut self) -> CmdSetResponse {
        if self.has_cmd_set_res() {
            match self.body.take() {
                ::std::option::Option::Some(Response_oneof_body::cmd_set_res(v)) => v,
                _ => panic!(),
            }
        } else {
            CmdSetResponse::new()
        }
    }

    pub fn get_cmd_set_res(&self) -> &CmdSetResponse {
        match self.body {
            ::std::option::Option::Some(Response_oneof_body::cmd_set_res(ref v)) => v,
            _ => CmdSetResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if let Some(Response_oneof_body::cmd_ping_res(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_body::cmd_get_res(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_body::cmd_set_res(ref v)) = self.body {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Response_oneof_body::cmd_ping_res(is.read_message()?));
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Response_oneof_body::cmd_get_res(is.read_message()?));
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.body = ::std::option::Option::Some(Response_oneof_body::cmd_set_res(is.read_message()?));
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != MessageType::CmdPing {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let ::std::option::Option::Some(ref v) = self.body {
            match v {
                &Response_oneof_body::cmd_ping_res(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
                &Response_oneof_body::cmd_get_res(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
                &Response_oneof_body::cmd_set_res(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                }
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.field_type != MessageType::CmdPing {
            os.write_enum(1, self.field_type.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.body {
            match v {
                &Response_oneof_body::cmd_ping_res(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
                &Response_oneof_body::cmd_get_res(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
                &Response_oneof_body::cmd_set_res(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                }
            };
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

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Response::get_field_type_for_reflect,
                    Response::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdPingResponse>(
                    "cmd_ping_res",
                    Response::has_cmd_ping_res,
                    Response::get_cmd_ping_res,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdGetResponse>(
                    "cmd_get_res",
                    Response::has_cmd_get_res,
                    Response::get_cmd_get_res,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CmdSetResponse>(
                    "cmd_set_res",
                    Response::has_cmd_set_res,
                    Response::get_cmd_set_res,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_cmd_ping_res();
        self.clear_cmd_get_res();
        self.clear_cmd_set_res();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    CmdPing = 0,
    CmdGet = 1,
    CmdSet = 2,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::CmdPing),
            1 => ::std::option::Option::Some(MessageType::CmdGet),
            2 => ::std::option::Option::Some(MessageType::CmdSet),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] =
            &[MessageType::CmdPing, MessageType::CmdGet, MessageType::CmdSet];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>)
                              -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                               ::protobuf::reflect::EnumDescriptor::new("MessageType",
                                                                        file_descriptor_proto())
                           })
        }
    }
}

impl ::std::marker::Copy for MessageType {}

impl ::std::default::Default for MessageType {
    fn default() -> Self {
        MessageType::CmdPing
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x14, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2f, 0x72, 0x70, 0x63, 0x70,
      0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x72, 0x70, 0x63, 0x70, 0x62, 0x22,
      0x21, 0x0a, 0x0d, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
      0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
      0x03, 0x6b, 0x65, 0x79, 0x22, 0x3e, 0x0a, 0x0e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52,
      0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
      0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
      0x74, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
      0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x37, 0x0a, 0x0d, 0x43, 0x6d, 0x64,
      0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b,
      0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14,
      0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05,
      0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x3e, 0x0a, 0x0e, 0x43, 0x6d, 0x64, 0x53, 0x65, 0x74,
      0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x73,
      0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
      0x6c, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01,
      0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x2a, 0x0a, 0x0e, 0x43, 0x6d,
      0x64, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a,
      0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
      0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x2b, 0x0a, 0x0f, 0x43, 0x6d, 0x64,
      0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a,
      0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
      0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0xe4, 0x01, 0x0a, 0x07, 0x52, 0x65,
      0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
      0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65,
      0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65,
      0x12, 0x39, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65,
      0x71, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x72, 0x70, 0x63, 0x70, 0x62,
      0x2e, 0x43, 0x6d, 0x64, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
      0x48, 0x00, 0x52, 0x0a, 0x63, 0x6d, 0x64, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x12,
      0x36, 0x0a, 0x0b, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18,
      0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
      0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52,
      0x09, 0x63, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x36, 0x0a, 0x0b, 0x63,
      0x6d, 0x64, 0x5f, 0x73, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28,
      0x0b, 0x32, 0x14, 0x2e, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x65,
      0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x09, 0x63, 0x6d, 0x64,
      0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x42, 0x06, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x22,
      0xe8, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a,
      0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x72,
      0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70,
      0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x3a, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f,
      0x70, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
      0x16, 0x2e, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x50, 0x69, 0x6e, 0x67,
      0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x6d, 0x64,
      0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x12, 0x37, 0x0a, 0x0b, 0x63, 0x6d, 0x64, 0x5f,
      0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
      0x2e, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65,
      0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x47, 0x65,
      0x74, 0x52, 0x65, 0x73, 0x12, 0x37, 0x0a, 0x0b, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x65, 0x74,
      0x5f, 0x72, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x72, 0x70,
      0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
      0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x53, 0x65, 0x74, 0x52, 0x65,
      0x73, 0x42, 0x06, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x2a, 0x32, 0x0a, 0x0b, 0x4d, 0x65,
      0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6d,
      0x64, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6d, 0x64, 0x47,
      0x65, 0x74, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6d, 0x64, 0x53, 0x65, 0x74, 0x10,
      0x02, 0x4a, 0xf3, 0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x34, 0x01, 0x0a, 0x08, 0x0a,
      0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01,
      0x08, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x05, 0x01, 0x0a,
      0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x00, 0x04, 0x12, 0x04, 0x04, 0x04, 0x03, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x00, 0x01, 0x12, 0x03, 0x04, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
      0x03, 0x12, 0x03, 0x04, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07,
      0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x16,
      0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x17, 0x0a, 0x0d,
      0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x18, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
      0x01, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
      0x01, 0x04, 0x12, 0x04, 0x09, 0x04, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
      0x01, 0x05, 0x12, 0x03, 0x09, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
      0x01, 0x12, 0x03, 0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
      0x12, 0x03, 0x09, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00,
      0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x15, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x16, 0x0a, 0x0d, 0x0a,
      0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x17, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
      0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
      0x04, 0x12, 0x04, 0x0e, 0x04, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
      0x05, 0x12, 0x03, 0x0e, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01,
      0x12, 0x03, 0x0e, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
      0x03, 0x0e, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x11, 0x00, 0x14,
      0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x11, 0x08, 0x16, 0x0a, 0x0b,
      0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x12, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
      0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x12, 0x04, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
      0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
      0x01, 0x12, 0x03, 0x13, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
      0x12, 0x04, 0x13, 0x04, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
      0x12, 0x03, 0x13, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
      0x03, 0x13, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
      0x13, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x16, 0x00, 0x18, 0x01,
      0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x16, 0x08, 0x16, 0x0a, 0x0b, 0x0a,
      0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x17, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
      0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x17, 0x04, 0x16, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
      0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
      0x00, 0x03, 0x12, 0x03, 0x17, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
      0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1a, 0x08,
      0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x19, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1b, 0x04, 0x1a, 0x19, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x04, 0x0a, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02,
      0x05, 0x00, 0x12, 0x04, 0x1e, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
      0x12, 0x03, 0x1e, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03,
      0x1f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f,
      0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1f, 0x19,
      0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x20, 0x04, 0x1b, 0x0a,
      0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x04, 0x0a, 0x0a, 0x0c,
      0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x20, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
      0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x21, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
      0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
      0x02, 0x02, 0x02, 0x12, 0x03, 0x21, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12,
      0x04, 0x24, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x24,
      0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x25, 0x04, 0x36,
      0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x25, 0x04, 0x24, 0x11,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x04, 0x0f, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x20, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x34, 0x35, 0x0a, 0x0c, 0x0a,
      0x04, 0x04, 0x06, 0x08, 0x00, 0x12, 0x04, 0x26, 0x04, 0x2a, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x06, 0x08, 0x00, 0x01, 0x12, 0x03, 0x26, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
      0x06, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
      0x01, 0x06, 0x12, 0x03, 0x27, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
      0x01, 0x12, 0x03, 0x27, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03,
      0x12, 0x03, 0x27, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03,
      0x28, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x28,
      0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x20,
      0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x38, 0x39,
      0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x29, 0x08, 0x3a, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x29, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x29, 0x20, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x29, 0x38, 0x39, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
      0x07, 0x12, 0x04, 0x2d, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
      0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2e,
      0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2e, 0x04,
      0x2d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x04,
      0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x1c, 0x20,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x34, 0x35, 0x0a,
      0x0c, 0x0a, 0x04, 0x04, 0x07, 0x08, 0x00, 0x12, 0x04, 0x2f, 0x04, 0x33, 0x05, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x07, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a,
      0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x30, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
      0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
      0x01, 0x03, 0x12, 0x03, 0x30, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02,
      0x12, 0x03, 0x31, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12,
      0x03, 0x31, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03,
      0x31, 0x20, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31,
      0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x32, 0x08, 0x3a,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12, 0x03, 0x32, 0x08, 0x16, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x32, 0x20, 0x2b, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x32, 0x38, 0x39, 0x62, 0x06, 0x70,
      0x72, 0x6f, 0x74, 0x6f, 0x33];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
