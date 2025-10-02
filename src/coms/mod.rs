use std::io::Read;

use thiserror::Error;

use crate::codec::{var_int::VarInt, var_long::VarLong, var_uint::VarUInt, var_ulong::VarULong};

pub mod deserialize;

#[derive(Debug, Error)]
pub enum ReadingError {
    #[error("EOF, Tried to read {0} but No bytes left to consume")]
    CleanEOF(String),
    #[error("incomplete: {0}")]
    Incomplete(String),
    #[error("too large: {0}")]
    TooLarge(String),
    #[error("{0}")]
    Message(String),
}

pub struct Data<'a> {
    data: &'a [u8],
}

pub trait NetworkReadExt {
    fn get_i8(&mut self) -> Result<i8, ReadingError>;
    fn get_u8(&mut self) -> Result<u8, ReadingError>;

    fn get_i16_be(&mut self) -> Result<i16, ReadingError>;
    fn get_u16_be(&mut self) -> Result<u16, ReadingError>;
    fn get_i32_be(&mut self) -> Result<i32, ReadingError>;
    fn get_u32_be(&mut self) -> Result<u32, ReadingError>;
    fn get_i64_be(&mut self) -> Result<i64, ReadingError>;
    fn get_u64_be(&mut self) -> Result<u64, ReadingError>;
    fn get_f32_be(&mut self) -> Result<f32, ReadingError>;
    fn get_f64_be(&mut self) -> Result<f64, ReadingError>;
    fn get_i128_be(&mut self) -> Result<i128, ReadingError>;
    fn get_u128_be(&mut self) -> Result<u128, ReadingError>;
    fn read_boxed_slice(&mut self, count: usize) -> Result<Box<[u8]>, ReadingError>;

    fn read_remaining_to_boxed_slice(&mut self, bound: usize) -> Result<Box<[u8]>, ReadingError>;

    fn get_bool(&mut self) -> Result<bool, ReadingError>;
    fn get_var_int(&mut self) -> Result<VarInt, ReadingError>;
    fn get_var_uint(&mut self) -> Result<VarUInt, ReadingError>;
    fn get_var_long(&mut self) -> Result<VarLong, ReadingError>;
    fn get_var_ulong(&mut self) -> Result<VarULong, ReadingError>;
    fn get_string_bounded(&mut self, bound: usize) -> Result<String, ReadingError>;
    fn get_string(&mut self) -> Result<String, ReadingError>;
    // fn get_resource_location(&mut self) -> Result<ResourceLocation, ReadingError>;
    fn get_uuid(&mut self) -> Result<uuid::Uuid, ReadingError>;
    // fn get_fixed_bitset(&mut self, bits: usize) -> Result<FixedBitSet, ReadingError>;

    fn get_option<G>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<G, ReadingError>,
    ) -> Result<Option<G>, ReadingError>;

    fn get_list<G>(
        &mut self,
        parse: impl Fn(&mut Self) -> Result<G, ReadingError>,
    ) -> Result<Vec<G>, ReadingError>;
}

macro_rules! get_number_be {
    ($name:ident, $type:ty) => {
        fn $name(&mut self) -> Result<$type, ReadingError> {
            let mut buf = [0u8; std::mem::size_of::<$type>()];
            self.read_exact(&mut buf)
                .map_err(|err| ReadingError::Incomplete(err.to_string()))?;
            Ok(<$type>::from_be_bytes(buf))
        }
    };
}

