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

json_struct!(AssetLibraryManager, "AssetLibraryManager",
    version: usize                               => "version",
    configs: BTreeMap<String,AssetLibraryConfig> => "configs",
    sets:    Vec<AssetLibrarySet>                => "sets"
);

data!(AssetLibraryManager, "Asset Library Manager.json");
