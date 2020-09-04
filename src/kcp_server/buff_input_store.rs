use std::sync::Arc;
use tokio::sync::Mutex;
use udp_server::TokenStore;
use crate::kcp_server::kcp_peer::KcpPeer;
use bytes::Bytes;
use std::error::Error;

/// 数据包输入原型
type BuffInputType<S> = dyn Fn(Arc<Mutex<TokenStore<Arc<KcpPeer<S>>>>>, Bytes) -> Result<(), Box<dyn Error>>
+ 'static
+ Send
+ Sync;

/// 用来存储 数据表输入函数
pub struct BuffInputStore<S: Send + 'static>(pub Option<Box<BuffInputType<S>>>);

impl<S: Send + 'static> BuffInputStore<S> {

    /// 获取
    pub fn get(&self) -> Option<&Box<BuffInputType<S>>> {
        match self.0 {
            None => None,
            Some(ref v) => Some(v),
        }
    }

    /// 设置
    pub fn set(&mut self, v: Box<BuffInputType<S>>) {
        self.0 = Some(v);
    }
}