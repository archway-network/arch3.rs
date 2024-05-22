// @generated
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.granting_contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.cwfees.v1.GenesisState", len)?;
        if !self.granting_contracts.is_empty() {
            struct_ser.serialize_field("grantingContracts", &self.granting_contracts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["granting_contracts", "grantingContracts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantingContracts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "grantingContracts" | "granting_contracts" => {
                                Ok(GeneratedField::GrantingContracts)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut granting_contracts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrantingContracts => {
                            if granting_contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantingContracts"));
                            }
                            granting_contracts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    granting_contracts: granting_contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.cwfees.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsGrantingContractRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.IsGrantingContractRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsGrantingContractRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsGrantingContractRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.IsGrantingContractRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<IsGrantingContractRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IsGrantingContractRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.IsGrantingContractRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for IsGrantingContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_granting_contract {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.IsGrantingContractResponse", len)?;
        if self.is_granting_contract {
            struct_ser.serialize_field("isGrantingContract", &self.is_granting_contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsGrantingContractResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["is_granting_contract", "isGrantingContract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsGrantingContract,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "isGrantingContract" | "is_granting_contract" => {
                                Ok(GeneratedField::IsGrantingContract)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsGrantingContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.IsGrantingContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<IsGrantingContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut is_granting_contract__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IsGrantingContract => {
                            if is_granting_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "isGrantingContract",
                                ));
                            }
                            is_granting_contract__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IsGrantingContractResponse {
                    is_granting_contract: is_granting_contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.IsGrantingContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRegisterAsGranter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.granting_contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.MsgRegisterAsGranter", len)?;
        if !self.granting_contract.is_empty() {
            struct_ser.serialize_field("grantingContract", &self.granting_contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterAsGranter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["granting_contract", "grantingContract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantingContract,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "grantingContract" | "granting_contract" => {
                                Ok(GeneratedField::GrantingContract)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterAsGranter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.MsgRegisterAsGranter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRegisterAsGranter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut granting_contract__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrantingContract => {
                            if granting_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantingContract"));
                            }
                            granting_contract__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRegisterAsGranter {
                    granting_contract: granting_contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.MsgRegisterAsGranter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRegisterAsGranterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.MsgRegisterAsGranterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterAsGranterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterAsGranterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.MsgRegisterAsGranterResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRegisterAsGranterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRegisterAsGranterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.MsgRegisterAsGranterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnregisterAsGranter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.granting_contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.MsgUnregisterAsGranter", len)?;
        if !self.granting_contract.is_empty() {
            struct_ser.serialize_field("grantingContract", &self.granting_contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnregisterAsGranter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["granting_contract", "grantingContract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantingContract,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "grantingContract" | "granting_contract" => {
                                Ok(GeneratedField::GrantingContract)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnregisterAsGranter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.MsgUnregisterAsGranter")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnregisterAsGranter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut granting_contract__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrantingContract => {
                            if granting_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantingContract"));
                            }
                            granting_contract__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUnregisterAsGranter {
                    granting_contract: granting_contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.MsgUnregisterAsGranter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnregisterAsGranterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.cwfees.v1.MsgUnregisterAsGranterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnregisterAsGranterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnregisterAsGranterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwfees.v1.MsgUnregisterAsGranterResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnregisterAsGranterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnregisterAsGranterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.cwfees.v1.MsgUnregisterAsGranterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
