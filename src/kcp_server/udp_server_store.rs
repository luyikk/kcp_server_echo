use super::udp_listener::UdpListener;

/// 用来存储 UDP SERVER
pub struct UdpServerStore(pub Option<Box<dyn UdpListener>>);

impl UdpServerStore {
    /// 是否有值
    pub fn have(&self) -> bool {
        match &self.0 {
            None => false,
            Some(_) => true,
        }
    }

    /// 获取
    pub fn get(&mut self) -> Option<&mut Box<dyn UdpListener>> {
        match self.0 {
            None => None,
            Some(ref mut v) => Some(v),
        }
    }

    /// 设置
    pub fn set(&mut self, v: Box<dyn UdpListener>) {
        self.0 = Some(v);
    }
}
