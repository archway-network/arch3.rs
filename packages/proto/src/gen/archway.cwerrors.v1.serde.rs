// @generated
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
        if !self.errors.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.errors.is_empty() {
            struct_ser.serialize_field("errors", &self.errors)?;
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
        const FIELDS: &[&str] = &["params", "errors"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Errors,
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
                            "errors" => Ok(GeneratedField::Errors),
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
                formatter.write_str("struct archway.cwerrors.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut errors__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::Errors => {
                            if errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            errors__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    errors: errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ModuleErrors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ErrUnknown => "ERR_UNKNOWN",
            Self::ErrCallbackExecutionFailed => "ERR_CALLBACK_EXECUTION_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ModuleErrors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ERR_UNKNOWN", "ERR_CALLBACK_EXECUTION_FAILED"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModuleErrors;

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
                    .and_then(ModuleErrors::from_i32)
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
                    .and_then(ModuleErrors::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERR_UNKNOWN" => Ok(ModuleErrors::ErrUnknown),
                    "ERR_CALLBACK_EXECUTION_FAILED" => Ok(ModuleErrors::ErrCallbackExecutionFailed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSubscribeToError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.MsgSubscribeToError", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSubscribeToError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract_address", "contractAddress", "fee"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractAddress,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
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
            type Value = MsgSubscribeToError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.MsgSubscribeToError")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSubscribeToError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgSubscribeToError {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.MsgSubscribeToError",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSubscribeToErrorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subscription_valid_till != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.MsgSubscribeToErrorResponse", len)?;
        if self.subscription_valid_till != 0 {
            struct_ser.serialize_field(
                "subscriptionValidTill",
                ToString::to_string(&self.subscription_valid_till).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSubscribeToErrorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subscription_valid_till", "subscriptionValidTill"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubscriptionValidTill,
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
                            "subscriptionValidTill" | "subscription_valid_till" => {
                                Ok(GeneratedField::SubscriptionValidTill)
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
            type Value = MsgSubscribeToErrorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.MsgSubscribeToErrorResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSubscribeToErrorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subscription_valid_till__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubscriptionValidTill => {
                            if subscription_valid_till__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subscriptionValidTill",
                                ));
                            }
                            subscription_valid_till__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSubscribeToErrorResponse {
                    subscription_valid_till: subscription_valid_till__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.MsgSubscribeToErrorResponse",
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
            serializer.serialize_struct("archway.cwerrors.v1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct archway.cwerrors.v1.MsgUpdateParams")
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
            "archway.cwerrors.v1.MsgUpdateParams",
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
            serializer.serialize_struct("archway.cwerrors.v1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct archway.cwerrors.v1.MsgUpdateParamsResponse")
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
            "archway.cwerrors.v1.MsgUpdateParamsResponse",
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
        if self.error_stored_time != 0 {
            len += 1;
        }
        if self.subscription_fee.is_some() {
            len += 1;
        }
        if self.subscription_period != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.cwerrors.v1.Params", len)?;
        if self.error_stored_time != 0 {
            struct_ser.serialize_field(
                "errorStoredTime",
                ToString::to_string(&self.error_stored_time).as_str(),
            )?;
        }
        if let Some(v) = self.subscription_fee.as_ref() {
            struct_ser.serialize_field("subscriptionFee", v)?;
        }
        if self.subscription_period != 0 {
            struct_ser.serialize_field(
                "subscriptionPeriod",
                ToString::to_string(&self.subscription_period).as_str(),
            )?;
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
            "error_stored_time",
            "errorStoredTime",
            "subscription_fee",
            "subscriptionFee",
            "subscription_period",
            "subscriptionPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorStoredTime,
            SubscriptionFee,
            SubscriptionPeriod,
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
                            "errorStoredTime" | "error_stored_time" => {
                                Ok(GeneratedField::ErrorStoredTime)
                            }
                            "subscriptionFee" | "subscription_fee" => {
                                Ok(GeneratedField::SubscriptionFee)
                            }
                            "subscriptionPeriod" | "subscription_period" => {
                                Ok(GeneratedField::SubscriptionPeriod)
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
                formatter.write_str("struct archway.cwerrors.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut error_stored_time__ = None;
                let mut subscription_fee__ = None;
                let mut subscription_period__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ErrorStoredTime => {
                            if error_stored_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorStoredTime"));
                            }
                            error_stored_time__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SubscriptionFee => {
                            if subscription_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptionFee"));
                            }
                            subscription_fee__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionPeriod => {
                            if subscription_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subscriptionPeriod",
                                ));
                            }
                            subscription_period__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Params {
                    error_stored_time: error_stored_time__.unwrap_or_default(),
                    subscription_fee: subscription_fee__,
                    subscription_period: subscription_period__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.cwerrors.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParamsUpdatedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.new_params.is_some() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.ParamsUpdatedEvent", len)?;
        if let Some(v) = self.new_params.as_ref() {
            struct_ser.serialize_field("newParams", v)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParamsUpdatedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["new_params", "newParams", "authority"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewParams,
            Authority,
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
                            "newParams" | "new_params" => Ok(GeneratedField::NewParams),
                            "authority" => Ok(GeneratedField::Authority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParamsUpdatedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.ParamsUpdatedEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ParamsUpdatedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut new_params__ = None;
                let mut authority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NewParams => {
                            if new_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newParams"));
                            }
                            new_params__ = map.next_value()?;
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ParamsUpdatedEvent {
                    new_params: new_params__,
                    authority: authority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.ParamsUpdatedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryErrorsRequest {
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
            serializer.serialize_struct("archway.cwerrors.v1.QueryErrorsRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryErrorsRequest {
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
            type Value = QueryErrorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.QueryErrorsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryErrorsRequest, V::Error>
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
                Ok(QueryErrorsRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.QueryErrorsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryErrorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.errors.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.QueryErrorsResponse", len)?;
        if !self.errors.is_empty() {
            struct_ser.serialize_field("errors", &self.errors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryErrorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["errors"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Errors,
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
                            "errors" => Ok(GeneratedField::Errors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryErrorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.QueryErrorsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryErrorsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut errors__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Errors => {
                            if errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            errors__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryErrorsResponse {
                    errors: errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.QueryErrorsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryIsSubscribedRequest {
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
            serializer.serialize_struct("archway.cwerrors.v1.QueryIsSubscribedRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIsSubscribedRequest {
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
            type Value = QueryIsSubscribedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.QueryIsSubscribedRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIsSubscribedRequest, V::Error>
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
                Ok(QueryIsSubscribedRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.QueryIsSubscribedRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryIsSubscribedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subscribed {
            len += 1;
        }
        if self.subscription_valid_till != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.QueryIsSubscribedResponse", len)?;
        if self.subscribed {
            struct_ser.serialize_field("subscribed", &self.subscribed)?;
        }
        if self.subscription_valid_till != 0 {
            struct_ser.serialize_field(
                "subscriptionValidTill",
                ToString::to_string(&self.subscription_valid_till).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIsSubscribedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subscribed",
            "subscription_valid_till",
            "subscriptionValidTill",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subscribed,
            SubscriptionValidTill,
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
                            "subscribed" => Ok(GeneratedField::Subscribed),
                            "subscriptionValidTill" | "subscription_valid_till" => {
                                Ok(GeneratedField::SubscriptionValidTill)
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
            type Value = QueryIsSubscribedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.QueryIsSubscribedResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIsSubscribedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subscribed__ = None;
                let mut subscription_valid_till__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subscribed => {
                            if subscribed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscribed"));
                            }
                            subscribed__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubscriptionValidTill => {
                            if subscription_valid_till__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subscriptionValidTill",
                                ));
                            }
                            subscription_valid_till__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryIsSubscribedResponse {
                    subscribed: subscribed__.unwrap_or_default(),
                    subscription_valid_till: subscription_valid_till__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.QueryIsSubscribedResponse",
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
            serializer.serialize_struct("archway.cwerrors.v1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct archway.cwerrors.v1.QueryParamsRequest")
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
            "archway.cwerrors.v1.QueryParamsRequest",
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
            serializer.serialize_struct("archway.cwerrors.v1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct archway.cwerrors.v1.QueryParamsResponse")
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
            "archway.cwerrors.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StoringErrorEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error.is_some() {
            len += 1;
        }
        if self.deletion_block_height != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.StoringErrorEvent", len)?;
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if self.deletion_block_height != 0 {
            struct_ser.serialize_field(
                "deletionBlockHeight",
                ToString::to_string(&self.deletion_block_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoringErrorEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["error", "deletion_block_height", "deletionBlockHeight"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            DeletionBlockHeight,
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
                            "error" => Ok(GeneratedField::Error),
                            "deletionBlockHeight" | "deletion_block_height" => {
                                Ok(GeneratedField::DeletionBlockHeight)
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
            type Value = StoringErrorEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.StoringErrorEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StoringErrorEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut deletion_block_height__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::DeletionBlockHeight => {
                            if deletion_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "deletionBlockHeight",
                                ));
                            }
                            deletion_block_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(StoringErrorEvent {
                    error: error__,
                    deletion_block_height: deletion_block_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.StoringErrorEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SubscribedToErrorsEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.fees_paid.is_some() {
            len += 1;
        }
        if self.subscription_valid_till != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.SubscribedToErrorsEvent", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if let Some(v) = self.fees_paid.as_ref() {
            struct_ser.serialize_field("feesPaid", v)?;
        }
        if self.subscription_valid_till != 0 {
            struct_ser.serialize_field(
                "subscriptionValidTill",
                ToString::to_string(&self.subscription_valid_till).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribedToErrorsEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "contract_address",
            "contractAddress",
            "fees_paid",
            "feesPaid",
            "subscription_valid_till",
            "subscriptionValidTill",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractAddress,
            FeesPaid,
            SubscriptionValidTill,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "feesPaid" | "fees_paid" => Ok(GeneratedField::FeesPaid),
                            "subscriptionValidTill" | "subscription_valid_till" => {
                                Ok(GeneratedField::SubscriptionValidTill)
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
            type Value = SubscribedToErrorsEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.SubscribedToErrorsEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<SubscribedToErrorsEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                let mut fees_paid__ = None;
                let mut subscription_valid_till__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::FeesPaid => {
                            if fees_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feesPaid"));
                            }
                            fees_paid__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionValidTill => {
                            if subscription_valid_till__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subscriptionValidTill",
                                ));
                            }
                            subscription_valid_till__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SubscribedToErrorsEvent {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    fees_paid: fees_paid__,
                    subscription_valid_till: subscription_valid_till__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.SubscribedToErrorsEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SudoError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module_name.is_empty() {
            len += 1;
        }
        if self.error_code != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.input_payload.is_empty() {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.cwerrors.v1.SudoError", len)?;
        if !self.module_name.is_empty() {
            struct_ser.serialize_field("moduleName", &self.module_name)?;
        }
        if self.error_code != 0 {
            struct_ser.serialize_field("errorCode", &self.error_code)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.input_payload.is_empty() {
            struct_ser.serialize_field("inputPayload", &self.input_payload)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SudoError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module_name",
            "moduleName",
            "error_code",
            "errorCode",
            "contract_address",
            "contractAddress",
            "input_payload",
            "inputPayload",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModuleName,
            ErrorCode,
            ContractAddress,
            InputPayload,
            ErrorMessage,
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
                            "moduleName" | "module_name" => Ok(GeneratedField::ModuleName),
                            "errorCode" | "error_code" => Ok(GeneratedField::ErrorCode),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "inputPayload" | "input_payload" => Ok(GeneratedField::InputPayload),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SudoError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.SudoError")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SudoError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut module_name__ = None;
                let mut error_code__ = None;
                let mut contract_address__ = None;
                let mut input_payload__ = None;
                let mut error_message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModuleName => {
                            if module_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleName"));
                            }
                            module_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorCode => {
                            if error_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorCode"));
                            }
                            error_code__ = Some(
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
                        GeneratedField::InputPayload => {
                            if input_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputPayload"));
                            }
                            input_payload__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SudoError {
                    module_name: module_name__.unwrap_or_default(),
                    error_code: error_code__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    input_payload: input_payload__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.cwerrors.v1.SudoError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SudoErrorCallbackFailedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error.is_some() {
            len += 1;
        }
        if !self.callback_error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.cwerrors.v1.SudoErrorCallbackFailedEvent", len)?;
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.callback_error_message.is_empty() {
            struct_ser.serialize_field("callbackErrorMessage", &self.callback_error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SudoErrorCallbackFailedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["error", "callback_error_message", "callbackErrorMessage"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            CallbackErrorMessage,
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
                            "error" => Ok(GeneratedField::Error),
                            "callbackErrorMessage" | "callback_error_message" => {
                                Ok(GeneratedField::CallbackErrorMessage)
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
            type Value = SudoErrorCallbackFailedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.cwerrors.v1.SudoErrorCallbackFailedEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<SudoErrorCallbackFailedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut callback_error_message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::CallbackErrorMessage => {
                            if callback_error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "callbackErrorMessage",
                                ));
                            }
                            callback_error_message__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SudoErrorCallbackFailedEvent {
                    error: error__,
                    callback_error_message: callback_error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.cwerrors.v1.SudoErrorCallbackFailedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
