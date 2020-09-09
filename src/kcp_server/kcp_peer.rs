use tokio::sync::Mutex;
use udp_server:: TokenStore;
use crate::kcp::{Kcp, KcpResult};
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use std::net::SocketAddr;
use std::future::Future;

/// KCP INPUT
/// 锁拆分 将 peeksize input recv 放在一个锁里
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


/// KCPSend
/// 将Send 有关的操作 锁此拆分
pub struct KcpSend(Arc<UnsafeCell<Kcp>>);
unsafe impl Sync for KcpSend{}
unsafe impl Send for KcpSend{}

impl KcpSend{
    pub fn send(&self,  buf: &[u8]) -> KcpResult<usize>{
        unsafe {
            (*self.0.get()).send(buf)
        }
    }

    pub fn update(&self, current: u32) -> impl Future<Output = KcpResult<()>> {
        unsafe {
            (*self.0.get()).update(current)
        }
    }

    pub fn flush(&self) -> impl Future<Output = KcpResult<()>>{
        unsafe {
            (*self.0.get()).flush()
        }
    }

    pub async fn flush_async(&self)-> impl Future<Output = KcpResult<()>>{
        unsafe {
            (*self.0.get()).flush_async()
        }
    }
}


impl Kcp{
    pub fn split(self)->(KcpInputRecv,KcpSend){
        let recv = Arc::new(UnsafeCell::new(self));
        let send = recv.clone();

        (KcpInputRecv(recv),KcpSend(send))
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
    pub conv: u32,
    pub addr: SocketAddr,
    pub token: Mutex<TokenStore<T>>,
    pub last_rev_time: AtomicI64
}

unsafe impl<T: Send> Send for KcpPeer<T>{}
unsafe impl<T: Send> Sync for KcpPeer<T>{}