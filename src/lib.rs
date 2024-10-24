use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Error<K>
where
    K: Debug + Clone,
{
    kind: K,
    msg_stack_v: Vec<(String, String)>,
}

impl<K> Error<K>
where
    K: Debug + Clone,
{
    pub fn new(kind: K) -> Self {
        Self {
            kind,
            msg_stack_v: vec![],
        }
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn msg_stack_v(&self) -> &Vec<(String, String)> {
        &self.msg_stack_v
    }
}

pub fn switch_kind<K, K1>(k: K1) -> impl FnOnce(Error<K>) -> Error<K1>
where
    K: Debug + Clone,
    K1: Debug + Clone,
{
    move |mut e| {
        if let Some((message, _)) = e.msg_stack_v.first_mut() {
            *message = format!("{:?}: {message}", e.kind);
        }
        Error::<K1> {
            kind: k,
            msg_stack_v: e.msg_stack_v,
        }
    }
}

pub fn unshift_msg_stack<K>(msg: String, stack: String) -> impl FnOnce(Error<K>) -> Error<K>
where
    K: Debug + Clone,
{
    move |mut e| {
        e.msg_stack_v.insert(0, (msg, stack));
        e
    }
}

#[cfg(test)]
mod tests {
    use crate::switch_kind;

    use super::unshift_msg_stack;

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

        let rs: Result<(), Error<ErrorKind>> = Err(Error::new(ErrorKind::NotFound))
            .map_err(unshift_msg_stack(format!("unknown"), format!("at unknown")))
            .map_err(switch_kind(ErrorKind::Other))
            .map_err(unshift_msg_stack(format!("unknown"), format!("at unknown")));

        let e = rs.unwrap_err();

        log::warn!("{e:?}");
    }
}
