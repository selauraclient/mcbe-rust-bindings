use std::{path::Path, sync::LazyLock};

use spinwait::SpinWait;
use windows::{
    Management::Core::ApplicationDataManager,
    Win32::{
        Foundation::ERROR_INSUFFICIENT_BUFFER, Storage::Packaging::Appx::GetPackagesByPackageFamily,
    },
    core::Result,
};

use crate::platform::windows::{App, CWSTR, Process};

static APP: LazyLock<App> =
    LazyLock::new(|| App::new("Microsoft.MinecraftUWP_8wekyb3d8bbwe!App").unwrap());

static PACKAGE: LazyLock<CWSTR> =
    LazyLock::new(|| CWSTR::new("Microsoft.MinecraftUWP_8wekyb3d8bbwe"));

pub struct Game;

impl Game {
    pub(crate) fn activate() -> Result<Process> {
        let string = format!(
            "{}{}",
            ApplicationDataManager::CreateForPackageFamily(&APP.package()?.Id()?.FamilyName()?)?
                .LocalFolder()?
                .Path()?,
            r"\games\com.mojang\minecraftpe\resource_init_lock"
        );
        let path = Path::new(&string);

        if !APP.running()? || path.exists() {
            let process = Process::new(APP.launch()?)?;
            let wait = SpinWait::new();
            let mut value = false;

            while process.running() {
                if value {
                    if !path.exists() {
                        break;
                    }
                } else {
                    value = path.exists()
                }
                wait.spin_once();
            }

            return Ok(process);
        }

        Process::new(APP.launch()?)
    }

    pub fn launch(value: bool) -> Result<u32> {
        Ok(if value {
            Self::activate()?.id
        } else {
            APP.launch()?
        })
    }

    pub fn debug(value: bool) -> Result<()> {
        APP.debug(value)
    }

    pub fn terminate() -> Result<()> {
        APP.terminate()
    }

    pub fn running() -> Result<bool> {
        APP.running()
    }

    pub fn unpackaged() -> Result<bool> {
        APP.package()?.IsDevelopmentMode()
    }

    pub fn installed() -> bool {
        let mut count = 0u32;
        let mut bufferlength = 0u32;

        unsafe {
            GetPackagesByPackageFamily(PACKAGE.0, &mut count, None, &mut bufferlength, None).0
                == ERROR_INSUFFICIENT_BUFFER.0
        }
    }
}