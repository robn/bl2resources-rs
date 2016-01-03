use std::collections::BTreeMap;
use std::error::Error;
use rustc_serialize::json::{Json,ToJson};

use json::{FromJson,FromJsonField,ToJsonField,ParseError};

json_struct!(AssetSublibraryDefinition, "AssetSublibraryDefinition",
    description: String         => "description",
    package:     Option<String> => "package",
    assets:      Vec<String>    => "assets"
);

json_struct!(AssetLibraryDefinition, "AssetLibraryDefinition",
    typ:          Option<String>                 => "type",
    sublibraries: Vec<AssetSublibraryDefinition> => "sublibraries"
);

json_struct!(AssetLibrarySet, "AssetLibrarySet",
    id:        usize                                   => "id",
    libraries: BTreeMap<String,AssetLibraryDefinition> => "libraries"
);

json_struct!(AssetLibraryConfig, "AssetLibraryConfig",
    sublibrary_bits: usize  => "sublibrary_bits",
    asset_bits:      usize  => "asset_bits",
    typ:             String => "type"
);

impl AssetLibraryConfig {
    pub fn none_index(&self) -> usize {
        (1 << (self.sublibrary_bits + self.asset_bits)) - 1
    }
    pub fn sublibrary_mask(&self) -> usize {
        (1 << (self.sublibrary_bits - 1)) - 1
    }
    pub fn use_set_id_mask(&self) -> usize {
        1 << (self.sublibrary_bits - 1)
    }
    pub fn asset_mask(&self) -> usize {
        (1 << self.asset_bits) - 1
    }
}

json_struct!(AssetLibraryManager, "AssetLibraryManager",
    version: usize                               => "version",
    configs: BTreeMap<String,AssetLibraryConfig> => "configs",
    sets:    Vec<AssetLibrarySet>                => "sets"
);

data!(AssetLibraryManager, "Asset Library Manager.json");
