// @generated
impl serde::Serialize for BlockTracking {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txs.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.tracking.v1.BlockTracking", len)?;
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockTracking {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["txs"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txs,
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
                            "txs" => Ok(GeneratedField::Txs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockTracking;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.BlockTracking")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockTracking, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut txs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BlockTracking {
                    txs: txs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.tracking.v1.BlockTracking",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONTRACT_OPERATION_UNSPECIFIED",
            Self::Instantiation => "CONTRACT_OPERATION_INSTANTIATION",
            Self::Execution => "CONTRACT_OPERATION_EXECUTION",
            Self::Query => "CONTRACT_OPERATION_QUERY",
            Self::Migrate => "CONTRACT_OPERATION_MIGRATE",
            Self::Ibc => "CONTRACT_OPERATION_IBC",
            Self::Sudo => "CONTRACT_OPERATION_SUDO",
            Self::Reply => "CONTRACT_OPERATION_REPLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ContractOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTRACT_OPERATION_UNSPECIFIED",
            "CONTRACT_OPERATION_INSTANTIATION",
            "CONTRACT_OPERATION_EXECUTION",
            "CONTRACT_OPERATION_QUERY",
            "CONTRACT_OPERATION_MIGRATE",
            "CONTRACT_OPERATION_IBC",
            "CONTRACT_OPERATION_SUDO",
            "CONTRACT_OPERATION_REPLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ContractOperation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ContractOperation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTRACT_OPERATION_UNSPECIFIED" => Ok(ContractOperation::Unspecified),
                    "CONTRACT_OPERATION_INSTANTIATION" => Ok(ContractOperation::Instantiation),
                    "CONTRACT_OPERATION_EXECUTION" => Ok(ContractOperation::Execution),
                    "CONTRACT_OPERATION_QUERY" => Ok(ContractOperation::Query),
                    "CONTRACT_OPERATION_MIGRATE" => Ok(ContractOperation::Migrate),
                    "CONTRACT_OPERATION_IBC" => Ok(ContractOperation::Ibc),
                    "CONTRACT_OPERATION_SUDO" => Ok(ContractOperation::Sudo),
                    "CONTRACT_OPERATION_REPLY" => Ok(ContractOperation::Reply),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContractOperationInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.tx_id != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.operation_type != 0 {
            len += 1;
        }
        if self.vm_gas != 0 {
            len += 1;
        }
        if self.sdk_gas != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.tracking.v1.ContractOperationInfo", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if self.tx_id != 0 {
            struct_ser.serialize_field("txId", ToString::to_string(&self.tx_id).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.operation_type != 0 {
            let v = ContractOperation::from_i32(self.operation_type).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.operation_type))
            })?;
            struct_ser.serialize_field("operationType", &v)?;
        }
        if self.vm_gas != 0 {
            struct_ser.serialize_field("vmGas", ToString::to_string(&self.vm_gas).as_str())?;
        }
        if self.sdk_gas != 0 {
            struct_ser.serialize_field("sdkGas", ToString::to_string(&self.sdk_gas).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractOperationInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "tx_id",
            "txId",
            "contract_address",
            "contractAddress",
            "operation_type",
            "operationType",
            "vm_gas",
            "vmGas",
            "sdk_gas",
            "sdkGas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TxId,
            ContractAddress,
            OperationType,
            VmGas,
            SdkGas,
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
                            "id" => Ok(GeneratedField::Id),
                            "txId" | "tx_id" => Ok(GeneratedField::TxId),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "operationType" | "operation_type" => Ok(GeneratedField::OperationType),
                            "vmGas" | "vm_gas" => Ok(GeneratedField::VmGas),
                            "sdkGas" | "sdk_gas" => Ok(GeneratedField::SdkGas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractOperationInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.ContractOperationInfo")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ContractOperationInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut tx_id__ = None;
                let mut contract_address__ = None;
                let mut operation_type__ = None;
                let mut vm_gas__ = None;
                let mut sdk_gas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxId => {
                            if tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txId"));
                            }
                            tx_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::OperationType => {
                            if operation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationType"));
                            }
                            operation_type__ = Some(map.next_value::<ContractOperation>()? as i32);
                        }
                        GeneratedField::VmGas => {
                            if vm_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmGas"));
                            }
                            vm_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SdkGas => {
                            if sdk_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sdkGas"));
                            }
                            sdk_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ContractOperationInfo {
                    id: id__.unwrap_or_default(),
                    tx_id: tx_id__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    operation_type: operation_type__.unwrap_or_default(),
                    vm_gas: vm_gas__.unwrap_or_default(),
                    sdk_gas: sdk_gas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.tracking.v1.ContractOperationInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tx_info_last_id != 0 {
            len += 1;
        }
        if !self.tx_infos.is_empty() {
            len += 1;
        }
        if self.contract_op_info_last_id != 0 {
            len += 1;
        }
        if !self.contract_op_infos.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.tracking.v1.GenesisState", len)?;
        if self.tx_info_last_id != 0 {
            struct_ser.serialize_field(
                "txInfoLastId",
                ToString::to_string(&self.tx_info_last_id).as_str(),
            )?;
        }
        if !self.tx_infos.is_empty() {
            struct_ser.serialize_field("txInfos", &self.tx_infos)?;
        }
        if self.contract_op_info_last_id != 0 {
            struct_ser.serialize_field(
                "contractOpInfoLastId",
                ToString::to_string(&self.contract_op_info_last_id).as_str(),
            )?;
        }
        if !self.contract_op_infos.is_empty() {
            struct_ser.serialize_field("contractOpInfos", &self.contract_op_infos)?;
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
        const FIELDS: &[&str] = &[
            "tx_info_last_id",
            "txInfoLastId",
            "tx_infos",
            "txInfos",
            "contract_op_info_last_id",
            "contractOpInfoLastId",
            "contract_op_infos",
            "contractOpInfos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxInfoLastId,
            TxInfos,
            ContractOpInfoLastId,
            ContractOpInfos,
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
                            "txInfoLastId" | "tx_info_last_id" => Ok(GeneratedField::TxInfoLastId),
                            "txInfos" | "tx_infos" => Ok(GeneratedField::TxInfos),
                            "contractOpInfoLastId" | "contract_op_info_last_id" => {
                                Ok(GeneratedField::ContractOpInfoLastId)
                            }
                            "contractOpInfos" | "contract_op_infos" => {
                                Ok(GeneratedField::ContractOpInfos)
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
                formatter.write_str("struct archway.tracking.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tx_info_last_id__ = None;
                let mut tx_infos__ = None;
                let mut contract_op_info_last_id__ = None;
                let mut contract_op_infos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TxInfoLastId => {
                            if tx_info_last_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txInfoLastId"));
                            }
                            tx_info_last_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxInfos => {
                            if tx_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txInfos"));
                            }
                            tx_infos__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractOpInfoLastId => {
                            if contract_op_info_last_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractOpInfoLastId",
                                ));
                            }
                            contract_op_info_last_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ContractOpInfos => {
                            if contract_op_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractOpInfos"));
                            }
                            contract_op_infos__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    tx_info_last_id: tx_info_last_id__.unwrap_or_default(),
                    tx_infos: tx_infos__.unwrap_or_default(),
                    contract_op_info_last_id: contract_op_info_last_id__.unwrap_or_default(),
                    contract_op_infos: contract_op_infos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.tracking.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBlockGasTrackingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.tracking.v1.QueryBlockGasTrackingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockGasTrackingRequest {
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
            type Value = QueryBlockGasTrackingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.QueryBlockGasTrackingRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryBlockGasTrackingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBlockGasTrackingRequest {})
            }
        }
        deserializer.deserialize_struct(
            "archway.tracking.v1.QueryBlockGasTrackingRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBlockGasTrackingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.tracking.v1.QueryBlockGasTrackingResponse", len)?;
        if let Some(v) = self.block.as_ref() {
            struct_ser.serialize_field("block", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockGasTrackingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["block"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
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
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBlockGasTrackingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.QueryBlockGasTrackingResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryBlockGasTrackingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBlockGasTrackingResponse { block: block__ })
            }
        }
        deserializer.deserialize_struct(
            "archway.tracking.v1.QueryBlockGasTrackingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TxInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.total_gas != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.tracking.v1.TxInfo", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.total_gas != 0 {
            struct_ser
                .serialize_field("totalGas", ToString::to_string(&self.total_gas).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "height", "total_gas", "totalGas"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Height,
            TotalGas,
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
                            "id" => Ok(GeneratedField::Id),
                            "height" => Ok(GeneratedField::Height),
                            "totalGas" | "total_gas" => Ok(GeneratedField::TotalGas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.TxInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TxInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut height__ = None;
                let mut total_gas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalGas => {
                            if total_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalGas"));
                            }
                            total_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TxInfo {
                    id: id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    total_gas: total_gas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.tracking.v1.TxInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxTracking {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info.is_some() {
            len += 1;
        }
        if !self.contract_operations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.tracking.v1.TxTracking", len)?;
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        if !self.contract_operations.is_empty() {
            struct_ser.serialize_field("contractOperations", &self.contract_operations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxTracking {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["info", "contract_operations", "contractOperations"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
            ContractOperations,
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
                            "info" => Ok(GeneratedField::Info),
                            "contractOperations" | "contract_operations" => {
                                Ok(GeneratedField::ContractOperations)
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
            type Value = TxTracking;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.tracking.v1.TxTracking")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TxTracking, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                let mut contract_operations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map.next_value()?;
                        }
                        GeneratedField::ContractOperations => {
                            if contract_operations__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractOperations",
                                ));
                            }
                            contract_operations__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TxTracking {
                    info: info__,
                    contract_operations: contract_operations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.tracking.v1.TxTracking", FIELDS, GeneratedVisitor)
    }
}
