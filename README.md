==custom build script==
see Cargo.toml
```
build = "build/build.rs" # defined custom build
```
use tera template generate some files, entity/src/meta_init.rs、entity/src/lib.rs、api/src/api_register.rs、api/src/lib.rs、service/src/lib.rs, if you want change, change there build/templates/*.tera. 