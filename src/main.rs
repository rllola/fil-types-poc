use serde::{Serialize, Deserialize};
use fil_original::{AddSignerParams, ChangeNumApprovalsThresholdParams};
use fvm_shared::encoding::RawBytes;
use fvm_shared::address::Address;

mod fil_original {
    use fvm_shared::address::Address;
    use fvm_shared::encoding::tuple::*;

    #[derive(Debug, Serialize_tuple, Deserialize_tuple)]
    pub struct AddSignerParams {
        pub signer: Address,
        pub increase: bool,
    }

    #[derive(Debug, Serialize_tuple, Deserialize_tuple)]
    pub struct ChangeNumApprovalsThresholdParams {
        pub new_threshold: u64,
    }

}

mod address_json {
    use fvm_shared::address::Address;
    use serde::{de, Serializer, Deserializer, Deserialize};
    use std::str::FromStr;

    pub fn serialize<S>(
        address: &Address,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = address.to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Address::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "AddSignerParams")]
pub struct AddSignerParamsAPI {
    #[serde(with = "address_json")]
    pub signer: Address,
    pub increase: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "ChangeNumApprovalsThresholdParams")]
pub struct ChangeNumApprovalsThresholdParamsAPI {
    pub new_threshold: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamsAPI {
    #[serde(with = "AddSignerParamsAPI")]
    AddSignerParamsAPI(AddSignerParams),
    #[serde(with = "ChangeNumApprovalsThresholdParamsAPI")]
    ChangeNumApprovalsThresholdParamsAPI(ChangeNumApprovalsThresholdParams),
}

fn main() {

    let foo_json = r#"{ "signer": "t01", "increase": true }"#;
    let foo2 : ParamsAPI = serde_json::from_str(foo_json).unwrap();
    println!("{:?}", foo2);

    let foo3 = match foo2 {
        ParamsAPI::AddSignerParamsAPI(params) => {
            RawBytes::serialize(params).unwrap()
        },
        _ => {
            println!("something went wrong");
            RawBytes::new(vec!())
        }
    };
    println!("{:?}", hex::encode(foo3.bytes()));

    let foo4 : AddSignerParams = RawBytes::deserialize(&foo3).unwrap();
    println!("{:?}", foo4);

    let foo = ParamsAPI::AddSignerParamsAPI(foo4);
    let foo5 = serde_json::to_string(&foo).unwrap();
    println!("{:?}", foo5);
}
