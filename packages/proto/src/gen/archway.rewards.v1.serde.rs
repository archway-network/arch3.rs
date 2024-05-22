// @generated
impl serde::Serialize for BlockRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.inflation_rewards.is_some() {
            len += 1;
        }
        if self.max_gas != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.rewards.v1.BlockRewards", len)?;
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.inflation_rewards.as_ref() {
            struct_ser.serialize_field("inflationRewards", v)?;
        }
        if self.max_gas != 0 {
            struct_ser.serialize_field("maxGas", ToString::to_string(&self.max_gas).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "inflation_rewards",
            "inflationRewards",
            "max_gas",
            "maxGas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            InflationRewards,
            MaxGas,
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
                            "height" => Ok(GeneratedField::Height),
                            "inflationRewards" | "inflation_rewards" => {
                                Ok(GeneratedField::InflationRewards)
                            }
                            "maxGas" | "max_gas" => Ok(GeneratedField::MaxGas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.BlockRewards")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut inflation_rewards__ = None;
                let mut max_gas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InflationRewards => {
                            if inflation_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflationRewards"));
                            }
                            inflation_rewards__ = map.next_value()?;
                        }
                        GeneratedField::MaxGas => {
                            if max_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGas"));
                            }
                            max_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BlockRewards {
                    height: height__.unwrap_or_default(),
                    inflation_rewards: inflation_rewards__,
                    max_gas: max_gas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.rewards.v1.BlockRewards", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockTracking {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.inflation_rewards.is_some() {
            len += 1;
        }
        if !self.tx_rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.BlockTracking", len)?;
        if let Some(v) = self.inflation_rewards.as_ref() {
            struct_ser.serialize_field("inflationRewards", v)?;
        }
        if !self.tx_rewards.is_empty() {
            struct_ser.serialize_field("txRewards", &self.tx_rewards)?;
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
        const FIELDS: &[&str] = &[
            "inflation_rewards",
            "inflationRewards",
            "tx_rewards",
            "txRewards",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InflationRewards,
            TxRewards,
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
                            "inflationRewards" | "inflation_rewards" => {
                                Ok(GeneratedField::InflationRewards)
                            }
                            "txRewards" | "tx_rewards" => Ok(GeneratedField::TxRewards),
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
                formatter.write_str("struct archway.rewards.v1.BlockTracking")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockTracking, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut inflation_rewards__ = None;
                let mut tx_rewards__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InflationRewards => {
                            if inflation_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflationRewards"));
                            }
                            inflation_rewards__ = map.next_value()?;
                        }
                        GeneratedField::TxRewards => {
                            if tx_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txRewards"));
                            }
                            tx_rewards__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BlockTracking {
                    inflation_rewards: inflation_rewards__,
                    tx_rewards: tx_rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.BlockTracking",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractFlatFeeSetEvent {
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
        if self.flat_fee.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.ContractFlatFeeSetEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.flat_fee.as_ref() {
            struct_ser.serialize_field("flatFee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractFlatFeeSetEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress", "flat_fee", "flatFee"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            FlatFee,
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
                            "flatFee" | "flat_fee" => Ok(GeneratedField::FlatFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractFlatFeeSetEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.ContractFlatFeeSetEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ContractFlatFeeSetEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut flat_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::FlatFee => {
                            if flat_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flatFee"));
                            }
                            flat_fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContractFlatFeeSetEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    flat_fee: flat_fee__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.ContractFlatFeeSetEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractMetadata {
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
        if !self.owner_address.is_empty() {
            len += 1;
        }
        if !self.rewards_address.is_empty() {
            len += 1;
        }
        if self.withdraw_to_wallet {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.ContractMetadata", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.owner_address.is_empty() {
            struct_ser.serialize_field("ownerAddress", &self.owner_address)?;
        }
        if !self.rewards_address.is_empty() {
            struct_ser.serialize_field("rewardsAddress", &self.rewards_address)?;
        }
        if self.withdraw_to_wallet {
            struct_ser.serialize_field("withdrawToWallet", &self.withdraw_to_wallet)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "owner_address",
            "ownerAddress",
            "rewards_address",
            "rewardsAddress",
            "withdraw_to_wallet",
            "withdrawToWallet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            OwnerAddress,
            RewardsAddress,
            WithdrawToWallet,
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
                            "ownerAddress" | "owner_address" => Ok(GeneratedField::OwnerAddress),
                            "rewardsAddress" | "rewards_address" => {
                                Ok(GeneratedField::RewardsAddress)
                            }
                            "withdrawToWallet" | "withdraw_to_wallet" => {
                                Ok(GeneratedField::WithdrawToWallet)
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
            type Value = ContractMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.ContractMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContractMetadata, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut owner_address__ = None;
                let mut rewards_address__ = None;
                let mut withdraw_to_wallet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::OwnerAddress => {
                            if owner_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerAddress"));
                            }
                            owner_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::RewardsAddress => {
                            if rewards_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsAddress"));
                            }
                            rewards_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::WithdrawToWallet => {
                            if withdraw_to_wallet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawToWallet"));
                            }
                            withdraw_to_wallet__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ContractMetadata {
                    contract_address: contract_address__.unwrap_or_default(),
                    owner_address: owner_address__.unwrap_or_default(),
                    rewards_address: rewards_address__.unwrap_or_default(),
                    withdraw_to_wallet: withdraw_to_wallet__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.ContractMetadata",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractMetadataSetEvent {
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
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.ContractMetadataSetEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractMetadataSetEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress", "metadata"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            Metadata,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractMetadataSetEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.ContractMetadataSetEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ContractMetadataSetEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContractMetadataSetEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.ContractMetadataSetEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ContractRewardCalculationEvent {
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
        if self.gas_consumed != 0 {
            len += 1;
        }
        if self.inflation_rewards.is_some() {
            len += 1;
        }
        if !self.fee_rebate_rewards.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.rewards.v1.ContractRewardCalculationEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.gas_consumed != 0 {
            struct_ser.serialize_field(
                "gasConsumed",
                ToString::to_string(&self.gas_consumed).as_str(),
            )?;
        }
        if let Some(v) = self.inflation_rewards.as_ref() {
            struct_ser.serialize_field("inflationRewards", v)?;
        }
        if !self.fee_rebate_rewards.is_empty() {
            struct_ser.serialize_field("feeRebateRewards", &self.fee_rebate_rewards)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContractRewardCalculationEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "gas_consumed",
            "gasConsumed",
            "inflation_rewards",
            "inflationRewards",
            "fee_rebate_rewards",
            "feeRebateRewards",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            GasConsumed,
            InflationRewards,
            FeeRebateRewards,
            Metadata,
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
                            "gasConsumed" | "gas_consumed" => Ok(GeneratedField::GasConsumed),
                            "inflationRewards" | "inflation_rewards" => {
                                Ok(GeneratedField::InflationRewards)
                            }
                            "feeRebateRewards" | "fee_rebate_rewards" => {
                                Ok(GeneratedField::FeeRebateRewards)
                            }
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractRewardCalculationEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.ContractRewardCalculationEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ContractRewardCalculationEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut gas_consumed__ = None;
                let mut inflation_rewards__ = None;
                let mut fee_rebate_rewards__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::GasConsumed => {
                            if gas_consumed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasConsumed"));
                            }
                            gas_consumed__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InflationRewards => {
                            if inflation_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflationRewards"));
                            }
                            inflation_rewards__ = map.next_value()?;
                        }
                        GeneratedField::FeeRebateRewards => {
                            if fee_rebate_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRebateRewards"));
                            }
                            fee_rebate_rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContractRewardCalculationEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    gas_consumed: gas_consumed__.unwrap_or_default(),
                    inflation_rewards: inflation_rewards__,
                    fee_rebate_rewards: fee_rebate_rewards__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.ContractRewardCalculationEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for FlatFee {
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
        if self.flat_fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.rewards.v1.FlatFee", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.flat_fee.as_ref() {
            struct_ser.serialize_field("flatFee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlatFee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress", "flat_fee", "flatFee"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            FlatFee,
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
                            "flatFee" | "flat_fee" => Ok(GeneratedField::FlatFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlatFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.FlatFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlatFee, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut flat_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::FlatFee => {
                            if flat_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flatFee"));
                            }
                            flat_fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(FlatFee {
                    contract_address: contract_address__.unwrap_or_default(),
                    flat_fee: flat_fee__,
                })
            }
        }
        deserializer.deserialize_struct("archway.rewards.v1.FlatFee", FIELDS, GeneratedVisitor)
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
        if self.params.is_some() {
            len += 1;
        }
        if !self.contracts_metadata.is_empty() {
            len += 1;
        }
        if !self.block_rewards.is_empty() {
            len += 1;
        }
        if !self.tx_rewards.is_empty() {
            len += 1;
        }
        if self.min_consensus_fee.is_some() {
            len += 1;
        }
        if self.rewards_record_last_id != 0 {
            len += 1;
        }
        if !self.rewards_records.is_empty() {
            len += 1;
        }
        if !self.flat_fees.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.rewards.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.contracts_metadata.is_empty() {
            struct_ser.serialize_field("contractsMetadata", &self.contracts_metadata)?;
        }
        if !self.block_rewards.is_empty() {
            struct_ser.serialize_field("blockRewards", &self.block_rewards)?;
        }
        if !self.tx_rewards.is_empty() {
            struct_ser.serialize_field("txRewards", &self.tx_rewards)?;
        }
        if let Some(v) = self.min_consensus_fee.as_ref() {
            struct_ser.serialize_field("minConsensusFee", v)?;
        }
        if self.rewards_record_last_id != 0 {
            struct_ser.serialize_field(
                "rewardsRecordLastId",
                ToString::to_string(&self.rewards_record_last_id).as_str(),
            )?;
        }
        if !self.rewards_records.is_empty() {
            struct_ser.serialize_field("rewardsRecords", &self.rewards_records)?;
        }
        if !self.flat_fees.is_empty() {
            struct_ser.serialize_field("flatFees", &self.flat_fees)?;
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
            "params",
            "contracts_metadata",
            "contractsMetadata",
            "block_rewards",
            "blockRewards",
            "tx_rewards",
            "txRewards",
            "min_consensus_fee",
            "minConsensusFee",
            "rewards_record_last_id",
            "rewardsRecordLastId",
            "rewards_records",
            "rewardsRecords",
            "flat_fees",
            "flatFees",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            ContractsMetadata,
            BlockRewards,
            TxRewards,
            MinConsensusFee,
            RewardsRecordLastId,
            RewardsRecords,
            FlatFees,
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
                            "params" => Ok(GeneratedField::Params),
                            "contractsMetadata" | "contracts_metadata" => {
                                Ok(GeneratedField::ContractsMetadata)
                            }
                            "blockRewards" | "block_rewards" => Ok(GeneratedField::BlockRewards),
                            "txRewards" | "tx_rewards" => Ok(GeneratedField::TxRewards),
                            "minConsensusFee" | "min_consensus_fee" => {
                                Ok(GeneratedField::MinConsensusFee)
                            }
                            "rewardsRecordLastId" | "rewards_record_last_id" => {
                                Ok(GeneratedField::RewardsRecordLastId)
                            }
                            "rewardsRecords" | "rewards_records" => {
                                Ok(GeneratedField::RewardsRecords)
                            }
                            "flatFees" | "flat_fees" => Ok(GeneratedField::FlatFees),
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
                formatter.write_str("struct archway.rewards.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut contracts_metadata__ = None;
                let mut block_rewards__ = None;
                let mut tx_rewards__ = None;
                let mut min_consensus_fee__ = None;
                let mut rewards_record_last_id__ = None;
                let mut rewards_records__ = None;
                let mut flat_fees__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::ContractsMetadata => {
                            if contracts_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractsMetadata"));
                            }
                            contracts_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlockRewards => {
                            if block_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockRewards"));
                            }
                            block_rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::TxRewards => {
                            if tx_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txRewards"));
                            }
                            tx_rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinConsensusFee => {
                            if min_consensus_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minConsensusFee"));
                            }
                            min_consensus_fee__ = map.next_value()?;
                        }
                        GeneratedField::RewardsRecordLastId => {
                            if rewards_record_last_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "rewardsRecordLastId",
                                ));
                            }
                            rewards_record_last_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RewardsRecords => {
                            if rewards_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsRecords"));
                            }
                            rewards_records__ = Some(map.next_value()?);
                        }
                        GeneratedField::FlatFees => {
                            if flat_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flatFees"));
                            }
                            flat_fees__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    contracts_metadata: contracts_metadata__.unwrap_or_default(),
                    block_rewards: block_rewards__.unwrap_or_default(),
                    tx_rewards: tx_rewards__.unwrap_or_default(),
                    min_consensus_fee: min_consensus_fee__,
                    rewards_record_last_id: rewards_record_last_id__.unwrap_or_default(),
                    rewards_records: rewards_records__.unwrap_or_default(),
                    flat_fees: flat_fees__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.rewards.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinConsensusFeeSetEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MinConsensusFeeSetEvent", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinConsensusFeeSetEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fee"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fee,
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
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MinConsensusFeeSetEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MinConsensusFeeSetEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MinConsensusFeeSetEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MinConsensusFeeSetEvent { fee: fee__ })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MinConsensusFeeSetEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetContractMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender_address.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgSetContractMetadata", len)?;
        if !self.sender_address.is_empty() {
            struct_ser.serialize_field("senderAddress", &self.sender_address)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetContractMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender_address", "senderAddress", "metadata"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SenderAddress,
            Metadata,
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
                            "senderAddress" | "sender_address" => Ok(GeneratedField::SenderAddress),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetContractMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgSetContractMetadata")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetContractMetadata, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender_address__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SenderAddress => {
                            if sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderAddress"));
                            }
                            sender_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgSetContractMetadata {
                    sender_address: sender_address__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgSetContractMetadata",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetContractMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("archway.rewards.v1.MsgSetContractMetadataResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetContractMetadataResponse {
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
            type Value = MsgSetContractMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgSetContractMetadataResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetContractMetadataResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetContractMetadataResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgSetContractMetadataResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetFlatFee {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender_address.is_empty() {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.flat_fee_amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgSetFlatFee", len)?;
        if !self.sender_address.is_empty() {
            struct_ser.serialize_field("senderAddress", &self.sender_address)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.flat_fee_amount.as_ref() {
            struct_ser.serialize_field("flatFeeAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetFlatFee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender_address",
            "senderAddress",
            "contract_address",
            "contractAddress",
            "flat_fee_amount",
            "flatFeeAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SenderAddress,
            ContractAddress,
            FlatFeeAmount,
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
                            "senderAddress" | "sender_address" => Ok(GeneratedField::SenderAddress),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "flatFeeAmount" | "flat_fee_amount" => {
                                Ok(GeneratedField::FlatFeeAmount)
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
            type Value = MsgSetFlatFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgSetFlatFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSetFlatFee, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender_address__ = None;
                let mut contract_address__ = None;
                let mut flat_fee_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SenderAddress => {
                            if sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderAddress"));
                            }
                            sender_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::FlatFeeAmount => {
                            if flat_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flatFeeAmount"));
                            }
                            flat_fee_amount__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgSetFlatFee {
                    sender_address: sender_address__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    flat_fee_amount: flat_fee_amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgSetFlatFee",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetFlatFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgSetFlatFeeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetFlatFeeResponse {
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
            type Value = MsgSetFlatFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgSetFlatFeeResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetFlatFeeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetFlatFeeResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgSetFlatFeeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgUpdateParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
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
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgWithdrawRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rewards_address.is_empty() {
            len += 1;
        }
        if self.mode.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgWithdrawRewards", len)?;
        if !self.rewards_address.is_empty() {
            struct_ser.serialize_field("rewardsAddress", &self.rewards_address)?;
        }
        if let Some(v) = self.mode.as_ref() {
            match v {
                msg_withdraw_rewards::Mode::RecordsLimit(v) => {
                    struct_ser.serialize_field("recordsLimit", v)?;
                }
                msg_withdraw_rewards::Mode::RecordIds(v) => {
                    struct_ser.serialize_field("recordIds", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgWithdrawRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rewards_address",
            "rewardsAddress",
            "records_limit",
            "recordsLimit",
            "record_ids",
            "recordIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardsAddress,
            RecordsLimit,
            RecordIds,
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
                            "rewardsAddress" | "rewards_address" => {
                                Ok(GeneratedField::RewardsAddress)
                            }
                            "recordsLimit" | "records_limit" => Ok(GeneratedField::RecordsLimit),
                            "recordIds" | "record_ids" => Ok(GeneratedField::RecordIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgWithdrawRewards")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgWithdrawRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rewards_address__ = None;
                let mut mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RewardsAddress => {
                            if rewards_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsAddress"));
                            }
                            rewards_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::RecordsLimit => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordsLimit"));
                            }
                            mode__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(msg_withdraw_rewards::Mode::RecordsLimit);
                        }
                        GeneratedField::RecordIds => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordIds"));
                            }
                            mode__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(msg_withdraw_rewards::Mode::RecordIds);
                        }
                    }
                }
                Ok(MsgWithdrawRewards {
                    rewards_address: rewards_address__.unwrap_or_default(),
                    mode: mode__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgWithdrawRewards",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for msg_withdraw_rewards::RecordIDs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgWithdrawRewards.RecordIDs", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field(
                "ids",
                &self.ids.iter().map(ToString::to_string).collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_withdraw_rewards::RecordIDs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ids"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_withdraw_rewards::RecordIDs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgWithdrawRewards.RecordIDs")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<msg_withdraw_rewards::RecordIDs, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(msg_withdraw_rewards::RecordIDs {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgWithdrawRewards.RecordIDs",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for msg_withdraw_rewards::RecordsLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.rewards.v1.MsgWithdrawRewards.RecordsLimit", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", ToString::to_string(&self.limit).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_withdraw_rewards::RecordsLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["limit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
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
                            "limit" => Ok(GeneratedField::Limit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_withdraw_rewards::RecordsLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgWithdrawRewards.RecordsLimit")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<msg_withdraw_rewards::RecordsLimit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(msg_withdraw_rewards::RecordsLimit {
                    limit: limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgWithdrawRewards.RecordsLimit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgWithdrawRewardsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.records_num != 0 {
            len += 1;
        }
        if !self.total_rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.MsgWithdrawRewardsResponse", len)?;
        if self.records_num != 0 {
            struct_ser.serialize_field(
                "recordsNum",
                ToString::to_string(&self.records_num).as_str(),
            )?;
        }
        if !self.total_rewards.is_empty() {
            struct_ser.serialize_field("totalRewards", &self.total_rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgWithdrawRewardsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["records_num", "recordsNum", "total_rewards", "totalRewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RecordsNum,
            TotalRewards,
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
                            "recordsNum" | "records_num" => Ok(GeneratedField::RecordsNum),
                            "totalRewards" | "total_rewards" => Ok(GeneratedField::TotalRewards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawRewardsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.MsgWithdrawRewardsResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgWithdrawRewardsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut records_num__ = None;
                let mut total_rewards__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RecordsNum => {
                            if records_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordsNum"));
                            }
                            records_num__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalRewards => {
                            if total_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRewards"));
                            }
                            total_rewards__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgWithdrawRewardsResponse {
                    records_num: records_num__.unwrap_or_default(),
                    total_rewards: total_rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.MsgWithdrawRewardsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inflation_rewards_ratio.is_empty() {
            len += 1;
        }
        if !self.tx_fee_rebate_ratio.is_empty() {
            len += 1;
        }
        if self.max_withdraw_records != 0 {
            len += 1;
        }
        if self.min_price_of_gas.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.rewards.v1.Params", len)?;
        if !self.inflation_rewards_ratio.is_empty() {
            struct_ser.serialize_field("inflationRewardsRatio", &self.inflation_rewards_ratio)?;
        }
        if !self.tx_fee_rebate_ratio.is_empty() {
            struct_ser.serialize_field("txFeeRebateRatio", &self.tx_fee_rebate_ratio)?;
        }
        if self.max_withdraw_records != 0 {
            struct_ser.serialize_field(
                "maxWithdrawRecords",
                ToString::to_string(&self.max_withdraw_records).as_str(),
            )?;
        }
        if let Some(v) = self.min_price_of_gas.as_ref() {
            struct_ser.serialize_field("minPriceOfGas", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inflation_rewards_ratio",
            "inflationRewardsRatio",
            "tx_fee_rebate_ratio",
            "txFeeRebateRatio",
            "max_withdraw_records",
            "maxWithdrawRecords",
            "min_price_of_gas",
            "minPriceOfGas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InflationRewardsRatio,
            TxFeeRebateRatio,
            MaxWithdrawRecords,
            MinPriceOfGas,
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
                            "inflationRewardsRatio" | "inflation_rewards_ratio" => {
                                Ok(GeneratedField::InflationRewardsRatio)
                            }
                            "txFeeRebateRatio" | "tx_fee_rebate_ratio" => {
                                Ok(GeneratedField::TxFeeRebateRatio)
                            }
                            "maxWithdrawRecords" | "max_withdraw_records" => {
                                Ok(GeneratedField::MaxWithdrawRecords)
                            }
                            "minPriceOfGas" | "min_price_of_gas" => {
                                Ok(GeneratedField::MinPriceOfGas)
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
            type Value = Params;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut inflation_rewards_ratio__ = None;
                let mut tx_fee_rebate_ratio__ = None;
                let mut max_withdraw_records__ = None;
                let mut min_price_of_gas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InflationRewardsRatio => {
                            if inflation_rewards_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "inflationRewardsRatio",
                                ));
                            }
                            inflation_rewards_ratio__ = Some(map.next_value()?);
                        }
                        GeneratedField::TxFeeRebateRatio => {
                            if tx_fee_rebate_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txFeeRebateRatio"));
                            }
                            tx_fee_rebate_ratio__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxWithdrawRecords => {
                            if max_withdraw_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxWithdrawRecords",
                                ));
                            }
                            max_withdraw_records__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinPriceOfGas => {
                            if min_price_of_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPriceOfGas"));
                            }
                            min_price_of_gas__ = map.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    inflation_rewards_ratio: inflation_rewards_ratio__.unwrap_or_default(),
                    tx_fee_rebate_ratio: tx_fee_rebate_ratio__.unwrap_or_default(),
                    max_withdraw_records: max_withdraw_records__.unwrap_or_default(),
                    min_price_of_gas: min_price_of_gas__,
                })
            }
        }
        deserializer.deserialize_struct("archway.rewards.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBlockRewardsTrackingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("archway.rewards.v1.QueryBlockRewardsTrackingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockRewardsTrackingRequest {
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
            type Value = QueryBlockRewardsTrackingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryBlockRewardsTrackingRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryBlockRewardsTrackingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBlockRewardsTrackingRequest {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryBlockRewardsTrackingRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBlockRewardsTrackingResponse {
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
            .serialize_struct("archway.rewards.v1.QueryBlockRewardsTrackingResponse", len)?;
        if let Some(v) = self.block.as_ref() {
            struct_ser.serialize_field("block", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlockRewardsTrackingResponse {
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
            type Value = QueryBlockRewardsTrackingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryBlockRewardsTrackingResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryBlockRewardsTrackingResponse, V::Error>
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
                Ok(QueryBlockRewardsTrackingResponse { block: block__ })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryBlockRewardsTrackingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractMetadataRequest {
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
            serializer.serialize_struct("archway.rewards.v1.QueryContractMetadataRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractMetadataRequest {
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
            type Value = QueryContractMetadataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryContractMetadataRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryContractMetadataRequest, V::Error>
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
                Ok(QueryContractMetadataRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryContractMetadataRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryContractMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryContractMetadataResponse", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryContractMetadataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["metadata"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryContractMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryContractMetadataResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryContractMetadataResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryContractMetadataResponse {
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryContractMetadataResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryEstimateTxFeesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gas_limit != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryEstimateTxFeesRequest", len)?;
        if self.gas_limit != 0 {
            struct_ser
                .serialize_field("gasLimit", ToString::to_string(&self.gas_limit).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEstimateTxFeesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gas_limit",
            "gasLimit",
            "contract_address",
            "contractAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GasLimit,
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
                            "gasLimit" | "gas_limit" => Ok(GeneratedField::GasLimit),
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
            type Value = QueryEstimateTxFeesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryEstimateTxFeesRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryEstimateTxFeesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut gas_limit__ = None;
                let mut contract_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GasLimit => {
                            if gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasLimit"));
                            }
                            gas_limit__ = Some(
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
                    }
                }
                Ok(QueryEstimateTxFeesRequest {
                    gas_limit: gas_limit__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryEstimateTxFeesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryEstimateTxFeesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gas_unit_price.is_some() {
            len += 1;
        }
        if !self.estimated_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryEstimateTxFeesResponse", len)?;
        if let Some(v) = self.gas_unit_price.as_ref() {
            struct_ser.serialize_field("gasUnitPrice", v)?;
        }
        if !self.estimated_fee.is_empty() {
            struct_ser.serialize_field("estimatedFee", &self.estimated_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEstimateTxFeesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gas_unit_price",
            "gasUnitPrice",
            "estimated_fee",
            "estimatedFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GasUnitPrice,
            EstimatedFee,
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
                            "gasUnitPrice" | "gas_unit_price" => Ok(GeneratedField::GasUnitPrice),
                            "estimatedFee" | "estimated_fee" => Ok(GeneratedField::EstimatedFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEstimateTxFeesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryEstimateTxFeesResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryEstimateTxFeesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut gas_unit_price__ = None;
                let mut estimated_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GasUnitPrice => {
                            if gas_unit_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUnitPrice"));
                            }
                            gas_unit_price__ = map.next_value()?;
                        }
                        GeneratedField::EstimatedFee => {
                            if estimated_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedFee"));
                            }
                            estimated_fee__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryEstimateTxFeesResponse {
                    gas_unit_price: gas_unit_price__,
                    estimated_fee: estimated_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryEstimateTxFeesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryFlatFeeRequest {
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
            serializer.serialize_struct("archway.rewards.v1.QueryFlatFeeRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFlatFeeRequest {
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
            type Value = QueryFlatFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryFlatFeeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryFlatFeeRequest, V::Error>
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
                Ok(QueryFlatFeeRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryFlatFeeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryFlatFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.flat_fee_amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryFlatFeeResponse", len)?;
        if let Some(v) = self.flat_fee_amount.as_ref() {
            struct_ser.serialize_field("flatFeeAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFlatFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["flat_fee_amount", "flatFeeAmount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FlatFeeAmount,
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
                            "flatFeeAmount" | "flat_fee_amount" => {
                                Ok(GeneratedField::FlatFeeAmount)
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
            type Value = QueryFlatFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryFlatFeeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryFlatFeeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut flat_fee_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FlatFeeAmount => {
                            if flat_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flatFeeAmount"));
                            }
                            flat_fee_amount__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryFlatFeeResponse {
                    flat_fee_amount: flat_fee_amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryFlatFeeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryOutstandingRewardsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rewards_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.rewards.v1.QueryOutstandingRewardsRequest", len)?;
        if !self.rewards_address.is_empty() {
            struct_ser.serialize_field("rewardsAddress", &self.rewards_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOutstandingRewardsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["rewards_address", "rewardsAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardsAddress,
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
                            "rewardsAddress" | "rewards_address" => {
                                Ok(GeneratedField::RewardsAddress)
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
            type Value = QueryOutstandingRewardsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryOutstandingRewardsRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryOutstandingRewardsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rewards_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RewardsAddress => {
                            if rewards_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsAddress"));
                            }
                            rewards_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryOutstandingRewardsRequest {
                    rewards_address: rewards_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryOutstandingRewardsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryOutstandingRewardsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.total_rewards.is_empty() {
            len += 1;
        }
        if self.records_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.rewards.v1.QueryOutstandingRewardsResponse", len)?;
        if !self.total_rewards.is_empty() {
            struct_ser.serialize_field("totalRewards", &self.total_rewards)?;
        }
        if self.records_num != 0 {
            struct_ser.serialize_field(
                "recordsNum",
                ToString::to_string(&self.records_num).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOutstandingRewardsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["total_rewards", "totalRewards", "records_num", "recordsNum"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalRewards,
            RecordsNum,
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
                            "totalRewards" | "total_rewards" => Ok(GeneratedField::TotalRewards),
                            "recordsNum" | "records_num" => Ok(GeneratedField::RecordsNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOutstandingRewardsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryOutstandingRewardsResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryOutstandingRewardsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_rewards__ = None;
                let mut records_num__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TotalRewards => {
                            if total_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRewards"));
                            }
                            total_rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::RecordsNum => {
                            if records_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordsNum"));
                            }
                            records_num__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryOutstandingRewardsResponse {
                    total_rewards: total_rewards__.unwrap_or_default(),
                    records_num: records_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryOutstandingRewardsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
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
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRewardsPoolRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryRewardsPoolRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardsPoolRequest {
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
            type Value = QueryRewardsPoolRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryRewardsPoolRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardsPoolRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryRewardsPoolRequest {})
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryRewardsPoolRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRewardsPoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.undistributed_funds.is_empty() {
            len += 1;
        }
        if !self.treasury_funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryRewardsPoolResponse", len)?;
        if !self.undistributed_funds.is_empty() {
            struct_ser.serialize_field("undistributedFunds", &self.undistributed_funds)?;
        }
        if !self.treasury_funds.is_empty() {
            struct_ser.serialize_field("treasuryFunds", &self.treasury_funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardsPoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "undistributed_funds",
            "undistributedFunds",
            "treasury_funds",
            "treasuryFunds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UndistributedFunds,
            TreasuryFunds,
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
                            "undistributedFunds" | "undistributed_funds" => {
                                Ok(GeneratedField::UndistributedFunds)
                            }
                            "treasuryFunds" | "treasury_funds" => Ok(GeneratedField::TreasuryFunds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRewardsPoolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryRewardsPoolResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardsPoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut undistributed_funds__ = None;
                let mut treasury_funds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UndistributedFunds => {
                            if undistributed_funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "undistributedFunds",
                                ));
                            }
                            undistributed_funds__ = Some(map.next_value()?);
                        }
                        GeneratedField::TreasuryFunds => {
                            if treasury_funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treasuryFunds"));
                            }
                            treasury_funds__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryRewardsPoolResponse {
                    undistributed_funds: undistributed_funds__.unwrap_or_default(),
                    treasury_funds: treasury_funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryRewardsPoolResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRewardsRecordsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rewards_address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryRewardsRecordsRequest", len)?;
        if !self.rewards_address.is_empty() {
            struct_ser.serialize_field("rewardsAddress", &self.rewards_address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardsRecordsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["rewards_address", "rewardsAddress", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardsAddress,
            Pagination,
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
                            "rewardsAddress" | "rewards_address" => {
                                Ok(GeneratedField::RewardsAddress)
                            }
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRewardsRecordsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryRewardsRecordsRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardsRecordsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rewards_address__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RewardsAddress => {
                            if rewards_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsAddress"));
                            }
                            rewards_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRewardsRecordsRequest {
                    rewards_address: rewards_address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryRewardsRecordsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRewardsRecordsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.records.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.QueryRewardsRecordsResponse", len)?;
        if !self.records.is_empty() {
            struct_ser.serialize_field("records", &self.records)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardsRecordsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["records", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Records,
            Pagination,
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
                            "records" => Ok(GeneratedField::Records),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRewardsRecordsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.QueryRewardsRecordsResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardsRecordsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut records__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Records => {
                            if records__.is_some() {
                                return Err(serde::de::Error::duplicate_field("records"));
                            }
                            records__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRewardsRecordsResponse {
                    records: records__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.QueryRewardsRecordsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RewardsRecord {
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
        if !self.rewards_address.is_empty() {
            len += 1;
        }
        if !self.rewards.is_empty() {
            len += 1;
        }
        if self.calculated_height != 0 {
            len += 1;
        }
        if self.calculated_time.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.RewardsRecord", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.rewards_address.is_empty() {
            struct_ser.serialize_field("rewardsAddress", &self.rewards_address)?;
        }
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        if self.calculated_height != 0 {
            struct_ser.serialize_field(
                "calculatedHeight",
                ToString::to_string(&self.calculated_height).as_str(),
            )?;
        }
        if let Some(v) = self.calculated_time.as_ref() {
            struct_ser.serialize_field("calculatedTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RewardsRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "rewards_address",
            "rewardsAddress",
            "rewards",
            "calculated_height",
            "calculatedHeight",
            "calculated_time",
            "calculatedTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            RewardsAddress,
            Rewards,
            CalculatedHeight,
            CalculatedTime,
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
                            "rewardsAddress" | "rewards_address" => {
                                Ok(GeneratedField::RewardsAddress)
                            }
                            "rewards" => Ok(GeneratedField::Rewards),
                            "calculatedHeight" | "calculated_height" => {
                                Ok(GeneratedField::CalculatedHeight)
                            }
                            "calculatedTime" | "calculated_time" => {
                                Ok(GeneratedField::CalculatedTime)
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
            type Value = RewardsRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.RewardsRecord")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RewardsRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut rewards_address__ = None;
                let mut rewards__ = None;
                let mut calculated_height__ = None;
                let mut calculated_time__ = None;
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
                        GeneratedField::RewardsAddress => {
                            if rewards_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardsAddress"));
                            }
                            rewards_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::CalculatedHeight => {
                            if calculated_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedHeight"));
                            }
                            calculated_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CalculatedTime => {
                            if calculated_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedTime"));
                            }
                            calculated_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(RewardsRecord {
                    id: id__.unwrap_or_default(),
                    rewards_address: rewards_address__.unwrap_or_default(),
                    rewards: rewards__.unwrap_or_default(),
                    calculated_height: calculated_height__.unwrap_or_default(),
                    calculated_time: calculated_time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.RewardsRecord",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RewardsWithdrawEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reward_address.is_empty() {
            len += 1;
        }
        if !self.rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.rewards.v1.RewardsWithdrawEvent", len)?;
        if !self.reward_address.is_empty() {
            struct_ser.serialize_field("rewardAddress", &self.reward_address)?;
        }
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RewardsWithdrawEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reward_address", "rewardAddress", "rewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RewardAddress,
            Rewards,
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
                            "rewardAddress" | "reward_address" => Ok(GeneratedField::RewardAddress),
                            "rewards" => Ok(GeneratedField::Rewards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RewardsWithdrawEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.RewardsWithdrawEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RewardsWithdrawEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reward_address__ = None;
                let mut rewards__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RewardAddress => {
                            if reward_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAddress"));
                            }
                            reward_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RewardsWithdrawEvent {
                    reward_address: reward_address__.unwrap_or_default(),
                    rewards: rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.rewards.v1.RewardsWithdrawEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for TxRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tx_id != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if !self.fee_rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.rewards.v1.TxRewards", len)?;
        if self.tx_id != 0 {
            struct_ser.serialize_field("txId", ToString::to_string(&self.tx_id).as_str())?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if !self.fee_rewards.is_empty() {
            struct_ser.serialize_field("feeRewards", &self.fee_rewards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["tx_id", "txId", "height", "fee_rewards", "feeRewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxId,
            Height,
            FeeRewards,
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
                            "txId" | "tx_id" => Ok(GeneratedField::TxId),
                            "height" => Ok(GeneratedField::Height),
                            "feeRewards" | "fee_rewards" => Ok(GeneratedField::FeeRewards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.rewards.v1.TxRewards")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TxRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tx_id__ = None;
                let mut height__ = None;
                let mut fee_rewards__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TxId => {
                            if tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txId"));
                            }
                            tx_id__ = Some(
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
                        GeneratedField::FeeRewards => {
                            if fee_rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRewards"));
                            }
                            fee_rewards__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TxRewards {
                    tx_id: tx_id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    fee_rewards: fee_rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.rewards.v1.TxRewards", FIELDS, GeneratedVisitor)
    }
}
