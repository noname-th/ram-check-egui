use std::process::Command;
use windows::Win32::System::SystemInformation::GetPhysicallyInstalledSystemMemory;
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_physical_kb: u64, // RAM ที่ติดตั้งจริง
    pub total_visible_kb: u64,  // RAM ที่แสดงทั้งหมด (จาก Windows API)
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            total_physical_kb: 0,
            total_visible_kb: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        // ดึงข้อมูล RAM จาก Windows API
        unsafe {
            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..Default::default()
            };

            match GlobalMemoryStatusEx(&mut mem_status) {
                Ok(_) => {
                    // รับค่าข้อมูล RAM ที่แสดงเป็น KB
                    self.total_visible_kb = mem_status.ullTotalPhys / 1024;

                    // รับค่าข้อมูล RAM ที่ติดตั้งจริง
                    GetPhysicallyInstalledSystemMemory(&mut self.total_physical_kb).map_err(
                        |e| format!("Failed to get physically installed memory: {:?}", e),
                    )?;

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
        if self.total_physical_kb == 0 {
            return false;
        }
        let half = self.total_physical_kb / 2;
        self.total_visible_kb < half
    }

    /// แปลง total_physical เป็น GB
    pub fn total_installed_gb(&self) -> f64 {
        self.total_physical_kb as f64 / 1_048_576.0
    }

    /// แปลง total_visible เป็น GB
    pub fn total_visible_gb(&self) -> f64 {
        self.total_visible_kb as f64 / 1_048_576.0
    }

    /// ฟังก์ชันสำหรับแก้ไขปัญหา RAM
    pub fn fix_ram_issue(&self) {
        // println!("Fixing RAM issue...");
        // println!("Total Physical: {:.2} GB", self.total_installed_gb());
        // println!("Total Visible: {:.2} GB", self.total_visible_gb());
        // Restart the computer
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(&["/C", "shutdown /r /t 0 "])
                .spawn()
                .expect("Failed to restart the computer");
        }
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self::new()
    }
}
