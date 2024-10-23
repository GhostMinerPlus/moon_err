use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Error<K>
where
    K: Debug + Clone,
{
    kind: K,
    message: String,
    stack_v: Vec<String>,
}

impl<K> Error<K>
where
    K: Debug + Clone,
{
    pub fn new(kind: K, message: String) -> Self {
        Self {
            kind,
            message,
            stack_v: vec![],
        }
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn stack_v(&self) -> &[String] {
        &self.stack_v
    }
}

pub fn switch_kind<K, K1>(k: K1) -> impl FnOnce(Error<K>) -> Error<K1>
where
    K: Debug + Clone,
    K1: Debug + Clone,
{
    move |e| Error::<K1> {
        kind: k,
        message: e.message,
        stack_v: e.stack_v,
    }
}

pub fn append_stack<K>(stack: String) -> impl FnOnce(Error<K>) -> Error<K>
where
    K: Debug + Clone,
{
    move |mut e| {
        e.stack_v.push(stack);
        e
    }
}

#[cfg(test)]
mod tests {
    use crate::switch_kind;

    use super::append_stack;

    use super::Error;

    #[derive(Debug, Clone)]
    enum ErrorKind {
        Other,
        NotFound,
    }

    #[test]
    fn test() {
        let _ =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .is_test(true)
                .try_init();

        let rs: Result<(), Error<ErrorKind>> = Err(Error::new(ErrorKind::NotFound, format!("")))
            .map_err(switch_kind(ErrorKind::Other))
            .map_err(append_stack(format!("1")));

        let e = rs.unwrap_err();

        log::warn!("{e:?}");
    }
}
