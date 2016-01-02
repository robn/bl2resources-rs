extern crate rustc_serialize;

#[macro_use] mod json;
#[macro_use] mod data;

pub mod asset_library_manager;

pub use asset_library_manager::AssetLibraryManager;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn asset_library_manager() {
        let alm = AssetLibraryManager::data();
        println!("{:#?}", alm);
        assert!(alm.is_ok());
    }
}
