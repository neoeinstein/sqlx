use bigdecimal::BigDecimal;

use crate::decode::{accepts, Decode};
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::mysql::io::MySqlBufMutExt;
use crate::mysql::protocol::text::ColumnType;
use crate::mysql::{MySql, MySqlTypeInfo, MySqlValueRef};
use crate::types::Type;

impl Type<MySql> for BigDecimal {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::NewDecimal)
    }
}

impl Encode<'_, MySql> for BigDecimal {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        buf.put_str_lenenc(&self.to_string());

        IsNull::No
    }
}

impl Decode<'_, MySql> for BigDecimal {
    fn accepts(ty: &MySqlTypeInfo) -> bool {
        accepts::<MySql, Self>(ty)
    }

    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?.parse()?)
    }
}
