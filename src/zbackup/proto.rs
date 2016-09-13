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
pub struct EncryptionKeyInfo {
    // message fields
    salt: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rounds: ::std::option::Option<u32>,
    encrypted_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_check_input: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_check_hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncryptionKeyInfo {}

impl EncryptionKeyInfo {
    pub fn new() -> EncryptionKeyInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncryptionKeyInfo {
        static mut instance: ::protobuf::lazy::Lazy<EncryptionKeyInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptionKeyInfo,
        };
        unsafe {
            instance.get(|| {
                EncryptionKeyInfo {
                    salt: ::protobuf::SingularField::none(),
                    rounds: ::std::option::Option::None,
                    encrypted_key: ::protobuf::SingularField::none(),
                    key_check_input: ::protobuf::SingularField::none(),
                    key_check_hmac: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes salt = 1;

    pub fn clear_salt(&mut self) {
        self.salt.clear();
    }

    pub fn has_salt(&self) -> bool {
        self.salt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_salt(&mut self, v: ::std::vec::Vec<u8>) {
        self.salt = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_salt(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.salt.is_none() {
            self.salt.set_default();
        };
        self.salt.as_mut().unwrap()
    }

    // Take field
    pub fn take_salt(&mut self) -> ::std::vec::Vec<u8> {
        self.salt.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_salt(&self) -> &[u8] {
        match self.salt.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 rounds = 2;

    pub fn clear_rounds(&mut self) {
        self.rounds = ::std::option::Option::None;
    }

    pub fn has_rounds(&self) -> bool {
        self.rounds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rounds(&mut self, v: u32) {
        self.rounds = ::std::option::Option::Some(v);
    }

    pub fn get_rounds(&self) -> u32 {
        self.rounds.unwrap_or(0)
    }

    // required bytes encrypted_key = 3;

    pub fn clear_encrypted_key(&mut self) {
        self.encrypted_key.clear();
    }

    pub fn has_encrypted_key(&self) -> bool {
        self.encrypted_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_key.is_none() {
            self.encrypted_key.set_default();
        };
        self.encrypted_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_key(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_key(&self) -> &[u8] {
        match self.encrypted_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes key_check_input = 4;

    pub fn clear_key_check_input(&mut self) {
        self.key_check_input.clear();
    }

    pub fn has_key_check_input(&self) -> bool {
        self.key_check_input.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_check_input(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_check_input = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_check_input(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key_check_input.is_none() {
            self.key_check_input.set_default();
        };
        self.key_check_input.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_check_input(&mut self) -> ::std::vec::Vec<u8> {
        self.key_check_input.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key_check_input(&self) -> &[u8] {
        match self.key_check_input.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes key_check_hmac = 5;

    pub fn clear_key_check_hmac(&mut self) {
        self.key_check_hmac.clear();
    }

    pub fn has_key_check_hmac(&self) -> bool {
        self.key_check_hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_check_hmac(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_check_hmac = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_check_hmac(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key_check_hmac.is_none() {
            self.key_check_hmac.set_default();
        };
        self.key_check_hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_check_hmac(&mut self) -> ::std::vec::Vec<u8> {
        self.key_check_hmac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key_check_hmac(&self) -> &[u8] {
        match self.key_check_hmac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for EncryptionKeyInfo {
    fn is_initialized(&self) -> bool {
        if self.salt.is_none() {
            return false;
        };
        if self.rounds.is_none() {
            return false;
        };
        if self.encrypted_key.is_none() {
            return false;
        };
        if self.key_check_input.is_none() {
            return false;
        };
        if self.key_check_hmac.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.salt));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.rounds = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_key));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key_check_input));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key_check_hmac));
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
        for value in &self.salt {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.rounds {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.encrypted_key {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.key_check_input {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.key_check_hmac {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.salt.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.rounds {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.encrypted_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.key_check_input.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.key_check_hmac.as_ref() {
            try!(os.write_bytes(5, &v));
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
        ::std::any::TypeId::of::<EncryptionKeyInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncryptionKeyInfo {
    fn new() -> EncryptionKeyInfo {
        EncryptionKeyInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncryptionKeyInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "salt",
                    EncryptionKeyInfo::has_salt,
                    EncryptionKeyInfo::get_salt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "rounds",
                    EncryptionKeyInfo::has_rounds,
                    EncryptionKeyInfo::get_rounds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_key",
                    EncryptionKeyInfo::has_encrypted_key,
                    EncryptionKeyInfo::get_encrypted_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key_check_input",
                    EncryptionKeyInfo::has_key_check_input,
                    EncryptionKeyInfo::get_key_check_input,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key_check_hmac",
                    EncryptionKeyInfo::has_key_check_hmac,
                    EncryptionKeyInfo::get_key_check_hmac,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptionKeyInfo>(
                    "EncryptionKeyInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncryptionKeyInfo {
    fn clear(&mut self) {
        self.clear_salt();
        self.clear_rounds();
        self.clear_encrypted_key();
        self.clear_key_check_input();
        self.clear_key_check_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncryptionKeyInfo {
    fn eq(&self, other: &EncryptionKeyInfo) -> bool {
        self.salt == other.salt &&
        self.rounds == other.rounds &&
        self.encrypted_key == other.encrypted_key &&
        self.key_check_input == other.key_check_input &&
        self.key_check_hmac == other.key_check_hmac &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncryptionKeyInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageInfo {
    // message fields
    chunk_max_size: ::std::option::Option<u32>,
    bundle_max_payload_size: ::std::option::Option<u32>,
    encryption_key: ::protobuf::SingularPtrField<EncryptionKeyInfo>,
    default_compression_method: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageInfo {}

impl StorageInfo {
    pub fn new() -> StorageInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageInfo {
        static mut instance: ::protobuf::lazy::Lazy<StorageInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageInfo,
        };
        unsafe {
            instance.get(|| {
                StorageInfo {
                    chunk_max_size: ::std::option::Option::None,
                    bundle_max_payload_size: ::std::option::Option::None,
                    encryption_key: ::protobuf::SingularPtrField::none(),
                    default_compression_method: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 chunk_max_size = 1;

    pub fn clear_chunk_max_size(&mut self) {
        self.chunk_max_size = ::std::option::Option::None;
    }

    pub fn has_chunk_max_size(&self) -> bool {
        self.chunk_max_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunk_max_size(&mut self, v: u32) {
        self.chunk_max_size = ::std::option::Option::Some(v);
    }

    pub fn get_chunk_max_size(&self) -> u32 {
        self.chunk_max_size.unwrap_or(0)
    }

    // optional uint32 bundle_max_payload_size = 2;

    pub fn clear_bundle_max_payload_size(&mut self) {
        self.bundle_max_payload_size = ::std::option::Option::None;
    }

    pub fn has_bundle_max_payload_size(&self) -> bool {
        self.bundle_max_payload_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bundle_max_payload_size(&mut self, v: u32) {
        self.bundle_max_payload_size = ::std::option::Option::Some(v);
    }

    pub fn get_bundle_max_payload_size(&self) -> u32 {
        self.bundle_max_payload_size.unwrap_or(0)
    }

    // optional .EncryptionKeyInfo encryption_key = 3;

    pub fn clear_encryption_key(&mut self) {
        self.encryption_key.clear();
    }

    pub fn has_encryption_key(&self) -> bool {
        self.encryption_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryption_key(&mut self, v: EncryptionKeyInfo) {
        self.encryption_key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryption_key(&mut self) -> &mut EncryptionKeyInfo {
        if self.encryption_key.is_none() {
            self.encryption_key.set_default();
        };
        self.encryption_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryption_key(&mut self) -> EncryptionKeyInfo {
        self.encryption_key.take().unwrap_or_else(|| EncryptionKeyInfo::new())
    }

    pub fn get_encryption_key(&self) -> &EncryptionKeyInfo {
        self.encryption_key.as_ref().unwrap_or_else(|| EncryptionKeyInfo::default_instance())
    }

    // optional string default_compression_method = 4;

    pub fn clear_default_compression_method(&mut self) {
        self.default_compression_method.clear();
    }

    pub fn has_default_compression_method(&self) -> bool {
        self.default_compression_method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_compression_method(&mut self, v: ::std::string::String) {
        self.default_compression_method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_compression_method(&mut self) -> &mut ::std::string::String {
        if self.default_compression_method.is_none() {
            self.default_compression_method.set_default();
        };
        self.default_compression_method.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_compression_method(&mut self) -> ::std::string::String {
        self.default_compression_method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_default_compression_method(&self) -> &str {
        match self.default_compression_method.as_ref() {
            Some(v) => &v,
            None => "lzma",
        }
    }
}

impl ::protobuf::Message for StorageInfo {
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
                    let tmp = try!(is.read_uint32());
                    self.chunk_max_size = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.bundle_max_payload_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.encryption_key));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.default_compression_method));
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
        for value in &self.chunk_max_size {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.bundle_max_payload_size {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.encryption_key {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.default_compression_method {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.chunk_max_size {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.bundle_max_payload_size {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.encryption_key.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.default_compression_method.as_ref() {
            try!(os.write_string(4, &v));
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
        ::std::any::TypeId::of::<StorageInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageInfo {
    fn new() -> StorageInfo {
        StorageInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "chunk_max_size",
                    StorageInfo::has_chunk_max_size,
                    StorageInfo::get_chunk_max_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "bundle_max_payload_size",
                    StorageInfo::has_bundle_max_payload_size,
                    StorageInfo::get_bundle_max_payload_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "encryption_key",
                    StorageInfo::has_encryption_key,
                    StorageInfo::get_encryption_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "default_compression_method",
                    StorageInfo::has_default_compression_method,
                    StorageInfo::get_default_compression_method,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageInfo>(
                    "StorageInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageInfo {
    fn clear(&mut self) {
        self.clear_chunk_max_size();
        self.clear_bundle_max_payload_size();
        self.clear_encryption_key();
        self.clear_default_compression_method();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageInfo {
    fn eq(&self, other: &StorageInfo) -> bool {
        self.chunk_max_size == other.chunk_max_size &&
        self.bundle_max_payload_size == other.bundle_max_payload_size &&
        self.encryption_key == other.encryption_key &&
        self.default_compression_method == other.default_compression_method &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LZMAConfigInfo {
    // message fields
    compression_level: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LZMAConfigInfo {}

impl LZMAConfigInfo {
    pub fn new() -> LZMAConfigInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LZMAConfigInfo {
        static mut instance: ::protobuf::lazy::Lazy<LZMAConfigInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LZMAConfigInfo,
        };
        unsafe {
            instance.get(|| {
                LZMAConfigInfo {
                    compression_level: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 compression_level = 1;

    pub fn clear_compression_level(&mut self) {
        self.compression_level = ::std::option::Option::None;
    }

    pub fn has_compression_level(&self) -> bool {
        self.compression_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compression_level(&mut self, v: u32) {
        self.compression_level = ::std::option::Option::Some(v);
    }

    pub fn get_compression_level(&self) -> u32 {
        self.compression_level.unwrap_or(6u32)
    }
}

impl ::protobuf::Message for LZMAConfigInfo {
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
                    let tmp = try!(is.read_uint32());
                    self.compression_level = ::std::option::Option::Some(tmp);
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
        for value in &self.compression_level {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.compression_level {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<LZMAConfigInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LZMAConfigInfo {
    fn new() -> LZMAConfigInfo {
        LZMAConfigInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<LZMAConfigInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "compression_level",
                    LZMAConfigInfo::has_compression_level,
                    LZMAConfigInfo::get_compression_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LZMAConfigInfo>(
                    "LZMAConfigInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LZMAConfigInfo {
    fn clear(&mut self) {
        self.clear_compression_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LZMAConfigInfo {
    fn eq(&self, other: &LZMAConfigInfo) -> bool {
        self.compression_level == other.compression_level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LZMAConfigInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ChunkConfigInfo {
    // message fields
    max_size: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChunkConfigInfo {}

impl ChunkConfigInfo {
    pub fn new() -> ChunkConfigInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChunkConfigInfo {
        static mut instance: ::protobuf::lazy::Lazy<ChunkConfigInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChunkConfigInfo,
        };
        unsafe {
            instance.get(|| {
                ChunkConfigInfo {
                    max_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 max_size = 1;

    pub fn clear_max_size(&mut self) {
        self.max_size = ::std::option::Option::None;
    }

    pub fn has_max_size(&self) -> bool {
        self.max_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_size(&mut self, v: u32) {
        self.max_size = ::std::option::Option::Some(v);
    }

    pub fn get_max_size(&self) -> u32 {
        self.max_size.unwrap_or(65536u32)
    }
}

impl ::protobuf::Message for ChunkConfigInfo {
    fn is_initialized(&self) -> bool {
        if self.max_size.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_uint32());
                    self.max_size = ::std::option::Option::Some(tmp);
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
        for value in &self.max_size {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_size {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<ChunkConfigInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChunkConfigInfo {
    fn new() -> ChunkConfigInfo {
        ChunkConfigInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChunkConfigInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_size",
                    ChunkConfigInfo::has_max_size,
                    ChunkConfigInfo::get_max_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChunkConfigInfo>(
                    "ChunkConfigInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChunkConfigInfo {
    fn clear(&mut self) {
        self.clear_max_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ChunkConfigInfo {
    fn eq(&self, other: &ChunkConfigInfo) -> bool {
        self.max_size == other.max_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ChunkConfigInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BundleConfigInfo {
    // message fields
    max_payload_size: ::std::option::Option<u32>,
    compression_method: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BundleConfigInfo {}

impl BundleConfigInfo {
    pub fn new() -> BundleConfigInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BundleConfigInfo {
        static mut instance: ::protobuf::lazy::Lazy<BundleConfigInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BundleConfigInfo,
        };
        unsafe {
            instance.get(|| {
                BundleConfigInfo {
                    max_payload_size: ::std::option::Option::None,
                    compression_method: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 max_payload_size = 2;

    pub fn clear_max_payload_size(&mut self) {
        self.max_payload_size = ::std::option::Option::None;
    }

    pub fn has_max_payload_size(&self) -> bool {
        self.max_payload_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_payload_size(&mut self, v: u32) {
        self.max_payload_size = ::std::option::Option::Some(v);
    }

    pub fn get_max_payload_size(&self) -> u32 {
        self.max_payload_size.unwrap_or(2097152u32)
    }

    // optional string compression_method = 3;

    pub fn clear_compression_method(&mut self) {
        self.compression_method.clear();
    }

    pub fn has_compression_method(&self) -> bool {
        self.compression_method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compression_method(&mut self, v: ::std::string::String) {
        self.compression_method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compression_method(&mut self) -> &mut ::std::string::String {
        if self.compression_method.is_none() {
            self.compression_method.set_default();
        };
        self.compression_method.as_mut().unwrap()
    }

    // Take field
    pub fn take_compression_method(&mut self) -> ::std::string::String {
        self.compression_method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_compression_method(&self) -> &str {
        match self.compression_method.as_ref() {
            Some(v) => &v,
            None => "lzma",
        }
    }
}

impl ::protobuf::Message for BundleConfigInfo {
    fn is_initialized(&self) -> bool {
        if self.max_payload_size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.max_payload_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.compression_method));
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
        for value in &self.max_payload_size {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.compression_method {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_payload_size {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.compression_method.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<BundleConfigInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BundleConfigInfo {
    fn new() -> BundleConfigInfo {
        BundleConfigInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BundleConfigInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_payload_size",
                    BundleConfigInfo::has_max_payload_size,
                    BundleConfigInfo::get_max_payload_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "compression_method",
                    BundleConfigInfo::has_compression_method,
                    BundleConfigInfo::get_compression_method,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BundleConfigInfo>(
                    "BundleConfigInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BundleConfigInfo {
    fn clear(&mut self) {
        self.clear_max_payload_size();
        self.clear_compression_method();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BundleConfigInfo {
    fn eq(&self, other: &BundleConfigInfo) -> bool {
        self.max_payload_size == other.max_payload_size &&
        self.compression_method == other.compression_method &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BundleConfigInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ConfigInfo {
    // message fields
    chunk: ::protobuf::SingularPtrField<ChunkConfigInfo>,
    bundle: ::protobuf::SingularPtrField<BundleConfigInfo>,
    lzma: ::protobuf::SingularPtrField<LZMAConfigInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigInfo {}

impl ConfigInfo {
    pub fn new() -> ConfigInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigInfo {
        static mut instance: ::protobuf::lazy::Lazy<ConfigInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigInfo,
        };
        unsafe {
            instance.get(|| {
                ConfigInfo {
                    chunk: ::protobuf::SingularPtrField::none(),
                    bundle: ::protobuf::SingularPtrField::none(),
                    lzma: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .ChunkConfigInfo chunk = 1;

    pub fn clear_chunk(&mut self) {
        self.chunk.clear();
    }

    pub fn has_chunk(&self) -> bool {
        self.chunk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunk(&mut self, v: ChunkConfigInfo) {
        self.chunk = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chunk(&mut self) -> &mut ChunkConfigInfo {
        if self.chunk.is_none() {
            self.chunk.set_default();
        };
        self.chunk.as_mut().unwrap()
    }

    // Take field
    pub fn take_chunk(&mut self) -> ChunkConfigInfo {
        self.chunk.take().unwrap_or_else(|| ChunkConfigInfo::new())
    }

    pub fn get_chunk(&self) -> &ChunkConfigInfo {
        self.chunk.as_ref().unwrap_or_else(|| ChunkConfigInfo::default_instance())
    }

    // required .BundleConfigInfo bundle = 2;

    pub fn clear_bundle(&mut self) {
        self.bundle.clear();
    }

    pub fn has_bundle(&self) -> bool {
        self.bundle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bundle(&mut self, v: BundleConfigInfo) {
        self.bundle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bundle(&mut self) -> &mut BundleConfigInfo {
        if self.bundle.is_none() {
            self.bundle.set_default();
        };
        self.bundle.as_mut().unwrap()
    }

    // Take field
    pub fn take_bundle(&mut self) -> BundleConfigInfo {
        self.bundle.take().unwrap_or_else(|| BundleConfigInfo::new())
    }

    pub fn get_bundle(&self) -> &BundleConfigInfo {
        self.bundle.as_ref().unwrap_or_else(|| BundleConfigInfo::default_instance())
    }

    // required .LZMAConfigInfo lzma = 3;

    pub fn clear_lzma(&mut self) {
        self.lzma.clear();
    }

    pub fn has_lzma(&self) -> bool {
        self.lzma.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lzma(&mut self, v: LZMAConfigInfo) {
        self.lzma = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lzma(&mut self) -> &mut LZMAConfigInfo {
        if self.lzma.is_none() {
            self.lzma.set_default();
        };
        self.lzma.as_mut().unwrap()
    }

    // Take field
    pub fn take_lzma(&mut self) -> LZMAConfigInfo {
        self.lzma.take().unwrap_or_else(|| LZMAConfigInfo::new())
    }

    pub fn get_lzma(&self) -> &LZMAConfigInfo {
        self.lzma.as_ref().unwrap_or_else(|| LZMAConfigInfo::default_instance())
    }
}

impl ::protobuf::Message for ConfigInfo {
    fn is_initialized(&self) -> bool {
        if self.chunk.is_none() {
            return false;
        };
        if self.bundle.is_none() {
            return false;
        };
        if self.lzma.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chunk));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bundle));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lzma));
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
        for value in &self.chunk {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.bundle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.lzma {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.chunk.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.bundle.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.lzma.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<ConfigInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConfigInfo {
    fn new() -> ConfigInfo {
        ConfigInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "chunk",
                    ConfigInfo::has_chunk,
                    ConfigInfo::get_chunk,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "bundle",
                    ConfigInfo::has_bundle,
                    ConfigInfo::get_bundle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lzma",
                    ConfigInfo::has_lzma,
                    ConfigInfo::get_lzma,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigInfo>(
                    "ConfigInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigInfo {
    fn clear(&mut self) {
        self.clear_chunk();
        self.clear_bundle();
        self.clear_lzma();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ConfigInfo {
    fn eq(&self, other: &ConfigInfo) -> bool {
        self.chunk == other.chunk &&
        self.bundle == other.bundle &&
        self.lzma == other.lzma &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ConfigInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExtendedStorageInfo {
    // message fields
    config: ::protobuf::SingularPtrField<ConfigInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtendedStorageInfo {}

impl ExtendedStorageInfo {
    pub fn new() -> ExtendedStorageInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtendedStorageInfo {
        static mut instance: ::protobuf::lazy::Lazy<ExtendedStorageInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtendedStorageInfo,
        };
        unsafe {
            instance.get(|| {
                ExtendedStorageInfo {
                    config: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ConfigInfo config = 1;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ConfigInfo) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ConfigInfo {
        if self.config.is_none() {
            self.config.set_default();
        };
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ConfigInfo {
        self.config.take().unwrap_or_else(|| ConfigInfo::new())
    }

    pub fn get_config(&self) -> &ConfigInfo {
        self.config.as_ref().unwrap_or_else(|| ConfigInfo::default_instance())
    }
}

impl ::protobuf::Message for ExtendedStorageInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config));
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
        for value in &self.config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.config.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<ExtendedStorageInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtendedStorageInfo {
    fn new() -> ExtendedStorageInfo {
        ExtendedStorageInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtendedStorageInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "config",
                    ExtendedStorageInfo::has_config,
                    ExtendedStorageInfo::get_config,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtendedStorageInfo>(
                    "ExtendedStorageInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtendedStorageInfo {
    fn clear(&mut self) {
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtendedStorageInfo {
    fn eq(&self, other: &ExtendedStorageInfo) -> bool {
        self.config == other.config &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtendedStorageInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BundleInfo {
    // message fields
    chunk_record: ::protobuf::RepeatedField<BundleInfo_ChunkRecord>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BundleInfo {}

impl BundleInfo {
    pub fn new() -> BundleInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BundleInfo {
        static mut instance: ::protobuf::lazy::Lazy<BundleInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BundleInfo,
        };
        unsafe {
            instance.get(|| {
                BundleInfo {
                    chunk_record: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .BundleInfo.ChunkRecord chunk_record = 1;

    pub fn clear_chunk_record(&mut self) {
        self.chunk_record.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunk_record(&mut self, v: ::protobuf::RepeatedField<BundleInfo_ChunkRecord>) {
        self.chunk_record = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunk_record(&mut self) -> &mut ::protobuf::RepeatedField<BundleInfo_ChunkRecord> {
        &mut self.chunk_record
    }

    // Take field
    pub fn take_chunk_record(&mut self) -> ::protobuf::RepeatedField<BundleInfo_ChunkRecord> {
        ::std::mem::replace(&mut self.chunk_record, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunk_record(&self) -> &[BundleInfo_ChunkRecord] {
        &self.chunk_record
    }
}

impl ::protobuf::Message for BundleInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunk_record));
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
        for value in &self.chunk_record {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunk_record {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<BundleInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BundleInfo {
    fn new() -> BundleInfo {
        BundleInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BundleInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "chunk_record",
                    BundleInfo::get_chunk_record,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BundleInfo>(
                    "BundleInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BundleInfo {
    fn clear(&mut self) {
        self.clear_chunk_record();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BundleInfo {
    fn eq(&self, other: &BundleInfo) -> bool {
        self.chunk_record == other.chunk_record &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BundleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BundleInfo_ChunkRecord {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    size: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BundleInfo_ChunkRecord {}

impl BundleInfo_ChunkRecord {
    pub fn new() -> BundleInfo_ChunkRecord {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BundleInfo_ChunkRecord {
        static mut instance: ::protobuf::lazy::Lazy<BundleInfo_ChunkRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BundleInfo_ChunkRecord,
        };
        unsafe {
            instance.get(|| {
                BundleInfo_ChunkRecord {
                    id: ::protobuf::SingularField::none(),
                    size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u32 {
        self.size.unwrap_or(0)
    }
}

impl ::protobuf::Message for BundleInfo_ChunkRecord {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.size = ::std::option::Option::Some(tmp);
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
        for value in &self.id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.size {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.size {
            try!(os.write_uint32(2, v));
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
        ::std::any::TypeId::of::<BundleInfo_ChunkRecord>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BundleInfo_ChunkRecord {
    fn new() -> BundleInfo_ChunkRecord {
        BundleInfo_ChunkRecord::new()
    }

    fn descriptor_static(_: ::std::option::Option<BundleInfo_ChunkRecord>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "id",
                    BundleInfo_ChunkRecord::has_id,
                    BundleInfo_ChunkRecord::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "size",
                    BundleInfo_ChunkRecord::has_size,
                    BundleInfo_ChunkRecord::get_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BundleInfo_ChunkRecord>(
                    "BundleInfo_ChunkRecord",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BundleInfo_ChunkRecord {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BundleInfo_ChunkRecord {
    fn eq(&self, other: &BundleInfo_ChunkRecord) -> bool {
        self.id == other.id &&
        self.size == other.size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BundleInfo_ChunkRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FileHeader {
    // message fields
    version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileHeader {}

impl FileHeader {
    pub fn new() -> FileHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileHeader {
        static mut instance: ::protobuf::lazy::Lazy<FileHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileHeader,
        };
        unsafe {
            instance.get(|| {
                FileHeader {
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }
}

impl ::protobuf::Message for FileHeader {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_uint32());
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<FileHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileHeader {
    fn new() -> FileHeader {
        FileHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "version",
                    FileHeader::has_version,
                    FileHeader::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileHeader>(
                    "FileHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileHeader {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FileHeader {
    fn eq(&self, other: &FileHeader) -> bool {
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FileHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BundleFileHeader {
    // message fields
    version: ::std::option::Option<u32>,
    compression_method: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BundleFileHeader {}

impl BundleFileHeader {
    pub fn new() -> BundleFileHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BundleFileHeader {
        static mut instance: ::protobuf::lazy::Lazy<BundleFileHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BundleFileHeader,
        };
        unsafe {
            instance.get(|| {
                BundleFileHeader {
                    version: ::std::option::Option::None,
                    compression_method: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    // optional string compression_method = 2;

    pub fn clear_compression_method(&mut self) {
        self.compression_method.clear();
    }

    pub fn has_compression_method(&self) -> bool {
        self.compression_method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compression_method(&mut self, v: ::std::string::String) {
        self.compression_method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compression_method(&mut self) -> &mut ::std::string::String {
        if self.compression_method.is_none() {
            self.compression_method.set_default();
        };
        self.compression_method.as_mut().unwrap()
    }

    // Take field
    pub fn take_compression_method(&mut self) -> ::std::string::String {
        self.compression_method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_compression_method(&self) -> &str {
        match self.compression_method.as_ref() {
            Some(v) => &v,
            None => "lzma",
        }
    }
}

impl ::protobuf::Message for BundleFileHeader {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_uint32());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.compression_method));
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
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.compression_method {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.compression_method.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<BundleFileHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BundleFileHeader {
    fn new() -> BundleFileHeader {
        BundleFileHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<BundleFileHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "version",
                    BundleFileHeader::has_version,
                    BundleFileHeader::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "compression_method",
                    BundleFileHeader::has_compression_method,
                    BundleFileHeader::get_compression_method,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BundleFileHeader>(
                    "BundleFileHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BundleFileHeader {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_compression_method();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BundleFileHeader {
    fn eq(&self, other: &BundleFileHeader) -> bool {
        self.version == other.version &&
        self.compression_method == other.compression_method &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BundleFileHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IndexBundleHeader {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IndexBundleHeader {}

impl IndexBundleHeader {
    pub fn new() -> IndexBundleHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IndexBundleHeader {
        static mut instance: ::protobuf::lazy::Lazy<IndexBundleHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IndexBundleHeader,
        };
        unsafe {
            instance.get(|| {
                IndexBundleHeader {
                    id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for IndexBundleHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id));
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
        for value in &self.id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<IndexBundleHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IndexBundleHeader {
    fn new() -> IndexBundleHeader {
        IndexBundleHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<IndexBundleHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "id",
                    IndexBundleHeader::has_id,
                    IndexBundleHeader::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IndexBundleHeader>(
                    "IndexBundleHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IndexBundleHeader {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IndexBundleHeader {
    fn eq(&self, other: &IndexBundleHeader) -> bool {
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IndexBundleHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BackupInstruction {
    // message fields
    chunk_to_emit: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bytes_to_emit: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BackupInstruction {}

impl BackupInstruction {
    pub fn new() -> BackupInstruction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BackupInstruction {
        static mut instance: ::protobuf::lazy::Lazy<BackupInstruction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BackupInstruction,
        };
        unsafe {
            instance.get(|| {
                BackupInstruction {
                    chunk_to_emit: ::protobuf::SingularField::none(),
                    bytes_to_emit: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes chunk_to_emit = 1;

    pub fn clear_chunk_to_emit(&mut self) {
        self.chunk_to_emit.clear();
    }

    pub fn has_chunk_to_emit(&self) -> bool {
        self.chunk_to_emit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunk_to_emit(&mut self, v: ::std::vec::Vec<u8>) {
        self.chunk_to_emit = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chunk_to_emit(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.chunk_to_emit.is_none() {
            self.chunk_to_emit.set_default();
        };
        self.chunk_to_emit.as_mut().unwrap()
    }

    // Take field
    pub fn take_chunk_to_emit(&mut self) -> ::std::vec::Vec<u8> {
        self.chunk_to_emit.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_chunk_to_emit(&self) -> &[u8] {
        match self.chunk_to_emit.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes bytes_to_emit = 2;

    pub fn clear_bytes_to_emit(&mut self) {
        self.bytes_to_emit.clear();
    }

    pub fn has_bytes_to_emit(&self) -> bool {
        self.bytes_to_emit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_to_emit(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_to_emit = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_to_emit(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes_to_emit.is_none() {
            self.bytes_to_emit.set_default();
        };
        self.bytes_to_emit.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes_to_emit(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes_to_emit.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes_to_emit(&self) -> &[u8] {
        match self.bytes_to_emit.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for BackupInstruction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.chunk_to_emit));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes_to_emit));
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
        for value in &self.chunk_to_emit {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.bytes_to_emit {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.chunk_to_emit.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.bytes_to_emit.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<BackupInstruction>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BackupInstruction {
    fn new() -> BackupInstruction {
        BackupInstruction::new()
    }

    fn descriptor_static(_: ::std::option::Option<BackupInstruction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "chunk_to_emit",
                    BackupInstruction::has_chunk_to_emit,
                    BackupInstruction::get_chunk_to_emit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_to_emit",
                    BackupInstruction::has_bytes_to_emit,
                    BackupInstruction::get_bytes_to_emit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BackupInstruction>(
                    "BackupInstruction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BackupInstruction {
    fn clear(&mut self) {
        self.clear_chunk_to_emit();
        self.clear_bytes_to_emit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BackupInstruction {
    fn eq(&self, other: &BackupInstruction) -> bool {
        self.chunk_to_emit == other.chunk_to_emit &&
        self.bytes_to_emit == other.bytes_to_emit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BackupInstruction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BackupInfo {
    // message fields
    backup_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iterations: ::std::option::Option<u32>,
    size: ::std::option::Option<u64>,
    sha256: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    time: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BackupInfo {}

impl BackupInfo {
    pub fn new() -> BackupInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BackupInfo {
        static mut instance: ::protobuf::lazy::Lazy<BackupInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BackupInfo,
        };
        unsafe {
            instance.get(|| {
                BackupInfo {
                    backup_data: ::protobuf::SingularField::none(),
                    iterations: ::std::option::Option::None,
                    size: ::std::option::Option::None,
                    sha256: ::protobuf::SingularField::none(),
                    time: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes backup_data = 1;

    pub fn clear_backup_data(&mut self) {
        self.backup_data.clear();
    }

    pub fn has_backup_data(&self) -> bool {
        self.backup_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backup_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.backup_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_backup_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.backup_data.is_none() {
            self.backup_data.set_default();
        };
        self.backup_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_backup_data(&mut self) -> ::std::vec::Vec<u8> {
        self.backup_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_backup_data(&self) -> &[u8] {
        match self.backup_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 iterations = 2;

    pub fn clear_iterations(&mut self) {
        self.iterations = ::std::option::Option::None;
    }

    pub fn has_iterations(&self) -> bool {
        self.iterations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iterations(&mut self, v: u32) {
        self.iterations = ::std::option::Option::Some(v);
    }

    pub fn get_iterations(&self) -> u32 {
        self.iterations.unwrap_or(0u32)
    }

    // required uint64 size = 3;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    // required bytes sha256 = 4;

    pub fn clear_sha256(&mut self) {
        self.sha256.clear();
    }

    pub fn has_sha256(&self) -> bool {
        self.sha256.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha256(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha256 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha256(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha256.is_none() {
            self.sha256.set_default();
        };
        self.sha256.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha256(&mut self) -> ::std::vec::Vec<u8> {
        self.sha256.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sha256(&self) -> &[u8] {
        match self.sha256.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional int64 time = 5;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> i64 {
        self.time.unwrap_or(0)
    }
}

impl ::protobuf::Message for BackupInfo {
    fn is_initialized(&self) -> bool {
        if self.backup_data.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        if self.sha256.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.backup_data));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.iterations = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha256));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.time = ::std::option::Option::Some(tmp);
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
        for value in &self.backup_data {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.iterations {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.size {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sha256 {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.time {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.backup_data.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.iterations {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.size {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.sha256.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.time {
            try!(os.write_int64(5, v));
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
        ::std::any::TypeId::of::<BackupInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BackupInfo {
    fn new() -> BackupInfo {
        BackupInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BackupInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "backup_data",
                    BackupInfo::has_backup_data,
                    BackupInfo::get_backup_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "iterations",
                    BackupInfo::has_iterations,
                    BackupInfo::get_iterations,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "size",
                    BackupInfo::has_size,
                    BackupInfo::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "sha256",
                    BackupInfo::has_sha256,
                    BackupInfo::get_sha256,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "time",
                    BackupInfo::has_time,
                    BackupInfo::get_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BackupInfo>(
                    "BackupInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BackupInfo {
    fn clear(&mut self) {
        self.clear_backup_data();
        self.clear_iterations();
        self.clear_size();
        self.clear_sha256();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BackupInfo {
    fn eq(&self, other: &BackupInfo) -> bool {
        self.backup_data == other.backup_data &&
        self.iterations == other.iterations &&
        self.size == other.size &&
        self.sha256 == other.sha256 &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BackupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x17, 0x65, 0x74, 0x63, 0x2f, 0x7a, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2d, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x79, 0x0a, 0x11, 0x45, 0x6e, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x65, 0x79, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c,
    0x0a, 0x04, 0x73, 0x61, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06,
    0x72, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d,
    0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x17, 0x0a, 0x0f, 0x6b, 0x65, 0x79, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e,
    0x6b, 0x65, 0x79, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x68, 0x6d, 0x61, 0x63, 0x18, 0x05,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0xa8, 0x01, 0x0a, 0x0b, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x0e, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x5f, 0x6d, 0x61,
    0x78, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x02, 0x18, 0x01,
    0x12, 0x23, 0x0a, 0x17, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x42, 0x02, 0x18, 0x01, 0x12, 0x2a, 0x0a, 0x0e, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x65, 0x79, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x2c, 0x0a, 0x1a, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x3a, 0x04, 0x6c, 0x7a, 0x6d, 0x61, 0x42, 0x02, 0x18, 0x01, 0x22,
    0x2e, 0x0a, 0x0e, 0x4c, 0x5a, 0x4d, 0x41, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x1c, 0x0a, 0x11, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x36, 0x22,
    0x2a, 0x0a, 0x0f, 0x43, 0x68, 0x75, 0x6e, 0x6b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x17, 0x0a, 0x08, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0d, 0x3a, 0x05, 0x36, 0x35, 0x35, 0x33, 0x36, 0x22, 0x57, 0x0a, 0x10, 0x42,
    0x75, 0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x21, 0x0a, 0x10, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x3a, 0x07, 0x32, 0x30, 0x39, 0x37, 0x31,
    0x35, 0x32, 0x12, 0x20, 0x0a, 0x12, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x5f, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x3a, 0x04,
    0x6c, 0x7a, 0x6d, 0x61, 0x22, 0x6f, 0x0a, 0x0a, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x1f, 0x0a, 0x05, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x43, 0x68, 0x75, 0x6e, 0x6b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x21, 0x0a, 0x06, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1d, 0x0a, 0x04, 0x6c, 0x7a, 0x6d, 0x61, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x4c, 0x5a, 0x4d, 0x41, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x32, 0x0a, 0x13, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x64, 0x65,
    0x64, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x06,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x64, 0x0a, 0x0a, 0x42, 0x75, 0x6e,
    0x64, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d, 0x0a, 0x0c, 0x63, 0x68, 0x75, 0x6e, 0x6b,
    0x5f, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x68, 0x75, 0x6e, 0x6b,
    0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x1a, 0x27, 0x0a, 0x0b, 0x43, 0x68, 0x75, 0x6e, 0x6b, 0x52,
    0x65, 0x63, 0x6f, 0x72, 0x64, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22,
    0x1d, 0x0a, 0x0a, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0f, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x45,
    0x0a, 0x10, 0x42, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0d, 0x12, 0x20, 0x0a, 0x12, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x5f, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x3a,
    0x04, 0x6c, 0x7a, 0x6d, 0x61, 0x22, 0x1f, 0x0a, 0x11, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x42, 0x75,
    0x6e, 0x64, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x41, 0x0a, 0x11, 0x42, 0x61, 0x63, 0x6b, 0x75, 0x70,
    0x49, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x15, 0x0a, 0x0d, 0x63,
    0x68, 0x75, 0x6e, 0x6b, 0x5f, 0x74, 0x6f, 0x5f, 0x65, 0x6d, 0x69, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x65,
    0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x64, 0x0a, 0x0a, 0x42, 0x61, 0x63,
    0x6b, 0x75, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x61, 0x63, 0x6b, 0x75,
    0x70, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0a,
    0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x3a, 0x01, 0x30, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x4a,
    0xc8, 0x31, 0x0a, 0x07, 0x12, 0x05, 0x23, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0xbd, 0x03, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x23, 0x00, 0x34, 0x01, 0x1a, 0xce, 0x02, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b,
    0x65, 0x79, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x20, 0x41, 0x20, 0x75, 0x73,
    0x65, 0x72, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x70, 0x61,
    0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x20, 0x2d, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x0a, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x73, 0x61, 0x6c, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x6f, 0x75, 0x6e,
    0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x20, 0x61,
    0x20, 0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x0a, 0x20,
    0x6b, 0x65, 0x79, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x77, 0x61, 0x79, 0x20, 0x77, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f,
    0x72, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x0a, 0x20, 0x72, 0x65, 0x2d, 0x65,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x0a, 0x22, 0x60, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x2c, 0x20, 0x73, 0x61, 0x6c,
    0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x20, 0x75, 0x73, 0x69,
    0x6e, 0x67, 0x0a, 0x20, 0x50, 0x4b, 0x43, 0x53, 0x35, 0x5f, 0x50, 0x42, 0x4b, 0x44, 0x46, 0x32,
    0x5f, 0x48, 0x4d, 0x41, 0x43, 0x5f, 0x53, 0x48, 0x41, 0x31, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x19, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x29, 0x02, 0x1a, 0x1a, 0x2d, 0x20, 0x53, 0x61, 0x6c, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f,
    0x72, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x18, 0x19, 0x0a, 0x60, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x1d, 0x1a, 0x53, 0x20, 0x52, 0x6f, 0x75, 0x6e, 0x64,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x68, 0x61, 0x73, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x2c, 0x1b, 0x1c, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2e,
    0x02, 0x23, 0x1a, 0x3e, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c,
    0x66, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x21, 0x22, 0x0a, 0x56, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x30, 0x02, 0x25, 0x1a, 0x49, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x61, 0x73, 0x20, 0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x65, 0x64, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x2d, 0x20, 0x73,
    0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x23, 0x24, 0x0a, 0x7a, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x33, 0x02, 0x24, 0x1a, 0x6d, 0x20, 0x48, 0x4d, 0x41, 0x43, 0x20, 0x6f,
    0x66, 0x20, 0x6b, 0x65, 0x79, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x61, 0x73, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x65,
    0x64, 0x20, 0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x72, 0x72,
    0x65, 0x63, 0x74, 0x6c, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x33,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x33, 0x11, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x33, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x36, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x36, 0x08, 0x13, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x39, 0x02, 0x39, 0x1a, 0x2d, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x63, 0x68,
    0x75, 0x6e, 0x6b, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b,
    0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x39, 0x25, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x39, 0x26, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x39, 0x26, 0x30, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x39, 0x26, 0x30, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39,
    0x26, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x39, 0x33, 0x37, 0x0a, 0xea, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x3e,
    0x02, 0x42, 0x1a, 0xdc, 0x01, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20,
    0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x2e,
    0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x64, 0x2c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x20, 0x41, 0x6e, 0x79, 0x20, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x74, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x74,
    0x0a, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20,
    0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x2c, 0x20, 0x73, 0x6f,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x62, 0x65, 0x20, 0x73, 0x6d, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e,
    0x0a, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x69, 0x7a, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x12, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x3e, 0x2e, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x3e, 0x2f, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x3e, 0x2f, 0x39, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x2f, 0x39, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x2f,
    0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x3e, 0x3c, 0x40, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x40, 0x02, 0x30,
    0x1a, 0x38, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x2f, 0x64, 0x65, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x6c, 0x6c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x40, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x40, 0x1d, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40,
    0x2e, 0x2f, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x42, 0x02, 0x57, 0x1a,
    0x25, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x62, 0x75,
    0x6e, 0x64, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x42,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x42, 0x12, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x42, 0x2f, 0x30, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x08, 0x12, 0x03, 0x42, 0x31, 0x56, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x07, 0x12, 0x03, 0x42, 0x3c, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x42, 0x44, 0x55, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x42, 0x44, 0x4e, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x01, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x42, 0x44, 0x4e,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x42, 0x44, 0x4e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x42, 0x51, 0x55, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x45, 0x00,
    0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x45, 0x08, 0x16, 0x0a, 0x3e,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x48, 0x02, 0x36, 0x1a, 0x31, 0x20, 0x43, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x4c, 0x5a, 0x4d, 0x41, 0x2d, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x48, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x48, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x48, 0x28, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x07, 0x12, 0x03, 0x48,
    0x33, 0x34, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x4b, 0x00, 0x4f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x17, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x4e, 0x02, 0x31, 0x1a, 0x2d, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75,
    0x6d, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x68, 0x75, 0x6e, 0x6b, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4e,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x1d, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4e, 0x1f, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x07, 0x12, 0x03, 0x4e, 0x2a, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x51, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x51,
    0x08, 0x18, 0x0a, 0xea, 0x01, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x57, 0x02, 0x3c,
    0x1a, 0xdc, 0x01, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x62, 0x75,
    0x6e, 0x64, 0x6c, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x2e, 0x20, 0x4f,
    0x6e, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x64, 0x2c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e,
    0x20, 0x41, 0x6e, 0x79, 0x20, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x74, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x74, 0x0a, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x73, 0x69,
    0x6e, 0x67, 0x6c, 0x65, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x6d, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x0a, 0x20,
    0x63, 0x68, 0x75, 0x6e, 0x6b, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x57, 0x27, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x07, 0x12, 0x03,
    0x57, 0x32, 0x3a, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x59, 0x02, 0x3c,
    0x1a, 0x24, 0x20, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6d,
    0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x62, 0x75,
    0x6e, 0x64, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x59,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x59, 0x12, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x59, 0x27, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x59, 0x29, 0x3b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x07, 0x12, 0x03, 0x59, 0x34, 0x3a, 0x0a, 0x46, 0x0a, 0x02, 0x04, 0x05,
    0x12, 0x04, 0x5d, 0x00, 0x62, 0x01, 0x1a, 0x3a, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x68, 0x61,
    0x76, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x5f, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5f, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x5f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x60, 0x02, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x60, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x60, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x60, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x60, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x61, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x61, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x61, 0x0b,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x61, 0x1a, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x61, 0x21, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x64, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x64, 0x08, 0x1b, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x67,
    0x02, 0x21, 0x1a, 0x15, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x67, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x67, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x67, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x6a, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x12, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x07, 0x03,
    0x00, 0x12, 0x04, 0x6d, 0x02, 0x73, 0x03, 0x1a, 0x22, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61,
    0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x63, 0x68,
    0x75, 0x6e, 0x6b, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x03, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x0a, 0x15, 0x0a, 0x20, 0x0a, 0x06, 0x04, 0x07, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x1a, 0x1a, 0x11, 0x20, 0x49, 0x64, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x13, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x18, 0x19, 0x0a, 0x22, 0x0a, 0x06, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x72, 0x04, 0x1d, 0x1a, 0x13, 0x20, 0x53, 0x69, 0x7a,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x72, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x14, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x1b, 0x1c, 0x0a,
    0x2a, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02, 0x28, 0x1a, 0x1d, 0x20, 0x41,
    0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x68, 0x75,
    0x6e, 0x6b, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x76, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x76, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x76, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x79, 0x00, 0x7d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x79, 0x08, 0x12, 0x0a, 0x22, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x02, 0x1e, 0x1a, 0x15, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7c, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x7c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x05, 0x7f,
    0x00, 0x89, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x18,
    0x0a, 0x23, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x02, 0x1e, 0x1a, 0x15,
    0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01,
    0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1c,
    0x1d, 0x0a, 0x87, 0x02, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x02, 0x3c,
    0x1a, 0xf8, 0x01, 0x20, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c,
    0x65, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x74, 0x72, 0x79, 0x20, 0x4c, 0x5a, 0x4d, 0x41, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x4c, 0x5a, 0x4d, 0x41, 0x2c, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x6e, 0x27, 0x74, 0x2c, 0x20, 0x69, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65,
    0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e,
    0x20, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x68,
    0x69, 0x67, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x63, 0x61,
    0x6e, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x12, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x88, 0x01, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x08,
    0x12, 0x04, 0x88, 0x01, 0x29, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x07, 0x12,
    0x04, 0x88, 0x01, 0x34, 0x3a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x8b, 0x01, 0x00,
    0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x19,
    0x0a, 0x68, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x18, 0x1a, 0x5a,
    0x20, 0x49, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x75, 0x6e, 0x64, 0x6c,
    0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f,
    0x74, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x11, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8f, 0x01, 0x16, 0x17, 0x0a, 0xe7, 0x01, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x94,
    0x01, 0x00, 0x9e, 0x01, 0x01, 0x1a, 0x73, 0x20, 0x41, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65,
    0x20, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x42, 0x61,
    0x63, 0x6b, 0x75, 0x70, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6d, 0x61, 0x64, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x6f, 0x73, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2c, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x61, 0x72, 0x65, 0x20, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x61, 0x66, 0x74, 0x65,
    0x72, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x22, 0x64, 0x20, 0x42, 0x6f, 0x74,
    0x68, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x69, 0x6d, 0x75, 0x6c, 0x74, 0x61, 0x6e, 0x65, 0x6f, 0x75,
    0x73, 0x6c, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x79, 0x20, 0x61, 0x72, 0x65, 0x20, 0x65, 0x76,
    0x61, 0x6c, 0x75, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x61, 0x6d, 0x65, 0x0a, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x19, 0x0a, 0x55, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x9a, 0x01, 0x02, 0x23, 0x1a, 0x47, 0x20, 0x49, 0x66,
    0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x68,
    0x75, 0x6e, 0x6b, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x64,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66,
    0x6c, 0x6f, 0x77, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9a,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9a, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x11,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x21, 0x22,
    0x0a, 0x60, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x23, 0x1a, 0x52,
    0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x6c, 0x6f,
    0x77, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xa0, 0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0c, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x12, 0x0a, 0x8f, 0x04, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x21, 0x1a, 0x80, 0x04, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x53, 0x69, 0x6e,
    0x63, 0x65, 0x20, 0x75, 0x73, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x71, 0x75, 0x69, 0x74, 0x65, 0x20, 0x6c, 0x61,
    0x72, 0x67, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x6c, 0x69, 0x66,
    0x65, 0x0a, 0x2f, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x73, 0x2c, 0x20, 0x77, 0x65, 0x20,
    0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x20, 0x69, 0x74, 0x73, 0x20, 0x73, 0x65, 0x72, 0x69,
    0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70,
    0x20, 0x61, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x0a, 0x20, 0x69, 0x74, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x69, 0x74,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x73, 0x68, 0x72, 0x69, 0x6e, 0x6b, 0x2e,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20,
    0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x69, 0x74,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x3d, 0x20, 0x30, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x0a, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x27, 0x73,
    0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x49, 0x66,
    0x20, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x3d, 0x20, 0x31, 0x2c,
    0x20, 0x69, 0x74, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x20, 0x75, 0x70, 0x20, 0x42, 0x61,
    0x63, 0x6b, 0x75, 0x70, 0x44, 0x61, 0x74, 0x61, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x77,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x27, 0x73, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x64,
    0x20, 0x75, 0x70, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x6f, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x69,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x2c, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x73, 0x6f, 0x20, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x69, 0x73, 0x20, 0x27, 0x62, 0x79, 0x74, 0x65, 0x73, 0x27, 0x20, 0x61,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xaa, 0x01, 0x1f, 0x20, 0x0a, 0x94, 0x01, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01,
    0x12, 0x04, 0xae, 0x01, 0x02, 0x2f, 0x1a, 0x85, 0x01, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70,
    0x5f, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x72, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x27, 0x72, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x27, 0x20, 0x61, 0x6c, 0x67, 0x6f,
    0x72, 0x69, 0x74, 0x68, 0x6d, 0x0a, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x77, 0x65,
    0x20, 0x67, 0x65, 0x74, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x20, 0x6e, 0x65, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x75, 0x73, 0x65, 0x72, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xae, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xae, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x08, 0x12, 0x04, 0xae, 0x01, 0x21, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x07, 0x12, 0x04, 0xae, 0x01, 0x2c, 0x2d, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02,
    0x12, 0x04, 0xb1, 0x01, 0x02, 0x1b, 0x1a, 0x24, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20,
    0x6f, 0x66, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x19, 0x1a, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03,
    0x12, 0x04, 0xb4, 0x01, 0x02, 0x1c, 0x1a, 0x1e, 0x20, 0x53, 0x48, 0x41, 0x2d, 0x32, 0x35, 0x36,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xb4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xb4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb4,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb4, 0x01,
    0x1a, 0x1b, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x1a,
    0x1a, 0x2c, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x20, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75,
    0x70, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x04, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x18, 0x19,
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
