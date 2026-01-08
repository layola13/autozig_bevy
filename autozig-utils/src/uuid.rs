//! UUID的Rust包装层 - 90% Zig实现

use autozig::include_zig;
use std::fmt;

// UUID结构体，与Zig的extern struct保持一致
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    bytes: [u8; 16],
}

// 引入Zig实现的UUID函数
include_zig!("src/zig/uuid.zig", {
    fn uuid_new() -> Uuid;
    fn uuid_from_bytes(bytes: *const [u8; 16]) -> Uuid;
    fn uuid_to_string(uuid: Uuid, buffer: *mut [u8; 36]);
    fn uuid_from_string(str_ptr: *const u8, str_len: usize) -> Uuid;
    fn uuid_equal(a: Uuid, b: Uuid) -> bool;
    fn uuid_to_u128(uuid: Uuid) -> u128;
    fn uuid_from_u128(value: u128) -> Uuid;
    fn uuid_get_bytes(uuid: Uuid, out_bytes: *mut [u8; 16]);
});

impl Uuid {
    /// 生成新的UUID v4 (随机)
    pub fn new() -> Self {
        uuid_new()
    }
    
    /// 从字节数组创建UUID
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        uuid_from_bytes(&bytes)
    }
    
    /// 从字符串解析UUID
    /// 
    /// 支持标准格式: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 36 {
            return None;
        }
        
        let uuid = uuid_from_string(s.as_ptr(), s.len());
        
        // 检查是否解析成功（失败时返回零UUID）
        if uuid.bytes == [0u8; 16] && s != "00000000-0000-0000-0000-000000000000" {
            None
        } else {
            Some(uuid)
        }
    }
    
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        let mut buffer = [0u8; 36];
        uuid_to_string(*self, &mut buffer);
        String::from_utf8(buffer.to_vec()).expect("UUID string is always valid UTF-8")
    }
    
    /// 获取字节数组
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
    
    /// 转换为u128
    pub fn as_u128(&self) -> u128 {
        uuid_to_u128(*self)
    }
    
    /// 从u128创建UUID
    pub fn from_u128(value: u128) -> Self {
        uuid_from_u128(value)
    }
    
    /// 零UUID（全零字节）
    pub const fn nil() -> Self {
        Self { bytes: [0u8; 16] }
    }
    
    /// 检查是否为零UUID
    pub fn is_nil(&self) -> bool {
        self.bytes == [0u8; 16]
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uuid({})", self.to_string())
    }
}

impl std::str::FromStr for Uuid {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or("Invalid UUID format")
    }
}

// UUID是线程安全的
unsafe impl Send for Uuid {}
unsafe impl Sync for Uuid {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uuid_creation() {
        let uuid = Uuid::new();
        assert!(!uuid.is_nil());
        
        // 验证版本位 (v4)
        assert_eq!(uuid.bytes[6] & 0xf0, 0x40);
        
        // 验证变体位 (RFC4122)
        assert_eq!(uuid.bytes[8] & 0xc0, 0x80);
    }
    
    #[test]
    fn test_uuid_nil() {
        let uuid = Uuid::nil();
        assert!(uuid.is_nil());
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
    }
    
    #[test]
    fn test_uuid_string_conversion() {
        let uuid1 = Uuid::new();
        let s = uuid1.to_string();
        
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().nth(8), Some('-'));
        assert_eq!(s.chars().nth(13), Some('-'));
        assert_eq!(s.chars().nth(18), Some('-'));
        assert_eq!(s.chars().nth(23), Some('-'));
        
        let uuid2 = Uuid::from_str(&s).unwrap();
        assert_eq!(uuid1, uuid2);
    }
    
    #[test]
    fn test_uuid_bytes() {
        let bytes = [1, 2, 3, 4, 5, 6, 0x47, 8, 0x89, 10, 11, 12, 13, 14, 15, 16];
        let uuid = Uuid::from_bytes(bytes);
        
        assert_eq!(uuid.as_bytes(), &bytes);
    }
    
    #[test]
    fn test_uuid_u128_conversion() {
        let uuid1 = Uuid::new();
        let value = uuid1.as_u128();
        let uuid2 = Uuid::from_u128(value);
        
        assert_eq!(uuid1, uuid2);
    }
    
    #[test]
    fn test_uuid_equality() {
        let uuid1 = Uuid::new();
        let uuid2 = uuid1;
        
        // 添加微小延迟确保不同的随机种子
        std::thread::sleep(std::time::Duration::from_millis(2));
        let uuid3 = Uuid::new();
        
        assert_eq!(uuid1, uuid2);
        assert_ne!(uuid1, uuid3, "Two newly generated UUIDs should be different");
    }
    
    #[test]
    fn test_uuid_display() {
        let uuid = Uuid::new();
        let display = format!("{}", uuid);
        let debug = format!("{:?}", uuid);
        
        assert_eq!(display.len(), 36);
        assert!(debug.starts_with("Uuid("));
        assert!(debug.ends_with(")"));
    }
    
    #[test]
    fn test_uuid_from_str_invalid() {
        assert!(Uuid::from_str("invalid").is_none());
        assert!(Uuid::from_str("").is_none());
        assert!(Uuid::from_str("12345678-1234-1234-1234").is_none());
    }
}