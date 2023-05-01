use std::{collections::BTreeMap};

use self::mongo::Mongo;

pub mod mongo;

pub enum Storage<Key, Value> {
    Mongo(Mongo),
    Stub(tokio::sync::Mutex<BTreeMap<Key, Value>>),
}

impl<Value> Storage<u64, Value> {
    pub fn create_mongo(db_name: String) -> Result<Self, mongodb::error::Error> {
        Mongo::create_storage(db_name).map(Storage::Mongo)
    }

    pub fn create_stub() -> Self {
        Self::Stub(tokio::sync::Mutex::new(BTreeMap::new()))
    }

    pub async fn store(&self, key: u64, val: Value) -> bool
    where
        Value: serde::Serialize,
    {
        match self {
            Storage::Mongo(mongo) => mongo
                .store(key, val)
                .await
                .inspect_err(|err| eprintln!("couldn't update id {key}: {err}"))
                .is_ok(),
            Storage::Stub(map) => {
                let mut locked = map.lock().await;
                locked.insert(key, val);
                true
            }
        }
    }

    pub async fn load(&self, key: u64) -> Option<Value>
    where
        for<'a> Value: serde::Deserialize<'a> + Clone + Unpin + Send + Sync,
    {
        match self {
            Storage::Mongo(mongo) => mongo.load(key).await,
            Storage::Stub(map) => map.lock().await.get(&key).cloned(),
        }
    }

    pub async fn load_all<'s>(&'s self) -> Option<Iter<Value>>
    where
        for<'a> Value: serde::Deserialize<'a> + Unpin + Send + Sync + Clone,
    {
        match self {
            Storage::Mongo(mongo) => mongo
                .load_all()
                .await
                .map(Iter::Mongo)
                .map(Some)
                .unwrap_or(None),
            Storage::Stub(stub) => {
                let locked = stub.lock().await;
                Some(Iter::Stub(StubIter {
                    data: locked.iter().map(|(_, val)| val).cloned().collect(),
                    idx: 0,
                }))
            }
        }
    }
}

#[async_trait_fn::async_trait]
pub trait AsyncIterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;

    fn map<F, Res>(self, func: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> Res + Send,
        Self: Sized,
    {
        Map {
            iter: self,
            func,
        }
    }
}

pub struct StubIter<Value> {
    data: Vec<Value>,
    idx: usize,
}

#[async_trait_fn::async_trait]
impl<Value: Send + Sync + Clone> AsyncIterator for StubIter<Value> {
    type Item = Value;

    async fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.len() {
            self.idx += 1;
            return Some(self.data[self.idx - 1].clone());
        }
        None
    }
}

pub enum Iter<Value> {
    Mongo(mongodb::Cursor<Value>),
    Stub(StubIter<Value>),
}

#[async_trait_fn::async_trait]
impl<Value> AsyncIterator for Iter<Value>
where
    Value: Send + Sync + Clone,
    for<'a> Value: serde::Deserialize<'a>,
{
    type Item = Value;

    async fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Mongo(mongo) => mongo.next().await,
            Self::Stub(stub) => stub.next().await,
        }
    }
}

pub struct Map<Iter, F> {
    iter: Iter,
    func: F,
}

#[async_trait_fn::async_trait]
impl<Iter, F, Res> AsyncIterator for Map<Iter, F>
where
    Iter: AsyncIterator + Send,
    F: FnMut(<Iter as AsyncIterator>::Item) -> Res + Send
{
    type Item = Res;


    async fn next(&mut self) -> Option<Self::Item> {
        let arg = self.iter.next().await?;
        Some((self.func)(arg))
    }
}
