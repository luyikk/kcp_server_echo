use tokio::sync::Mutex;
use udp_server::{UdpSend, TokenStore};
use crate::kcp::{Kcp,KcpWrite};
use futures::executor::block_on;

/// 实现UDPSend KcpWrite 实现,好通过UDP 发送数据
impl KcpWrite for UdpSend{
    fn write_all(&self, data: &[u8]) -> std::io::Result<usize> {
        block_on(async move{
            self.send(data).await
        })
    }
}



/// 用来存储KCP 对象
pub struct KcpStore(Option<Box<Kcp<UdpSend>>>);
impl KcpStore {
    /// 是否有值
    pub fn have(&self) -> bool {
        match &self.0 {
            None => false,
            Some(_) => true,
        }
    }

    ///获取
    pub fn get(&mut self) -> Option<&mut Box<Kcp<UdpSend>>> {
        match self.0 {
            None => None,
            Some(ref mut v) => Some(v),
        }
    }

    ///设置
    pub fn set(&mut self, v: Box<Kcp<UdpSend>>) {
        self.0 = Some(v);
    }
}

/// KCP Peer
/// UDP的包进入 KCP PEER 经过KCP 处理后 输出
/// 输入的包 进入KCP PEER处理,然后 输出到UDP PEER SEND TO
/// 同时还需要一个UPDATE 线程 去10MS 一次的运行KCP UPDATE
/// token 用于扩赞逻辑上下文
pub struct KcpPeer<T: Send> {
    inner: Mutex<KcpStore>,
    pub conv: u32,
    pub addr: String,
    pub token: Mutex<TokenStore<T>>,
}