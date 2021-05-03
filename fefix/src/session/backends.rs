use super::{Backend, State};
use fnv::FnvHashMap;
use futures::future;
use futures::future::ready;
use rand;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Local {
    messages: FnvHashMap<u128, Vec<Vec<u8>>>,
    sessions: FnvHashMap<u128, State>,
}

impl Local {
    pub fn new() -> Self {
        Self {
            messages: FnvHashMap::default(),
            sessions: FnvHashMap::default(),
        }
    }
}

#[derive(Debug)]
pub enum LocalError {
    InvalidSession,
}

impl<'a> Backend<'a> for Local {
    type Error = ();
    type IMessages = future::Ready<Result<Vec<u8>, Self::Error>>;
    type IStore = future::Ready<Result<(), Self::Error>>;
    type ISessionState = future::Ready<Result<Option<State>, Self::Error>>;
    type ICreateSession = future::Ready<Result<u128, Self::Error>>;

    fn create_session(&mut self) -> Self::ICreateSession {
        let id = rand::random::<u128>();
        self.sessions.insert(
            id,
            State {
                next_inbound: 1,
                next_outbound: 1,
            },
        );
        ready(Ok(id))
    }

    fn session_state(&self, session_id: u128) -> Self::ISessionState {
        ready(Ok(self.sessions.get(&session_id).cloned()))
    }

    fn messages(&'a self, session_id: u128, _range: Range<u64>) -> Self::IMessages {
        ready(Ok(self.messages.get(&session_id).unwrap()[0].clone()))
    }

    fn store(&mut self, session_id: u128, fix_message: &[u8]) -> Self::IStore {
        self.messages
            .get_mut(&session_id)
            .unwrap()
            .push(fix_message.to_vec());
        ready(Ok(()))
    }
}

//#[derive(Debug, Clone)]
//pub struct BackPg {
//    pool: sqlx::PgPool,
//}
//
//impl<'a> Backend<'a> for BackPg {
//    type Messages = BackPgMessages;
//    type ItemMessage = futures::future::Ready<Self::Messages>;
//    type Error = ();
//    type SessionId = u64;
//    type ConnectionId = u64;
//
//    fn messages(&'a self, _range: Range<u64>) -> Self::ItemMessage {}
//
//    fn store(&mut self, fix_message: &[u8], _id: u64) -> Result<(), Self::Error> {
//        let query = sqlx::query_as("SELECT $1")
//            .bind(150_i64)
//            .fetch_one::<(i64,)>(&self.pool);
//        query.map(|_| ())
//    }
//}
