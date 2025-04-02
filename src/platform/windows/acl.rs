use windows::Win32::Security::ACL;

pub struct Acl(pub *mut ACL);

unsafe impl Sync for Acl {}

unsafe impl Send for Acl {}