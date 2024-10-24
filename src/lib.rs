use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Error<K>
where
    K: Debug + Clone,
{
    stack_v: Vec<(K, String, String)>,
}

impl<K> Error<K>
where
    K: Debug + Clone,
{
    pub fn new(kind: K, msg: String, stack: String) -> Self {
        Self {
            stack_v: vec![(kind, msg, stack)],
        }
    }

    pub fn first(&self) -> &(K, String, String) {
        self.stack_v.first().unwrap()
    }

    pub fn stack_v(&self) -> &Vec<(K, String, String)> {
        &self.stack_v
    }

    pub fn unshift_stack(&mut self, kind: K, msg: String, stack: String) {
        self.stack_v.insert(0, (kind, msg, stack));
    }
}

pub fn unshift_stack<K>(kind: K, msg: String, stack: String) -> impl FnOnce(Error<K>) -> Error<K>
where
    K: Debug + Clone,
{
    move |mut e| {
        e.unshift_stack(kind, msg, stack);
        e
    }
}

#[cfg(test)]
mod tests {
    use crate::unshift_stack;

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

        let rs: Result<(), Error<ErrorKind>> = Err(Error::new(
            ErrorKind::NotFound,
            format!("unknown"),
            format!("at unknown"),
        ))
        .map_err(unshift_stack(
            ErrorKind::Other,
            format!("unknown"),
            format!("at unknown"),
        ));

        let e = rs.unwrap_err();

        log::warn!("{e:?}");
    }
}
