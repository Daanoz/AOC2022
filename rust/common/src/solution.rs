use async_trait::async_trait;

#[async_trait]
pub trait Solution {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String>;
    async fn solve_b(&mut self, input: String) -> Result<Answer, String>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Answer {
    result: String,
}
impl Answer {
    pub fn get_result(&self) -> String {
        self.result.clone()
    }
}

impl Into<Result<Answer, String>> for Answer {
    fn into(self) -> Result<Answer, String> {
        Ok(self)
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self { result: value }
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self {
            result: String::from(value),
        }
    }
}

macro_rules! from_numeric_to_answer {
    ($type:ty) => {
        impl From<$type> for Answer {
            fn from(value: $type) -> Self {
                Self {
                    result: value.to_string(),
                }
            }
        }
        impl From<&$type> for Answer {
            fn from(value: &$type) -> Self {
                Self {
                    result: value.to_string(),
                }
            }
        }
        impl From<Option<$type>> for Answer {
            fn from(value: Option<$type>) -> Self {
                Self {
                    result: value.map(|v| v.to_string()).unwrap_or(String::new()),
                }
            }
        }
    };
}
from_numeric_to_answer!(usize);
from_numeric_to_answer!(u64);
from_numeric_to_answer!(u32);
from_numeric_to_answer!(u16);
from_numeric_to_answer!(u8);
from_numeric_to_answer!(isize);
from_numeric_to_answer!(i64);
from_numeric_to_answer!(i32);
from_numeric_to_answer!(i16);
from_numeric_to_answer!(i8);
from_numeric_to_answer!(f32);
from_numeric_to_answer!(f64);