impl<R: Read> NetworkReadExt for R {
    //TODO: Macroize this
    fn get_i8(&mut self) -> Result<i8, ReadingError> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)
            .map_err(|err| ReadingError::Incomplete(err.to_string()))?;

        Ok(buf[0] as i8)
    }

    fn get_u8(&mut self) -> Result<u8, ReadingError> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)
            .map_err(|err| ReadingError::Incomplete(err.to_string()))?;

        Ok(buf[0])
    }

    get_number_be!(get_i16_be, i16);
    get_number_be!(get_u16_be, u16);
    get_number_be!(get_i32_be, i32);
    get_number_be!(get_u32_be, u32);
    get_number_be!(get_i64_be, i64);
    get_number_be!(get_u64_be, u64);
    get_number_be!(get_i128_be, i128);
    get_number_be!(get_u128_be, u128);
    get_number_be!(get_f32_be, f32);
    get_number_be!(get_f64_be, f64);

    fn read_boxed_slice(&mut self, count: usize) -> Result<Box<[u8]>, ReadingError> {
        let mut buf = vec![0u8; count];
        self.read_exact(&mut buf)
            .map_err(|err| ReadingError::Incomplete(err.to_string()))?;

        Ok(buf.into())
    }

    fn read_remaining_to_boxed_slice(&mut self, bound: usize) -> Result<Box<[u8]>, ReadingError> {
        let mut return_buf = Vec::new();

        // TODO: We can probably remove the temp buffer somehow
        let mut temp_buf = [0; 1024];
        loop {
            let bytes_read = self
                .read(&mut temp_buf)
                .map_err(|err| ReadingError::Incomplete(err.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            if return_buf.len() + bytes_read > bound {
                return Err(ReadingError::TooLarge(
                    "Read remaining too long".to_string(),
                ));
            }

            return_buf.extend(&temp_buf[..bytes_read]);
        }
        Ok(return_buf.into_boxed_slice())
    }

    fn get_bool(&mut self) -> Result<bool, ReadingError> {
        let byte = self.get_u8()?;
        Ok(byte != 0)
    }

    fn get_var_int(&mut self) -> Result<VarInt, ReadingError> {
        VarInt::decode(self)
    }
    fn get_var_uint(&mut self) -> Result<VarUInt, ReadingError> {
        VarUInt::decode(self)
    }

    fn get_var_long(&mut self) -> Result<VarLong, ReadingError> {
        VarLong::decode(self)
    }

    fn get_var_ulong(&mut self) -> Result<VarULong, ReadingError> {
        VarULong::decode(self)
    }

    fn get_string_bounded(&mut self, bound: usize) -> Result<String, ReadingError> {
        let size = self.get_var_uint()?.0 as usize;
        if size > bound {
            return Err(ReadingError::TooLarge("string".to_string()));
        }

        let data = self.read_boxed_slice(size)?;
        String::from_utf8(data.into()).map_err(|e| ReadingError::Message(e.to_string()))
    }

    fn get_string(&mut self) -> Result<String, ReadingError> {
        self.get_string_bounded(i32::MAX as usize)
    }

    // fn get_resource_location(&mut self) -> Result<ResourceLocation, ReadingError> {
    //     let resource_location = self.get_string_bounded(ResourceLocation::MAX_SIZE.get())?;
    //     match resource_location.split_once(":") {
    //         Some((namespace, path)) => Ok(ResourceLocation {
    //             namespace: namespace.to_string(),
    //             path: path.to_string(),
    //         }),
    //         None => Err(ReadingError::Incomplete("ResourceLocation".to_string())),
    //     }
    // }

    fn get_uuid(&mut self) -> Result<uuid::Uuid, ReadingError> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)
            .map_err(|err| ReadingError::Incomplete(err.to_string()))?;
        Ok(uuid::Uuid::from_slice(&bytes).expect("Failed to parse UUID"))
    }

    // fn get_fixed_bitset(&mut self, bits: usize) -> Result<FixedBitSet, ReadingError> {
    //     let bytes = self.read_boxed_slice(bits.div_ceil(8))?;
    //     Ok(bytes)
    // }

    fn get_option<G>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<G, ReadingError>,
    ) -> Result<Option<G>, ReadingError> {
        if self.get_bool()? {
            Ok(Some(parse(self)?))
        } else {
            Ok(None)
        }
    }

    fn get_list<G>(
        &mut self,
        parse: impl Fn(&mut Self) -> Result<G, ReadingError>,
    ) -> Result<Vec<G>, ReadingError> {
        let len = self.get_var_int()?.0 as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            list.push(parse(self)?);
        }
        Ok(list)
    }
}
