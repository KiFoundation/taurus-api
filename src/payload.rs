extern crate quick_protobuf;

use base64::engine::general_purpose;
use base64::Engine;
use quick_protobuf::Writer;
use serde::Serialize;
use std::borrow::Cow;
use std::str;

pub const KIND_STRING: &str = "String";
pub const KIND_ARRAY: &str = "Array";
pub const KIND_MESSAGE: &str = "Message";
pub const KIND_BYTES: &str = "Bytes";
pub const KIND_INT: &str = "Int";
pub const KIND_COIN: &str = "Coin";

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StringObject {
    pub string: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayDetail {
    pub kind: String,
    pub elems: Vec<Objects>,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayObject {
    pub array: ArrayDetail,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageObject {
    pub message: Vec<MesssageDetails>,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BytesObject {
    pub bytes: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntObject {
    pub int: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoinObject {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum Objects {
    String(StringObject),
    Array(ArrayObject),
    Message(MessageObject),
    Bytes(BytesObject),
    Int(IntObject),
    Coin(CoinObject),
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MesssageDetails {
    pub kind: String,
    pub name: String,
    pub field_id: u32,
    pub value: Box<Objects>,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub url: String,
    pub message: Vec<MesssageDetails>,
}

impl Message {
    fn encode_proto_string(data: &str) -> String {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&crate::proto::authz::Authz {
                msg: Cow::Borrowed(data),
            })
            .expect("encode_proto_string error");

        let out2 = &out.as_slice()[1..];

        general_purpose::STANDARD.encode(out2)
    }

    pub fn build_authz_message(
        granter: String,
        grantee: String,
        authorization: String,
        _expiration: String,
    ) -> Message {
        Message {
            url: "/cosmos.authz.v1beta1.MsgGrant".to_string(),
            message: vec![
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "granter".to_string(),
                    field_id: 1,
                    value: Box::new(Objects::String(StringObject { string: granter })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "grantee".to_string(),
                    field_id: 2,
                    value: Box::new(Objects::String(StringObject { string: grantee })),
                },
                MesssageDetails {
                    kind: KIND_MESSAGE.to_string(),
                    name: "grant".to_string(),
                    field_id: 3,
                    value: Box::new(Objects::Message(MessageObject {
                        message: vec![
                            MesssageDetails {
                                kind: KIND_MESSAGE.to_string(),
                                name: "authorization".to_string(),
                                field_id: 1,
                                value: Box::new(Objects::Message(MessageObject {
                                    message: vec![
                                        MesssageDetails {
                                            kind: "String".to_string(),
                                            name: "type_url".to_string(),
                                            field_id: 1,
                                            value: Box::new(Objects::String(StringObject {
                                                string:
                                                    "/cosmos.authz.v1beta1.GenericAuthorization"
                                                        .to_string(),
                                            })),
                                        },
                                        MesssageDetails {
                                            kind: KIND_BYTES.to_string(),
                                            name: "value".to_string(),
                                            field_id: 2,
                                            value: Box::new(Objects::Bytes(BytesObject {
                                                bytes: Self::encode_proto_string(
                                                    authorization.as_str(),
                                                ),
                                            })),
                                        },
                                    ],
                                })),
                            },
                            MesssageDetails {
                                kind: KIND_MESSAGE.to_string(),
                                name: "timestamp".to_string(),
                                field_id: 2,
                                value: Box::new(Objects::Message(MessageObject {
                                    message: vec![
                                        MesssageDetails {
                                            kind: KIND_INT.to_string(),
                                            name: "seconds".to_string(),
                                            field_id: 1,
                                            value: Box::new(Objects::Int(IntObject {
                                                int: "1732193243".to_string(),
                                            })),
                                        },
                                        MesssageDetails {
                                            kind: KIND_INT.to_string(),
                                            name: "nanos".to_string(),
                                            field_id: 2,
                                            value: Box::new(Objects::Int(IntObject {
                                                int: "0".to_string(),
                                            })),
                                        },
                                    ],
                                })),
                            },
                        ],
                    })),
                },
            ],
        }
    }

    pub fn build_revoke_message(granter: String, grantee: String, msg_type_url: String) -> Message {
        Message {
            url: "/cosmos.authz.v1beta1.MsgRevoke".to_string(),
            message: vec![
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "granter".to_string(),
                    field_id: 1,
                    value: Box::new(Objects::String(StringObject { string: granter })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "grantee".to_string(),
                    field_id: 2,
                    value: Box::new(Objects::String(StringObject { string: grantee })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "msg_type_url".to_string(),
                    field_id: 3,
                    value: Box::new(Objects::String(StringObject {
                        string: msg_type_url,
                    })),
                },
            ],
        }
    }

    pub fn build_contract_migrate(
        sender: String,
        contract: String,
        code_id: u64,
        msg: String,
    ) -> Message {
        Message {
            url: "/cosmwasm.wasm.v1.MsgMigrateContract".to_string(),
            message: vec![
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "sender".to_string(),
                    field_id: 1,
                    value: Box::new(Objects::String(StringObject { string: sender })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "contract".to_string(),
                    field_id: 2,
                    value: Box::new(Objects::String(StringObject { string: contract })),
                },
                MesssageDetails {
                    kind: KIND_INT.to_string(),
                    name: "code_id".to_string(),
                    field_id: 3,
                    value: Box::new(Objects::Int(IntObject {
                        int: code_id.to_string(),
                    })),
                },
                MesssageDetails {
                    kind: KIND_BYTES.to_string(),
                    name: "msg".to_string(),
                    field_id: 4,
                    value: Box::new(Objects::Bytes(BytesObject { bytes: msg })),
                },
            ],
        }
    }

    pub fn build_contract_execute(
        sender: String,
        contract: String,
        msg: String,
        funds: Vec<CoinObject>,
    ) -> Message {
        Message {
            url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
            message: vec![
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "sender".to_string(),
                    field_id: 1,
                    value: Box::new(Objects::String(StringObject { string: sender })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "contract".to_string(),
                    field_id: 2,
                    value: Box::new(Objects::String(StringObject { string: contract })),
                },
                MesssageDetails {
                    kind: KIND_BYTES.to_string(),
                    name: "msg".to_string(),
                    field_id: 3,
                    value: Box::new(Objects::Bytes(BytesObject { bytes: msg })),
                },
                MesssageDetails {
                    kind: KIND_ARRAY.to_string(),
                    name: "funds".to_string(),
                    field_id: 5,
                    // funds is an array of Coin objects
                    value: Box::new(Objects::Array(ArrayObject {
                        array: ArrayDetail {
                            kind: KIND_ARRAY.to_string(),
                            elems: funds
                                .into_iter()
                                .map(|coin| {
                                    Objects::Coin(CoinObject {
                                        denom: coin.denom,
                                        amount: coin.amount,
                                    })
                                })
                                .collect(),
                        },
                    })),
                },
            ],
        }
    }

    pub fn build_contract_instantiate(
        sender: String,
        admin: String,
        label: String,
        code_id: u64,
        msg: String,
    ) -> Message {
        Message {
            url: "/cosmwasm.wasm.v1.MsgInstantiateContract".to_string(),
            message: vec![
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "sender".to_string(),
                    field_id: 1,
                    value: Box::new(Objects::String(StringObject { string: sender })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "admin".to_string(),
                    field_id: 2,
                    value: Box::new(Objects::String(StringObject { string: admin })),
                },
                MesssageDetails {
                    kind: KIND_INT.to_string(),
                    name: "code_id".to_string(),
                    field_id: 3,
                    value: Box::new(Objects::Int(IntObject {
                        int: code_id.to_string(),
                    })),
                },
                MesssageDetails {
                    kind: KIND_STRING.to_string(),
                    name: "label".to_string(),
                    field_id: 4,
                    value: Box::new(Objects::String(StringObject { string: label })),
                },
                MesssageDetails {
                    kind: KIND_BYTES.to_string(),
                    name: "msg".to_string(),
                    field_id: 5,
                    value: Box::new(Objects::Bytes(BytesObject { bytes: msg })),
                },
            ],
        }
    }
}
