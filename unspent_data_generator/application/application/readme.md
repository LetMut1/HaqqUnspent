
# Docker
## Services
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_generator/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [data_proxy.tcp.socket_address]

    [data_proxy.security.server_access_token]

    [resource.postgresql.configuration] - База данных Постгрес, исходная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено.
    ```

2. Запустить контейнер /$ROOT_PROJECT_DIR/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile. (https://github.com/unspentio/data_generator/blob/master/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile).

    Из-под контейнера выполнить команды:
    ```
    1. cargo run --manifest-path=/unspentio/application/application/Cargo.toml --release --bin=data_generator generate_asset_snapshot &

    2. cargo run --manifest-path=/unspentio/application/application/Cargo.toml --release --bin=data_generator generate_balance_snapshot &
    ```

# Manual
## Infrastructure
### Production/Development/Local-development environment
1. Установить Rust и его компоненты.

    Необходимая версия:
    /$ROOT_PROJECT_DIR/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile
    (https://github.com/unspentio/data_generator/blob/master/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile)

## Services
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_generator/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [data_proxy.tcp.socket_address]

    [data_proxy.security.server_access_token]

    [resource.postgresql.configuration] - База данных Постгрес, исходная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено.
    ```
2. Выполнить команды, заменив параметры подключения и $ROOT_PROJECT_DIR:
    ```
    1. cargo run --manifest-path=/$ROOT_PROJECT_DIR/application/application/Cargo.toml --release --bin=data_generator generate_asset_snapshot &

    2. cargo run --manifest-path=/$ROOT_PROJECT_DIR/application/application/Cargo.toml --release --bin=data_generator generate_balance_snapshot &
    ```