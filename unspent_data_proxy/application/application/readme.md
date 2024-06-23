
# Docker
## Infrastructure
### Production/Development/Local-development environment
1. В директории /$ROOT_PROJECT_DIR/development_environment/resource/docker выполнить:
    ```
    docker-compose up -d --build
    ```
## Data migration
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_proxy/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [resource.clickhouse....] - База данных Кликхаус. В коде есть поддержка Tls, поэтому value для .url должно быть "https://...:8443"

    [resource.postgresql.selecting.configuration] - База данных Постгрес, исходная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено в течение недели.

    [resource.postgresql.updating.configuration] - База данных Постгрес, временная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено в течение недели.
    ```

2. Запустить контейнер /$ROOT_PROJECT_DIR/development_environment/resource/docker/service/dbmate/dockerfile (https://github.com/unspentio/data_proxy/blob/master/development_environment/resource/docker/service/dbmate/dockerfile).

    Из-под контейнера выполнить команды, заменив параметры:
    ```
    1. Необходимо использовать порт 9440:

    dbmate -u clickhouse://root:password@clickhouse:9440/unspentio -d /unspentio/application/migration/clickhouse/migration --migrations-table migration_schema --no-dump-schema up

    2. dbmate -u postgres://root:password@postgresql:5432/unspentio?sslmode=disable -d /unspentio/application/migration/postgresql/migration --migrations-table migration_schema  --no-dump-schema up
    ```

3. Запустить контейнер /$ROOT_PROJECT_DIR/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile. (https://github.com/unspentio/data_proxy/blob/master/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile).

    Из-под контейнера выполнить команды:
    ```
    1. cargo run --manifest-path=/unspentio/application/application/Cargo.toml --release --bin=migrate_asset_snapshot &

    2. cargo run --manifest-path=/unspentio/application/application/Cargo.toml --release --bin=migrate_balance_snapshot &
    ```

## Services
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_proxy/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [tokio_runtime.worker_threads_quantity]

    [tokio_runtime.maximum_blocking_threads_quantity]

    [application_server.tcp.socket_address]

    [remote_service.user_authorization.url]

    [logging.directory_path]

    [security.server_access_token]

    [resource.clickhouse....] - База данных Кликхаус. В коде есть поддержка Tls, поэтому value для .url должно быть "https://...:8443"
    ```

2. Запустить контейнер /$ROOT_PROJECT_DIR/development_environment/resource/docker/service/dbmate/dockerfile (https://github.com/unspentio/data_proxy/blob/master/development_environment/resource/docker/service/dbmate/dockerfile).
    Из-под контейнера выполнить команды, заменив параметры:
    ```
    1. Необходимо использовать порт 9440:

    dbmate -u clickhouse://root:password@clickhouse:9440/unspentio -d /unspentio/application/migration/clickhouse/migration --migrations-table migration_schema --no-dump-schema up

    2. dbmate -u postgres://root:password@postgresql:5432/unspentio?sslmode=disable -d /unspentio/application/migration/postgresql/migration --migrations-table migration_schema  --no-dump-schema up
    ```
3. Запустить контейнер /$ROOT_PROJECT_DIR/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile. (https://github.com/unspentio/data_proxy/blob/master/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile).

    Из-под контейнера выполнить команды:
    ```
    1. cargo run --manifest-path=/unspentio/application/application/Cargo.toml --release --bin=run_server &
    ```

# Manual
## Infrastructure
### Production/Development/Local-development environment
1. Создать базу данных Кликхаус.

    Использовать стандартный файл конфигурации.

    Необходимая версия:
    /$ROOT_PROJECT_DIR/development_environment/resource/docker/service/clickhouse/dockerfile
    (https://github.com/unspentio/data_proxy/blob/master/development_environment/resource/docker/service/clickhouse/dockerfile)

2. Создать базу данных Постгрес.

    Использовать стандартный файл конфигурации.

    Необходимая версия:
    /$ROOT_PROJECT_DIR/development_environment/resource/docker/service/postgresql/dockerfile
    (https://github.com/unspentio/data_proxy/blob/master/development_environment/resource/docker/service/postgresql/dockerfile)

    База данных временна, и будет удалена после завершения процесса миграции данных.

3. Установить DbMate

    Необходимая версия:
    /$ROOT_PROJECT_DIR/development_environment/resource/docker/service/dbmate/dockerfile
    (https://github.com/unspentio/data_proxy/blob/master/development_environment/resource/docker/service/dbmate/dockerfile)

4. Установить Rust и его компоненты.

    Необходимая версия:
    /$ROOT_PROJECT_DIR/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile
    (https://github.com/unspentio/data_proxy/blob/master/development_environment/tool/rust/docker/service/rust_stable_toolchain/dockerfile)

## Data migration
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_proxy/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [resource.clickhouse....] - База данных Кликхаус. В коде есть поддержка Tls, поэтому value для .url должно быть "https://...:8443"

    [resource.postgresql.selecting.configuration] - База данных Постгрес, исходная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено в течение недели.

    [resource.postgresql.updating.configuration] - База данных Постгрес, временная. В коде нет поддержки Tls, поэтому value должно быть "host= port=5432 user= password= dbname= sslmode=disable".
    Это подключение временно, и будет удалено в течение недели.
    ```

2. Выполнить команды, заменив параметры и $ROOT_PROJECT_DIR:
    ```
    1. Необходимо использовать порт 9440:

    dbmate -u clickhouse://root:password@clickhouse:9440/unspentio -d /$ROOT_PROJECT_DIR/application/migration/clickhouse/migration --migrations-table migration_schema --no-dump-schema up

    2. dbmate -u postgres://root:password@postgresql:5432/unspentio?sslmode=disable -d /$ROOT_PROJECT_DIR/application/migration/postgresql/migration --migrations-table migration_schema  --no-dump-schema up

    3. cargo run --manifest-path=/$ROOT_PROJECT_DIR/application/application/Cargo.toml --release --bin=migrate_asset_snapshot &

    4. cargo run --manifest-path=/$ROOT_PROJECT_DIR/application/application/Cargo.toml --release --bin=migrate_balance_snapshot &
    ```

## Services
1. Скопировать содержимое файла /$ROOT_PROJECT_DIR/application/application/environment_configuration/environment.example.toml (https://github.com/unspentio/data_proxy/blob/master/application/application/environment_configuration/environment.example.toml) в файл /$ROOT_PROJECT_DIR/application/application/environment_configuration/$ENVIRONMENT/environment.toml, заменив параметры:
    ```
    [tokio_runtime.worker_threads_quantity]

    [tokio_runtime.maximum_blocking_threads_quantity]

    [application_server.tcp.socket_address]

    [remote_service.user_authorization.url]

    [logging.directory_path]

    [security.server_access_token]

    [resource.clickhouse....] - База данных Кликхаус. В коде есть поддержка Tls, поэтому value для .url должно быть "https://...:8443"
    ```

2. Выполнить команды, заменив параметры и $ROOT_PROJECT_DIR:
    ```
    1. Необходимо использовать порт 9440:

    dbmate -u clickhouse://root:password@clickhouse:9440/unspentio -d /$ROOT_PROJECT_DIR/application/migration/clickhouse/migration --migrations-table migration_schema --no-dump-schema up

    2. dbmate -u postgres://root:password@postgresql:5432/unspentio?sslmode=disable -d /$ROOT_PROJECT_DIR/application/migration/postgresql/migration --migrations-table migration_schema  --no-dump-schema up

    3. cargo run --manifest-path=/$ROOT_PROJECT_DIR/application/application/Cargo.toml --release --bin=run_server &
    ```