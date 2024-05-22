// @generated
impl serde::Serialize for Callback {
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
        if self.job_id != 0 {
            len += 1;
        }
        if self.callback_height != 0 {
            len += 1;
        }
        if self.fee_split.is_some() {
            len += 1;
        }
        if !self.reserved_by.is_empty() {
            len += 1;
        }
        if self.max_gas_limit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.callback.v1.Callback", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if self.callback_height != 0 {
            struct_ser.serialize_field(
                "callbackHeight",
                ToString::to_string(&self.callback_height).as_str(),
            )?;
        }
        if let Some(v) = self.fee_split.as_ref() {
            struct_ser.serialize_field("feeSplit", v)?;
        }
        if !self.reserved_by.is_empty() {
            struct_ser.serialize_field("reservedBy", &self.reserved_by)?;
        }
        if self.max_gas_limit != 0 {
            struct_ser.serialize_field(
                "maxGasLimit",
                ToString::to_string(&self.max_gas_limit).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Callback {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "callback_height",
            "callbackHeight",
            "fee_split",
            "feeSplit",
            "reserved_by",
            "reservedBy",
            "max_gas_limit",
            "maxGasLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            JobId,
            CallbackHeight,
            FeeSplit,
            ReservedBy,
            MaxGasLimit,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "callbackHeight" | "callback_height" => {
                                Ok(GeneratedField::CallbackHeight)
                            }
                            "feeSplit" | "fee_split" => Ok(GeneratedField::FeeSplit),
                            "reservedBy" | "reserved_by" => Ok(GeneratedField::ReservedBy),
                            "maxGasLimit" | "max_gas_limit" => Ok(GeneratedField::MaxGasLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Callback;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.Callback")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Callback, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut callback_height__ = None;
                let mut fee_split__ = None;
                let mut reserved_by__ = None;
                let mut max_gas_limit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallbackHeight => {
                            if callback_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackHeight"));
                            }
                            callback_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeeSplit => {
                            if fee_split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeSplit"));
                            }
                            fee_split__ = map.next_value()?;
                        }
                        GeneratedField::ReservedBy => {
                            if reserved_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedBy"));
                            }
                            reserved_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxGasLimit => {
                            if max_gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasLimit"));
                            }
                            max_gas_limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Callback {
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    callback_height: callback_height__.unwrap_or_default(),
                    fee_split: fee_split__,
                    reserved_by: reserved_by__.unwrap_or_default(),
                    max_gas_limit: max_gas_limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.callback.v1.Callback", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CallbackCancelledEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cancelled_by.is_empty() {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.job_id != 0 {
            len += 1;
        }
        if self.callback_height != 0 {
            len += 1;
        }
        if self.refund_amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.CallbackCancelledEvent", len)?;
        if !self.cancelled_by.is_empty() {
            struct_ser.serialize_field("cancelledBy", &self.cancelled_by)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if self.callback_height != 0 {
            struct_ser.serialize_field(
                "callbackHeight",
                ToString::to_string(&self.callback_height).as_str(),
            )?;
        }
        if let Some(v) = self.refund_amount.as_ref() {
            struct_ser.serialize_field("refundAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CallbackCancelledEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cancelled_by",
            "cancelledBy",
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "callback_height",
            "callbackHeight",
            "refund_amount",
            "refundAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CancelledBy,
            ContractAddress,
            JobId,
            CallbackHeight,
            RefundAmount,
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
                            "cancelledBy" | "cancelled_by" => Ok(GeneratedField::CancelledBy),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "callbackHeight" | "callback_height" => {
                                Ok(GeneratedField::CallbackHeight)
                            }
                            "refundAmount" | "refund_amount" => Ok(GeneratedField::RefundAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CallbackCancelledEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.CallbackCancelledEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<CallbackCancelledEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cancelled_by__ = None;
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut callback_height__ = None;
                let mut refund_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CancelledBy => {
                            if cancelled_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelledBy"));
                            }
                            cancelled_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallbackHeight => {
                            if callback_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackHeight"));
                            }
                            callback_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RefundAmount => {
                            if refund_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refundAmount"));
                            }
                            refund_amount__ = map.next_value()?;
                        }
                    }
                }
                Ok(CallbackCancelledEvent {
                    cancelled_by: cancelled_by__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    callback_height: callback_height__.unwrap_or_default(),
                    refund_amount: refund_amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.CallbackCancelledEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for CallbackExecutedFailedEvent {
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
        if self.job_id != 0 {
            len += 1;
        }
        if !self.sudo_msg.is_empty() {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        if !self.error.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.CallbackExecutedFailedEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if !self.sudo_msg.is_empty() {
            struct_ser.serialize_field("sudoMsg", &self.sudo_msg)?;
        }
        if self.gas_used != 0 {
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
        }
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CallbackExecutedFailedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "sudo_msg",
            "sudoMsg",
            "gas_used",
            "gasUsed",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            JobId,
            SudoMsg,
            GasUsed,
            Error,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "sudoMsg" | "sudo_msg" => Ok(GeneratedField::SudoMsg),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CallbackExecutedFailedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.CallbackExecutedFailedEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<CallbackExecutedFailedEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut sudo_msg__ = None;
                let mut gas_used__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SudoMsg => {
                            if sudo_msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sudoMsg"));
                            }
                            sudo_msg__ = Some(map.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CallbackExecutedFailedEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    sudo_msg: sudo_msg__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.CallbackExecutedFailedEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for CallbackExecutedSuccessEvent {
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
        if self.job_id != 0 {
            len += 1;
        }
        if !self.sudo_msg.is_empty() {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.CallbackExecutedSuccessEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if !self.sudo_msg.is_empty() {
            struct_ser.serialize_field("sudoMsg", &self.sudo_msg)?;
        }
        if self.gas_used != 0 {
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CallbackExecutedSuccessEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "sudo_msg",
            "sudoMsg",
            "gas_used",
            "gasUsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            JobId,
            SudoMsg,
            GasUsed,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "sudoMsg" | "sudo_msg" => Ok(GeneratedField::SudoMsg),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CallbackExecutedSuccessEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.CallbackExecutedSuccessEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<CallbackExecutedSuccessEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut sudo_msg__ = None;
                let mut gas_used__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SudoMsg => {
                            if sudo_msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sudoMsg"));
                            }
                            sudo_msg__ = Some(map.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(CallbackExecutedSuccessEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    sudo_msg: sudo_msg__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.CallbackExecutedSuccessEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for CallbackFeesFeeSplit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transaction_fees.is_some() {
            len += 1;
        }
        if self.block_reservation_fees.is_some() {
            len += 1;
        }
        if self.future_reservation_fees.is_some() {
            len += 1;
        }
        if self.surplus_fees.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.CallbackFeesFeeSplit", len)?;
        if let Some(v) = self.transaction_fees.as_ref() {
            struct_ser.serialize_field("transactionFees", v)?;
        }
        if let Some(v) = self.block_reservation_fees.as_ref() {
            struct_ser.serialize_field("blockReservationFees", v)?;
        }
        if let Some(v) = self.future_reservation_fees.as_ref() {
            struct_ser.serialize_field("futureReservationFees", v)?;
        }
        if let Some(v) = self.surplus_fees.as_ref() {
            struct_ser.serialize_field("surplusFees", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CallbackFeesFeeSplit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_fees",
            "transactionFees",
            "block_reservation_fees",
            "blockReservationFees",
            "future_reservation_fees",
            "futureReservationFees",
            "surplus_fees",
            "surplusFees",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionFees,
            BlockReservationFees,
            FutureReservationFees,
            SurplusFees,
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
                            "transactionFees" | "transaction_fees" => {
                                Ok(GeneratedField::TransactionFees)
                            }
                            "blockReservationFees" | "block_reservation_fees" => {
                                Ok(GeneratedField::BlockReservationFees)
                            }
                            "futureReservationFees" | "future_reservation_fees" => {
                                Ok(GeneratedField::FutureReservationFees)
                            }
                            "surplusFees" | "surplus_fees" => Ok(GeneratedField::SurplusFees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CallbackFeesFeeSplit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.CallbackFeesFeeSplit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CallbackFeesFeeSplit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transaction_fees__ = None;
                let mut block_reservation_fees__ = None;
                let mut future_reservation_fees__ = None;
                let mut surplus_fees__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransactionFees => {
                            if transaction_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionFees"));
                            }
                            transaction_fees__ = map.next_value()?;
                        }
                        GeneratedField::BlockReservationFees => {
                            if block_reservation_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "blockReservationFees",
                                ));
                            }
                            block_reservation_fees__ = map.next_value()?;
                        }
                        GeneratedField::FutureReservationFees => {
                            if future_reservation_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "futureReservationFees",
                                ));
                            }
                            future_reservation_fees__ = map.next_value()?;
                        }
                        GeneratedField::SurplusFees => {
                            if surplus_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surplusFees"));
                            }
                            surplus_fees__ = map.next_value()?;
                        }
                    }
                }
                Ok(CallbackFeesFeeSplit {
                    transaction_fees: transaction_fees__,
                    block_reservation_fees: block_reservation_fees__,
                    future_reservation_fees: future_reservation_fees__,
                    surplus_fees: surplus_fees__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.CallbackFeesFeeSplit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for CallbackRegisteredEvent {
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
        if self.job_id != 0 {
            len += 1;
        }
        if self.callback_height != 0 {
            len += 1;
        }
        if self.fee_split.is_some() {
            len += 1;
        }
        if !self.reserved_by.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.CallbackRegisteredEvent", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if self.callback_height != 0 {
            struct_ser.serialize_field(
                "callbackHeight",
                ToString::to_string(&self.callback_height).as_str(),
            )?;
        }
        if let Some(v) = self.fee_split.as_ref() {
            struct_ser.serialize_field("feeSplit", v)?;
        }
        if !self.reserved_by.is_empty() {
            struct_ser.serialize_field("reservedBy", &self.reserved_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CallbackRegisteredEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "callback_height",
            "callbackHeight",
            "fee_split",
            "feeSplit",
            "reserved_by",
            "reservedBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            JobId,
            CallbackHeight,
            FeeSplit,
            ReservedBy,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "callbackHeight" | "callback_height" => {
                                Ok(GeneratedField::CallbackHeight)
                            }
                            "feeSplit" | "fee_split" => Ok(GeneratedField::FeeSplit),
                            "reservedBy" | "reserved_by" => Ok(GeneratedField::ReservedBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CallbackRegisteredEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.CallbackRegisteredEvent")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<CallbackRegisteredEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut callback_height__ = None;
                let mut fee_split__ = None;
                let mut reserved_by__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallbackHeight => {
                            if callback_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackHeight"));
                            }
                            callback_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeeSplit => {
                            if fee_split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeSplit"));
                            }
                            fee_split__ = map.next_value()?;
                        }
                        GeneratedField::ReservedBy => {
                            if reserved_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedBy"));
                            }
                            reserved_by__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CallbackRegisteredEvent {
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    callback_height: callback_height__.unwrap_or_default(),
                    fee_split: fee_split__,
                    reserved_by: reserved_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.CallbackRegisteredEvent",
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
        if self.params.is_some() {
            len += 1;
        }
        if !self.callbacks.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.callbacks.is_empty() {
            struct_ser.serialize_field("callbacks", &self.callbacks)?;
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
        const FIELDS: &[&str] = &["params", "callbacks"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Callbacks,
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
                            "callbacks" => Ok(GeneratedField::Callbacks),
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
                formatter.write_str("struct archway.callback.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut callbacks__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::Callbacks => {
                            if callbacks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbacks"));
                            }
                            callbacks__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    callbacks: callbacks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.GenesisState",
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
            Self::ErrOutOfGas => "ERR_OUT_OF_GAS",
            Self::ErrContractExecutionFailed => "ERR_CONTRACT_EXECUTION_FAILED",
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
        const FIELDS: &[&str] = &[
            "ERR_UNKNOWN",
            "ERR_OUT_OF_GAS",
            "ERR_CONTRACT_EXECUTION_FAILED",
        ];

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
                    "ERR_OUT_OF_GAS" => Ok(ModuleErrors::ErrOutOfGas),
                    "ERR_CONTRACT_EXECUTION_FAILED" => Ok(ModuleErrors::ErrContractExecutionFailed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCancelCallback {
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
        if self.job_id != 0 {
            len += 1;
        }
        if self.callback_height != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.MsgCancelCallback", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if self.callback_height != 0 {
            struct_ser.serialize_field(
                "callbackHeight",
                ToString::to_string(&self.callback_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelCallback {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "callback_height",
            "callbackHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractAddress,
            JobId,
            CallbackHeight,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "callbackHeight" | "callback_height" => {
                                Ok(GeneratedField::CallbackHeight)
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
            type Value = MsgCancelCallback;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.MsgCancelCallback")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCancelCallback, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut callback_height__ = None;
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
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallbackHeight => {
                            if callback_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackHeight"));
                            }
                            callback_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgCancelCallback {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    callback_height: callback_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.MsgCancelCallback",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCancelCallbackResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.refund.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.MsgCancelCallbackResponse", len)?;
        if let Some(v) = self.refund.as_ref() {
            struct_ser.serialize_field("refund", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelCallbackResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["refund"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Refund,
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
                            "refund" => Ok(GeneratedField::Refund),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelCallbackResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.MsgCancelCallbackResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCancelCallbackResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut refund__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Refund => {
                            if refund__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refund"));
                            }
                            refund__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCancelCallbackResponse { refund: refund__ })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.MsgCancelCallbackResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRequestCallback {
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
        if self.job_id != 0 {
            len += 1;
        }
        if self.callback_height != 0 {
            len += 1;
        }
        if self.fees.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.MsgRequestCallback", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.job_id != 0 {
            struct_ser.serialize_field("jobId", ToString::to_string(&self.job_id).as_str())?;
        }
        if self.callback_height != 0 {
            struct_ser.serialize_field(
                "callbackHeight",
                ToString::to_string(&self.callback_height).as_str(),
            )?;
        }
        if let Some(v) = self.fees.as_ref() {
            struct_ser.serialize_field("fees", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRequestCallback {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "contract_address",
            "contractAddress",
            "job_id",
            "jobId",
            "callback_height",
            "callbackHeight",
            "fees",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractAddress,
            JobId,
            CallbackHeight,
            Fees,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "callbackHeight" | "callback_height" => {
                                Ok(GeneratedField::CallbackHeight)
                            }
                            "fees" => Ok(GeneratedField::Fees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRequestCallback;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.MsgRequestCallback")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRequestCallback, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                let mut job_id__ = None;
                let mut callback_height__ = None;
                let mut fees__ = None;
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
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallbackHeight => {
                            if callback_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackHeight"));
                            }
                            callback_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Fees => {
                            if fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fees"));
                            }
                            fees__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgRequestCallback {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    callback_height: callback_height__.unwrap_or_default(),
                    fees: fees__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.MsgRequestCallback",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRequestCallbackResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("archway.callback.v1.MsgRequestCallbackResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRequestCallbackResponse {
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
            type Value = MsgRequestCallbackResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.MsgRequestCallbackResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRequestCallbackResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRequestCallbackResponse {})
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.MsgRequestCallbackResponse",
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
            serializer.serialize_struct("archway.callback.v1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct archway.callback.v1.MsgUpdateParams")
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
            "archway.callback.v1.MsgUpdateParams",
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
            serializer.serialize_struct("archway.callback.v1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct archway.callback.v1.MsgUpdateParamsResponse")
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
            "archway.callback.v1.MsgUpdateParamsResponse",
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
        if self.callback_gas_limit != 0 {
            len += 1;
        }
        if self.max_block_reservation_limit != 0 {
            len += 1;
        }
        if self.max_future_reservation_limit != 0 {
            len += 1;
        }
        if !self.block_reservation_fee_multiplier.is_empty() {
            len += 1;
        }
        if !self.future_reservation_fee_multiplier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("archway.callback.v1.Params", len)?;
        if self.callback_gas_limit != 0 {
            struct_ser.serialize_field(
                "callbackGasLimit",
                ToString::to_string(&self.callback_gas_limit).as_str(),
            )?;
        }
        if self.max_block_reservation_limit != 0 {
            struct_ser.serialize_field(
                "maxBlockReservationLimit",
                ToString::to_string(&self.max_block_reservation_limit).as_str(),
            )?;
        }
        if self.max_future_reservation_limit != 0 {
            struct_ser.serialize_field(
                "maxFutureReservationLimit",
                ToString::to_string(&self.max_future_reservation_limit).as_str(),
            )?;
        }
        if !self.block_reservation_fee_multiplier.is_empty() {
            struct_ser.serialize_field(
                "blockReservationFeeMultiplier",
                &self.block_reservation_fee_multiplier,
            )?;
        }
        if !self.future_reservation_fee_multiplier.is_empty() {
            struct_ser.serialize_field(
                "futureReservationFeeMultiplier",
                &self.future_reservation_fee_multiplier,
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
            "callback_gas_limit",
            "callbackGasLimit",
            "max_block_reservation_limit",
            "maxBlockReservationLimit",
            "max_future_reservation_limit",
            "maxFutureReservationLimit",
            "block_reservation_fee_multiplier",
            "blockReservationFeeMultiplier",
            "future_reservation_fee_multiplier",
            "futureReservationFeeMultiplier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CallbackGasLimit,
            MaxBlockReservationLimit,
            MaxFutureReservationLimit,
            BlockReservationFeeMultiplier,
            FutureReservationFeeMultiplier,
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
                            "callbackGasLimit" | "callback_gas_limit" => {
                                Ok(GeneratedField::CallbackGasLimit)
                            }
                            "maxBlockReservationLimit" | "max_block_reservation_limit" => {
                                Ok(GeneratedField::MaxBlockReservationLimit)
                            }
                            "maxFutureReservationLimit" | "max_future_reservation_limit" => {
                                Ok(GeneratedField::MaxFutureReservationLimit)
                            }
                            "blockReservationFeeMultiplier"
                            | "block_reservation_fee_multiplier" => {
                                Ok(GeneratedField::BlockReservationFeeMultiplier)
                            }
                            "futureReservationFeeMultiplier"
                            | "future_reservation_fee_multiplier" => {
                                Ok(GeneratedField::FutureReservationFeeMultiplier)
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
                formatter.write_str("struct archway.callback.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut callback_gas_limit__ = None;
                let mut max_block_reservation_limit__ = None;
                let mut max_future_reservation_limit__ = None;
                let mut block_reservation_fee_multiplier__ = None;
                let mut future_reservation_fee_multiplier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CallbackGasLimit => {
                            if callback_gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackGasLimit"));
                            }
                            callback_gas_limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxBlockReservationLimit => {
                            if max_block_reservation_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxBlockReservationLimit",
                                ));
                            }
                            max_block_reservation_limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxFutureReservationLimit => {
                            if max_future_reservation_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxFutureReservationLimit",
                                ));
                            }
                            max_future_reservation_limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BlockReservationFeeMultiplier => {
                            if block_reservation_fee_multiplier__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "blockReservationFeeMultiplier",
                                ));
                            }
                            block_reservation_fee_multiplier__ = Some(map.next_value()?);
                        }
                        GeneratedField::FutureReservationFeeMultiplier => {
                            if future_reservation_fee_multiplier__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "futureReservationFeeMultiplier",
                                ));
                            }
                            future_reservation_fee_multiplier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    callback_gas_limit: callback_gas_limit__.unwrap_or_default(),
                    max_block_reservation_limit: max_block_reservation_limit__.unwrap_or_default(),
                    max_future_reservation_limit: max_future_reservation_limit__
                        .unwrap_or_default(),
                    block_reservation_fee_multiplier: block_reservation_fee_multiplier__
                        .unwrap_or_default(),
                    future_reservation_fee_multiplier: future_reservation_fee_multiplier__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("archway.callback.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCallbacksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_height != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.QueryCallbacksRequest", len)?;
        if self.block_height != 0 {
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCallbacksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["block_height", "blockHeight"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeight,
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
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCallbacksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.QueryCallbacksRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCallbacksRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block_height__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryCallbacksRequest {
                    block_height: block_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.QueryCallbacksRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCallbacksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.callbacks.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("archway.callback.v1.QueryCallbacksResponse", len)?;
        if !self.callbacks.is_empty() {
            struct_ser.serialize_field("callbacks", &self.callbacks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCallbacksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["callbacks"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Callbacks,
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
                            "callbacks" => Ok(GeneratedField::Callbacks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCallbacksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.QueryCallbacksResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCallbacksResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut callbacks__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Callbacks => {
                            if callbacks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbacks"));
                            }
                            callbacks__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryCallbacksResponse {
                    callbacks: callbacks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.QueryCallbacksResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryEstimateCallbackFeesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.callback.v1.QueryEstimateCallbackFeesRequest", len)?;
        if self.block_height != 0 {
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEstimateCallbackFeesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["block_height", "blockHeight"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeight,
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
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEstimateCallbackFeesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.QueryEstimateCallbackFeesRequest")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryEstimateCallbackFeesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block_height__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryEstimateCallbackFeesRequest {
                    block_height: block_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.QueryEstimateCallbackFeesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryEstimateCallbackFeesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_fees.is_some() {
            len += 1;
        }
        if self.fee_split.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("archway.callback.v1.QueryEstimateCallbackFeesResponse", len)?;
        if let Some(v) = self.total_fees.as_ref() {
            struct_ser.serialize_field("totalFees", v)?;
        }
        if let Some(v) = self.fee_split.as_ref() {
            struct_ser.serialize_field("feeSplit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEstimateCallbackFeesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["total_fees", "totalFees", "fee_split", "feeSplit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalFees,
            FeeSplit,
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
                            "totalFees" | "total_fees" => Ok(GeneratedField::TotalFees),
                            "feeSplit" | "fee_split" => Ok(GeneratedField::FeeSplit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEstimateCallbackFeesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct archway.callback.v1.QueryEstimateCallbackFeesResponse")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryEstimateCallbackFeesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_fees__ = None;
                let mut fee_split__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TotalFees => {
                            if total_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalFees"));
                            }
                            total_fees__ = map.next_value()?;
                        }
                        GeneratedField::FeeSplit => {
                            if fee_split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeSplit"));
                            }
                            fee_split__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryEstimateCallbackFeesResponse {
                    total_fees: total_fees__,
                    fee_split: fee_split__,
                })
            }
        }
        deserializer.deserialize_struct(
            "archway.callback.v1.QueryEstimateCallbackFeesResponse",
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
            serializer.serialize_struct("archway.callback.v1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct archway.callback.v1.QueryParamsRequest")
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
            "archway.callback.v1.QueryParamsRequest",
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
            serializer.serialize_struct("archway.callback.v1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct archway.callback.v1.QueryParamsResponse")
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
            "archway.callback.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
