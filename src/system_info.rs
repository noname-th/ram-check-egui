use ::windows::Win32::System::SystemInformation::GetPhysicallyInstalledSystemMemory;
use std::process::Command;
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_physical: u64, // RAM ที่ติดตั้งจริง
    pub total_visible: u64,  // RAM ที่แสดงทั้งหมด (จาก Windows API)
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            total_physical: 0,
            total_visible: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        unsafe {
            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..Default::default()
            };

            match GlobalMemoryStatusEx(&mut mem_status) {
                Ok(_) => {
                    self.total_visible = mem_status.ullTotalPhys;

                    let mut totlal_physical_kb: u64 = 0;
                    if (GetPhysicallyInstalledSystemMemory(&mut totlal_physical_kb)).is_ok() {
                        self.total_physical = totlal_physical_kb * 1024; // แปลงจาก KB เป็น Bytes
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Failed to get memory info: {:?}", e)),
            }
        }
    }

    /// ตรวจสอบว่า RAM ที่แสดงมากกว่าครึ่งหนึ่งของ RAM ที่ติดตั้งหรือไม่
    /// ถ้ามากกว่าครึ่ง = ปกติ (false)
    /// ถ้าน้อยกว่าครึ่ง = มีปัญหา (true)
    pub fn has_problem(&self) -> bool {
        if self.total_physical == 0 {
            return false;
        }
        let half = self.total_physical / 2;
        self.total_visible < half
    }

    /// คำนวณอัตราส่วนของ RAM ที่แสดงต่อ RAM ที่ติดตั้ง (เป็นเปอร์เซ็นต์)
    pub fn visible_ratio(&self) -> f32 {
        if self.total_physical == 0 {
            return 0.0;
        }
        (self.total_visible as f64 / self.total_physical as f64 * 100.0) as f32
    }

    /// แปลง total_physical เป็น GB
    pub fn total_installed_gb(&self) -> f64 {
        self.total_physical as f64 / 1_073_741_824.0
    }

    /// แปลง total_visible เป็น GB
    pub fn total_visible_gb(&self) -> f64 {
        self.total_visible as f64 / 1_073_741_824.0
    }

    /// Abstract function: ฟังก์ชันสำหรับแก้ไขปัญหา RAM
    /// สามารถ implement ตามต้องการ
    pub fn fix_ram_issue(&self) {
        // TODO: ใส่โค้ดที่ต้องการให้ทำงานเมื่อพบปัญหา RAM
        // เช่น รีสตาร์ท, ล้าง cache, ฯลฯ
        println!("Fixing RAM issue...");
        println!("Total Physical: {:.2} GB", self.total_installed_gb());
        println!("Total Visible: {:.2} GB", self.total_visible_gb());
        println!("Ratio: {:.1}%", self.visible_ratio());
        // Restart the computer
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(&[
                    "/C",
                    "shutdown /r /t 0 /c \"RAM issue detected, restarting...\"",
                ])
                .spawn();
        }
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self::new()
    }
}
