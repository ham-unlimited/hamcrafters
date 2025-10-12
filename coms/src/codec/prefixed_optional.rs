#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrefixedOptional<T> {
    present: bool,
    data: Vec<T>,
}

impl<'de, T> Deserialize<'de> for PrefixedOptional<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrefixedOptionalVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for PrefixedOptionalVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = PrefixedOptional<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a prefixed optional: a boolean indicating whether the value is present, followed by the value if present"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let present: bool = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let mut items = Vec::with_capacity(1);

                if present {
                    let item: T = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    items.push(item);
                }

                if let Some(_) = seq.next_element::<de::IgnoredAny>()? {
                    return Err(de::Error::custom(
                        "extra elements after the prefixed optional",
                    ));
                }

                Ok(PrefixedOptional {
                    present,
                    data: items,
                })
            }
        }

        deserializer.deserialize_seq(PrefixedOptionalVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
