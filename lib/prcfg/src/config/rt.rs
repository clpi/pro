use serde::{de::Expected, Deserialize, Serialize};
use std::path::{Path, PathBuf};
use dirs::{config_dir, cache_dir, config_local_dir, data_dir, data_local_dir, executable_dir, home_dir};


/// Cfg on overall runtime (dev side)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct CfgRt {
    #[serde(default = "CfgRt::get_data_dirn")]
    pub data_dir: PathBuf,
    #[serde(default = "CfgRt::get_cache_dirn")]
    pub cache_d: PathBuf,
    #[serde(default = "CfgRt::get_rt_dirn")]
    pub rt_d: PathBuf,
    #[serde(default = "CfgRt::get_exe_dirn")]
    pub exe_d: PathBuf,
}
impl Default for CfgRt {
    fn default() -> Self {
        Self {
            data_dir: Self::get_data_dirn(),
            cache_d: Self::get_cache_dirn(),
            rt_d: Self::get_rt_dirn(),
            exe_d: Self::get_exe_dirn(),
        }
    }
}
impl CfgRt {
    pub fn get_data_dirn() -> PathBuf {
        dirs::data_dir()
            .or(dirs::data_local_dir())
            .or(dirs::preference_dir())
            .or(dirs::cache_dir())
            .unwrap_or_default()
    }
    pub fn get_data_dir(fb: impl Into<PathBuf>) -> PathBuf {
        dirs::data_dir().or(dirs::data_local_dir()).unwrap_or(fb.into())
    }

    pub fn get_exe_dirn() -> PathBuf {
        dirs::executable_dir().or(dirs::data_dir())
            .or(dirs::data_local_dir())
            .or(dirs::cache_dir())
            .or(dirs::state_dir())
            .or(dirs::runtime_dir())
            .unwrap_or_default()
    }
    pub fn get_exe_dir(fb: impl Into<PathBuf>) -> PathBuf {
        dirs::executable_dir().or(dirs::data_dir())
            .or(dirs::data_local_dir())
            .or(dirs::cache_dir())
            .or(dirs::state_dir())
            .or(dirs::runtime_dir())
            .unwrap_or(fb.into())
    }

    pub fn get_cache_dirn() -> PathBuf {
        dirs::cache_dir().or(dirs::data_local_dir())
            .or(dirs::data_dir())
            .or(dirs::config_local_dir())
            .unwrap_or_default()
    }
    pub fn get_cache_dir(fb: impl Into<PathBuf>) -> PathBuf {
        dirs::cache_dir().or(dirs::data_local_dir())
            .or(dirs::data_dir())
            .or(dirs::config_local_dir())
            .unwrap_or(fb.into())
    }

    pub fn get_rt_dirn() -> PathBuf {
        dirs::runtime_dir()
            .or(dirs::state_dir())
            .unwrap_or_default()
    }

    pub fn get_rt_dir(fb: impl Into<PathBuf>) -> PathBuf {
        dirs::runtime_dir().or(dirs::state_dir())
            .unwrap_or(fb.into())
    }

}