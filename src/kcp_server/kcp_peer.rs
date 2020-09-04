use tokio::sync::Mutex;
use udp_server::{UdpSend, TokenStore};
use crate::kcp::{Kcp, KcpResult};
use futures::executor::block_on;
use std::time::Instant;
use std::cell::{RefCell, UnsafeCell};
use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use std::ops::Deref;


pub struct KcpInputRecv(Arc<UnsafeCell<Kcp>>);
unsafe impl Send for KcpInputRecv{}
unsafe impl Sync for KcpInputRecv{}

impl KcpInputRecv{
    pub fn peeksize(&self)-> KcpResult<usize>{
        unsafe {
            (*self.0.get()).peeksize()
        }
    }

    pub fn input(&self, buf: &[u8]) -> KcpResult<usize>{
        unsafe {
            (*self.0.get()).input(buf)
        }
    }

    pub fn recv(&self, buf: &mut [u8]) -> KcpResult<usize>{
        unsafe {
            (*self.0.get()).recv(buf)
        }
    }
}



pub struct KcpSend(Arc<UnsafeCell<Kcp>>);
unsafe impl Sync for KcpSend{}
unsafe impl Send for KcpSend{}

impl KcpSend{
    pub fn send(&self, mut buf: &[u8]) -> KcpResult<usize>{
        unsafe {
            (*self.0.get()).send(buf)
        }
    }
}

pub struct KcpUpdate(Arc<UnsafeCell<Kcp>>);
unsafe impl Sync for KcpUpdate{}
unsafe impl Send for KcpUpdate{}

impl KcpUpdate{
    pub async fn update(&self, current: u32) -> KcpResult<()>{
        unsafe {
            let x=self.0.get();
            (*x).update(current).await
        }
    }

    pub async fn flush(&self) -> KcpResult<()>{
        unsafe {
            (*self.0.get()).flush().await
        }
    }

    pub async fn flush_async(&self)-> KcpResult<()>{
        unsafe {
            (*self.0.get()).flush_async().await
        }
    }
}

impl Kcp{
    pub fn split(self)->(KcpInputRecv,KcpSend,KcpUpdate){
        let recv = Arc::new(UnsafeCell::new(self));
        let send = recv.clone();
        let update = recv.clone();

        (KcpInputRecv(recv),KcpSend(send),KcpUpdate(update))
    }
}



/// KCP Peer
/// UDP的包进入 KCP PEER 经过KCP 处理后 输出
/// 输入的包 进入KCP PEER处理,然后 输出到UDP PEER SEND TO
/// 同时还需要一个UPDATE 线程 去10MS 一次的运行KCP UPDATE
/// token 用于扩赞逻辑上下文
pub struct KcpPeer<T: Send> {
    pub kcp_recv:Arc<Mutex<KcpInputRecv>>,
    pub kcp_send:Arc<Mutex<KcpSend>>,
    pub kcp_update:Arc<Mutex<KcpUpdate>>,
    pub conv: u32,
    pub addr: String,
    pub token: Mutex<TokenStore<T>>,
    pub last_rev_time: AtomicI64
}

unsafe impl<T: Send> Send for KcpPeer<T>{}
unsafe impl<T: Send> Sync for KcpPeer<T>{}