// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `interchainaccounts/interchainaccounts_tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MsgRegisterInterchainAccount is used to register an account on a remote zone.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccount)
pub struct MsgRegisterInterchainAccount {
    // message fields
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccount.from_address)
    pub from_address: ::std::string::String,
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccount.connection_id)
    pub connection_id: ::std::string::String,
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccount.owner)
    pub owner: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccount.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgRegisterInterchainAccount {
    fn default() -> &'a MsgRegisterInterchainAccount {
        <MsgRegisterInterchainAccount as ::protobuf::Message>::default_instance()
    }
}

impl MsgRegisterInterchainAccount {
    pub fn new() -> MsgRegisterInterchainAccount {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_address",
            |m: &MsgRegisterInterchainAccount| { &m.from_address },
            |m: &mut MsgRegisterInterchainAccount| { &mut m.from_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "connection_id",
            |m: &MsgRegisterInterchainAccount| { &m.connection_id },
            |m: &mut MsgRegisterInterchainAccount| { &mut m.connection_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "owner",
            |m: &MsgRegisterInterchainAccount| { &m.owner },
            |m: &mut MsgRegisterInterchainAccount| { &mut m.owner },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgRegisterInterchainAccount>(
            "MsgRegisterInterchainAccount",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgRegisterInterchainAccount {
    const NAME: &'static str = "MsgRegisterInterchainAccount";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.from_address = is.read_string()?;
                },
                18 => {
                    self.connection_id = is.read_string()?;
                },
                26 => {
                    self.owner = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.from_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.from_address);
        }
        if !self.connection_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.connection_id);
        }
        if !self.owner.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.owner);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.from_address.is_empty() {
            os.write_string(1, &self.from_address)?;
        }
        if !self.connection_id.is_empty() {
            os.write_string(2, &self.connection_id)?;
        }
        if !self.owner.is_empty() {
            os.write_string(3, &self.owner)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgRegisterInterchainAccount {
        MsgRegisterInterchainAccount::new()
    }

    fn clear(&mut self) {
        self.from_address.clear();
        self.connection_id.clear();
        self.owner.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgRegisterInterchainAccount {
        static instance: MsgRegisterInterchainAccount = MsgRegisterInterchainAccount {
            from_address: ::std::string::String::new(),
            connection_id: ::std::string::String::new(),
            owner: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgRegisterInterchainAccount {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgRegisterInterchainAccount").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgRegisterInterchainAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgRegisterInterchainAccount {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgRegisterInterchainAccountResponse is the response type for
///  MsgRegisterInterchainAccount.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccountResponse)
pub struct MsgRegisterInterchainAccountResponse {
    // special fields
    // @@protoc_insertion_point(special_field:neutron.interchainadapter.interchaintxs.v1.MsgRegisterInterchainAccountResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgRegisterInterchainAccountResponse {
    fn default() -> &'a MsgRegisterInterchainAccountResponse {
        <MsgRegisterInterchainAccountResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgRegisterInterchainAccountResponse {
    pub fn new() -> MsgRegisterInterchainAccountResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgRegisterInterchainAccountResponse>(
            "MsgRegisterInterchainAccountResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgRegisterInterchainAccountResponse {
    const NAME: &'static str = "MsgRegisterInterchainAccountResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgRegisterInterchainAccountResponse {
        MsgRegisterInterchainAccountResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgRegisterInterchainAccountResponse {
        static instance: MsgRegisterInterchainAccountResponse = MsgRegisterInterchainAccountResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgRegisterInterchainAccountResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgRegisterInterchainAccountResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgRegisterInterchainAccountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgRegisterInterchainAccountResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgSubmitTx defines the payload for Msg/SubmitTx
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx)
pub struct MsgSubmitTx {
    // message fields
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx.from_address)
    pub from_address: ::std::string::String,
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx.owner)
    pub owner: ::std::string::String,
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx.connection_id)
    pub connection_id: ::std::string::String,
    // @@protoc_insertion_point(field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx.msgs)
    pub msgs: ::std::vec::Vec<::protobuf::well_known_types::any::Any>,
    // special fields
    // @@protoc_insertion_point(special_field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTx.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgSubmitTx {
    fn default() -> &'a MsgSubmitTx {
        <MsgSubmitTx as ::protobuf::Message>::default_instance()
    }
}

impl MsgSubmitTx {
    pub fn new() -> MsgSubmitTx {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_address",
            |m: &MsgSubmitTx| { &m.from_address },
            |m: &mut MsgSubmitTx| { &mut m.from_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "owner",
            |m: &MsgSubmitTx| { &m.owner },
            |m: &mut MsgSubmitTx| { &mut m.owner },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "connection_id",
            |m: &MsgSubmitTx| { &m.connection_id },
            |m: &mut MsgSubmitTx| { &mut m.connection_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "msgs",
            |m: &MsgSubmitTx| { &m.msgs },
            |m: &mut MsgSubmitTx| { &mut m.msgs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgSubmitTx>(
            "MsgSubmitTx",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgSubmitTx {
    const NAME: &'static str = "MsgSubmitTx";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.from_address = is.read_string()?;
                },
                18 => {
                    self.owner = is.read_string()?;
                },
                26 => {
                    self.connection_id = is.read_string()?;
                },
                34 => {
                    self.msgs.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.from_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.from_address);
        }
        if !self.owner.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.owner);
        }
        if !self.connection_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.connection_id);
        }
        for value in &self.msgs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.from_address.is_empty() {
            os.write_string(1, &self.from_address)?;
        }
        if !self.owner.is_empty() {
            os.write_string(2, &self.owner)?;
        }
        if !self.connection_id.is_empty() {
            os.write_string(3, &self.connection_id)?;
        }
        for v in &self.msgs {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgSubmitTx {
        MsgSubmitTx::new()
    }

    fn clear(&mut self) {
        self.from_address.clear();
        self.owner.clear();
        self.connection_id.clear();
        self.msgs.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgSubmitTx {
        static instance: MsgSubmitTx = MsgSubmitTx {
            from_address: ::std::string::String::new(),
            owner: ::std::string::String::new(),
            connection_id: ::std::string::String::new(),
            msgs: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgSubmitTx {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgSubmitTx").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgSubmitTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgSubmitTx {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTxResponse)
pub struct MsgSubmitTxResponse {
    // special fields
    // @@protoc_insertion_point(special_field:neutron.interchainadapter.interchaintxs.v1.MsgSubmitTxResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgSubmitTxResponse {
    fn default() -> &'a MsgSubmitTxResponse {
        <MsgSubmitTxResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgSubmitTxResponse {
    pub fn new() -> MsgSubmitTxResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgSubmitTxResponse>(
            "MsgSubmitTxResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgSubmitTxResponse {
    const NAME: &'static str = "MsgSubmitTxResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgSubmitTxResponse {
        MsgSubmitTxResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgSubmitTxResponse {
        static instance: MsgSubmitTxResponse = MsgSubmitTxResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgSubmitTxResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgSubmitTxResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgSubmitTxResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgSubmitTxResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.interchainaccounts/interchainaccounts_tx.proto\x12*neutron.interchain\
    adapter.interchaintxs.v1\x1a\x19google/protobuf/any.proto\"|\n\x1cMsgReg\
    isterInterchainAccount\x12!\n\x0cfrom_address\x18\x01\x20\x01(\tR\x0bfro\
    mAddress\x12#\n\rconnection_id\x18\x02\x20\x01(\tR\x0cconnectionId\x12\
    \x14\n\x05owner\x18\x03\x20\x01(\tR\x05owner\"&\n$MsgRegisterInterchainA\
    ccountResponse\"\x95\x01\n\x0bMsgSubmitTx\x12!\n\x0cfrom_address\x18\x01\
    \x20\x01(\tR\x0bfromAddress\x12\x14\n\x05owner\x18\x02\x20\x01(\tR\x05ow\
    ner\x12#\n\rconnection_id\x18\x03\x20\x01(\tR\x0cconnectionId\x12(\n\x04\
    msgs\x18\x04\x20\x03(\x0b2\x14.google.protobuf.AnyR\x04msgs\"\x15\n\x13M\
    sgSubmitTxResponseB=Z;github.com/neutron/interchain-adapter/x/interchain\
    txs/typesJ\xa5\x07\n\x06\x12\x04\0\0\x1c\x1e\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x01\03\n\x08\n\x01\x08\x12\x03\x03\0R\n\t\n\
    \x02\x08\x0b\x12\x03\x03\0R\n\t\n\x02\x03\0\x12\x03\x06\0#\n[\n\x02\x04\
    \0\x12\x04\t\0\r\x01\x1aO\x20MsgRegisterInterchainAccount\x20is\x20used\
    \x20to\x20register\x20an\x20account\x20on\x20a\x20remote\x20zone.\n\n\n\
    \n\x03\x04\0\x01\x12\x03\t\x08$\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02\
    \x1a\n\r\n\x05\x04\0\x02\0\x04\x12\x04\n\x02\t&\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\t\x15\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\n\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x0b\x02\x1b\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x02\n\x1a\n\x0c\
    \n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x0b\t\x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x19\x1a\
    \n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x02\x13\n\r\n\x05\x04\0\x02\x02\
    \x04\x12\x04\x0c\x02\x0b\x1b\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\t\x0e\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03\x0c\x11\x12\ni\n\x02\x04\x01\x12\x03\x11\0/\x1a^\
    \x20MsgRegisterInterchainAccountResponse\x20is\x20the\x20response\x20typ\
    e\x20for\n\x20MsgRegisterInterchainAccount.\n\n\n\n\x03\x04\x01\x01\x12\
    \x03\x11\x08,\n>\n\x02\x04\x02\x12\x04\x14\0\x19\x01\x1a2\x20MsgSubmitTx\
    \x20defines\x20the\x20payload\x20for\x20Msg/SubmitTx\n\n\n\n\x03\x04\x02\
    \x01\x12\x03\x14\x08\x13\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x15\x02\x1a\n\
    \r\n\x05\x04\x02\x02\0\x04\x12\x04\x15\x02\x14\x15\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x15\
    \t\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x15\x18\x19\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x16\x02\x13\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    \x16\x02\x15\x1a\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x16\x02\x08\n\
    \x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x16\t\x0e\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x03\x16\x11\x12\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x17\x02\
    \x1b\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x17\x02\x16\x13\n\x0c\n\x05\
    \x04\x02\x02\x02\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x17\t\x16\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x17\x19\x1a\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03\x18\x02(\n\x0c\n\x05\x04\x02\x02\x03\
    \x04\x12\x03\x18\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03\x18\x0b\
    \x1e\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x18\x1f#\n\x0c\n\x05\x04\
    \x02\x02\x03\x03\x12\x03\x18&'\nF\n\x02\x04\x03\x12\x03\x1c\0\x1e\x1a;\
    \x20MsgSubmitTxResponse\x20defines\x20the\x20response\x20for\x20Msg/Subm\
    itTx\n\n\n\n\x03\x04\x03\x01\x12\x03\x1c\x08\x1bb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::well_known_types::any::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(MsgRegisterInterchainAccount::generated_message_descriptor_data());
            messages.push(MsgRegisterInterchainAccountResponse::generated_message_descriptor_data());
            messages.push(MsgSubmitTx::generated_message_descriptor_data());
            messages.push(MsgSubmitTxResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
