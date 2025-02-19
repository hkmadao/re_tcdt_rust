**custom build script**
see Cargo.toml
```
build = "build/build.rs" # defined custom build
```
use tera template generate some files, entity/src/meta_init.rs、entity/src/lib.rs、api/src/api_register.rs、api/src/lib.rs、service/src/lib.rs, if you want change, change there build/templates/*.tera.

# Run
1. modify "conf/.env" file
2. create database in mysql or mariadb, import init sql from "init_sql" folder
3. run tcdt_rust or tcdt_rust.exe
4. see http://[your server ip]:[your server port]/tcdt/index.html, like: http://127.0.0.1:8800/tcdt/index.html

**set your username/password**
modify "conf/.env"
```
# SECURITY=true
SECURITY=false
```
restart server, you can assess server without login.   
create your access user info.   
don't forgot restore the config file and restart server.