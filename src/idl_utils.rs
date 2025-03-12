/// Utility functions for converting between different Anchor IDL formats
/// This module provides tools to convert the newer Anchor IDL format to the older format.
use serde::{
  Deserialize,
  Serialize,
};

/// The AnchorIDLNew struct represents the new IDL format.
/// This struct is used to deserialize the new IDL format from a JSON file.
#[derive(Deserialize, Clone)]
pub struct AnchorIDLNew {
  metadata: Metadata,
  instructions: Vec<Instruction>,
  accounts: Vec<Account>,
  types: Vec<Type>,
  address: String,
}

/// The old IDL format is represented by the AnchorIDLOld struct.
/// This struct is used to serialize the old IDL format to a JSON file.
#[derive(Serialize)]
pub struct AnchorIDLOld {
  version: String,
  name: String,
  instructions: Vec<InstructionOld>,
  accounts: Vec<AccountOld>,
  types: Vec<TypeOld>,
  errors: Vec<Error>,
  metadata: MetadataOld,
}

/// The Metadata struct represents the metadata field in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Metadata {
  version: String,
  name: String,
}

/// This Instruction struct represents an instruction in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Instruction {
  name: String,
  accounts: Vec<Account>,
  args: Vec<Arg>,
}

/// This Account struct represents an account in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Account {
  name: String,
  writable: Option<bool>,
  signer: Option<bool>,
}

/// This Arg struct represents an argument in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Arg {
  name: String,
  #[serde(rename = "type")]
  type_: Type,
}

/// This Type struct represents a type in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Type {
  kind: String,
  fields: Option<Vec<Field>>,
  variants: Option<Vec<Variant>>,
  defined: Option<Box<Type>>,
  length: Option<usize>,
  option: Option<Box<Type>>,
}

/// This Field struct represents a field in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Field {
  name: String,
  #[serde(rename = "type")]
  type_: Type,
}

/// This Variant struct represents a variant in the new IDL format.
#[derive(Deserialize, Clone)]
pub struct Variant {
  name: String,
}

/// This InstructionOld struct represents an instruction in the old IDL format.
#[derive(Serialize)]
pub struct InstructionOld {
  name: String,
  accounts: Vec<AccountOld>,
  args: Vec<ArgOld>,
}

/// This AccountOld struct represents an account in the old IDL format.
#[derive(Serialize)]
pub struct AccountOld {
  name: String,
  is_mut: bool,
  is_signer: bool,
}

/// This ArgOld struct represents an argument in the old IDL format.
#[derive(Serialize)]
pub struct ArgOld {
  name: String,
  #[serde(rename = "type")]
  type_: TypeOld,
}

/// This TypeOld struct represents a type in the old IDL format.
#[derive(Default, Serialize)]
pub struct TypeOld {
  kind: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  fields: Option<Vec<FieldOld>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  variants: Option<Vec<VariantOld>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  defined: Option<Box<TypeOld>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  length: Option<usize>,
}

/// This FieldOld struct represents a field in the old IDL format.
#[derive(Default, Serialize)]
pub struct FieldOld {
  name: String,
  #[serde(rename = "type")]
  type_: TypeOld,
}

/// This VariantOld struct represents a variant in the old IDL format.
#[derive(Default, Serialize)]
pub struct VariantOld {
  name: String,
}

/// This Error struct represents an error in the old IDL format.
#[derive(Serialize)]
pub struct Error {
  code: u32,
  name: String,
  msg: String,
}

/// This MetadataOld struct represents the metadata field in the old IDL format.
#[derive(Serialize)]
pub struct MetadataOld {
  address: String,
}

/// Converts a snake_case string to camelCase.
/// This is used to convert the snake_case fields in the new IDL to camelCase
/// fields in the old IDL.
fn convert_snake_case_to_camel_case(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut uppercase_next = false;

  for c in s.chars() {
    if c == '_' {
      uppercase_next = true;
    } else if uppercase_next {
      result.push(c.to_ascii_uppercase());
      uppercase_next = false;
    } else {
      result.push(c);
    }
  }
  result
}

fn convert_type(type_: &Type) -> TypeOld {
  match type_.kind.as_str() {
    "struct" => TypeOld {
      kind: "struct".to_string(),
      fields: type_.fields.as_ref().map(|fields| {
        fields
          .iter()
          .map(|field| FieldOld {
            name: convert_snake_case_to_camel_case(&field.name),
            type_: convert_type(&field.type_),
          })
          .collect()
      }),
      ..Default::default()
    },
    "enum" => TypeOld {
      kind: "enum".to_string(),
      variants: type_.variants.as_ref().map(|variants| {
        variants
          .iter()
          .map(|variant| VariantOld {
            name: convert_snake_case_to_camel_case(&variant.name),
          })
          .collect()
      }),
      ..Default::default()
    },
    "option" => {
      let defined_type = match &type_.option {
        Some(inner) => inner,
        None => {
          if let Some(defined) = &type_.defined {
            defined
          } else {
            panic!("Option type missing inner type definition")
          }
        },
      };

      TypeOld {
        kind: "option".to_string(),
        defined: Some(Box::new(convert_type(defined_type))),
        ..Default::default()
      }
    },
    "array" => TypeOld {
      kind: "array".to_string(),
      length: type_.length,
      ..Default::default()
    },
    _ => TypeOld {
      kind: type_.kind.clone(),
      ..Default::default()
    },
  }
}

/// Converts an AnchorIDLNew to an AnchorIDLOld.
/// This function is used to convert the new IDL format to the old IDL format.
pub fn anchor_idl_convert_new_to_old(data: AnchorIDLNew) -> AnchorIDLOld {
  AnchorIDLOld {
    version: data.metadata.version.clone(),
    name: data.metadata.name.clone(),
    instructions: data
      .instructions
      .iter()
      .map(|instruction| InstructionOld {
        name: convert_snake_case_to_camel_case(&instruction.name),
        accounts: instruction
          .accounts
          .iter()
          .map(|account| AccountOld {
            name: convert_snake_case_to_camel_case(&account.name),
            is_mut: account.writable.unwrap_or(false),
            is_signer: account.signer.unwrap_or(false),
          })
          .collect(),
        args: instruction
          .args
          .iter()
          .map(|arg| ArgOld {
            name: convert_snake_case_to_camel_case(&arg.name),
            type_: convert_type(&arg.type_),
          })
          .collect(),
      })
      .collect(),
    accounts: data
      .accounts
      .iter()
      .map(|account| AccountOld {
        name: account.name.clone(),
        is_mut: false,
        is_signer: false,
      })
      .collect(),
    types: data.types.iter().map(convert_type).collect(),
    errors: vec![],
    metadata: MetadataOld {
      address: data.address,
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_snake_to_camel_case() {
    assert_eq!(convert_snake_case_to_camel_case("hello_world"), "helloWorld");
    assert_eq!(convert_snake_case_to_camel_case("hello"), "hello");
    assert_eq!(
      convert_snake_case_to_camel_case("hello_world_again"),
      "helloWorldAgain"
    );
  }
}
