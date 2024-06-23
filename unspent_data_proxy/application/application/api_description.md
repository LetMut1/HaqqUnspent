# Standards
## Request
 - All data is transferred in `HTTP body` as `bytes` in encoded with `json` form.
 - Every request should contain this `HTTP header`s:
```
- content-type: application/octet-stream
- content-length: --number--
```
## Response
- All data is transferred in `HTTP body` as `bytes` in encoded with `json` form.
- Every response will contain this `HTTP header`s:
```
- content-type: application/octet-stream
- content-length: --number--
```
 - The permanent general structure of the each response with `HTTP status code` equal to `200` looks like:
```
enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}

enum Data<T> {
    Empty,
    Filled {
        data: T,
    },
}
```
```
json, all cases:

{"Target":{"data": "Empty"}}
{"Target":{"data":{"Filled":{"data": ____ }}}}
{"Precedent":{"precedent": ____ }}
```
- `HTTP status code` unequal to `200` have not got `HTTP body`.
### Exclusion
 - Some endpoints for monitoring purpose do not comply with these rules due to existing restrictions.
<br/>
<br/>

# API
## /balance_snapshot/create POST
`Request`
 - data:
```
struct Incoming {
    server_access_token: String,
    balance_snapshot_registry: Vec<BalanceSnapshot>,
}

struct BalanceSnapshot {
    user_id: i32,
    asset_id: String,
    total_amount: String,
    created_at: u32,
}
```
 - validation:
```
`server_access_token`

`total_amount`:
    - string representation of float number greater than zero.
```
 - json, example:
```
Incoming:
'{"server_access_token": "q", "balance_snapshot_registry":[{"user_id":1, "asset_id": "btc", "total_amount": "0.123", "created_at": 1111111}]}'
```
`Response`
 - data: absent.
## /balance_snapshot/history POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    reference_asset_id: String,
    range: String,
}
```
 - validation:
```
`access_token`

`range`:
    - one of ["1d", "7d", "1m", "1y", "all"].
```
 - json, example:
```
Incoming:
'{"access_token": "q", "reference_asset_id": "btc", "range": "1d"}'
```
`Response`
 - data:
```
struct Outcoming {
    balance_snapshot_registry: Vec<BalanceSnapshot>,
}

struct BalanceSnapshot {
    btc_value: String,
    fiat_value: String,
    timestamp: u32,
}
```
 - json, example:
```
Outcoming:
'{"balance_snapshot_registry":[{"btc_value": "1.123", "fiat_value": "12.123", "timestamp": 1111111}]}'
```
## /base_balance_snapshot/create POST
`Request`
 - data:
```
struct Incoming {
    server_access_token: String,
    base_balance_snapshot_registry: Vec<BaseBalanceSnapshot>,
}

struct BaseBalanceSnapshot_ {
    user_id: i32,
    exchange_id: Option<String>,
    exchange_name: Option<String>,
    wallet_id: Option<i32>,
    wallet_address: Option<String>,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
    amount: String,
    created_at: u32,
}
```
 - validation:
```
`server_access_token`

`amount`:
    - string representation of float number greater than zero.
```
 - json, example:
```
Incoming:
'{"server_access_token": "q", "base_balance_snapshot_registry":[{"user_id":1, "exchange_id":null, "exchange_name": null, "wallet_id": 12345, "wallet_address": "q", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc", "amount": "0.123", "created_at": 1111111}]}'
```
`Response`
 - data: absent.
## /subportfolio_base_balance_snapshot/history POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
    reference_asset_id: String,
    range: String,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.

`range`:
    - one of ["1d", "7d", "1m", "1y", "all"].
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "reference_asset_id": "btc", "range": "1d"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_base_balance_snapshot_registry: Vec<BalanceSnapshot>,
}

struct BalanceSnapshot {
    btc_value: String,
    fiat_value: String,
    timestamp: u32,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_base_balance_snapshot_registry":[{"btc_value": "1.123", "fiat_value": "12.123", "timestamp": 1111111}]}'
```
## /subportfolio_base_balance_snapshot/history_for_subportfolio_link POST (GET)
`Request`
 - data:
```
struct Incoming {
    subportfolio_link_id: String,
    reference_asset_id: String,
    range: String,
}
```
 - validation:
```
`subportfolio_link_id`:
    - classic string representation of uuid.

`range`:
    - one of ["1d", "7d", "1m", "1y", "all"].
```
 - json, example:
```
Incoming:
'{"subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000", "reference_asset_id": "btc", "range": "1d"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_base_balance_snapshot_registry: Vec<BalanceSnapshot_>
}

struct BalanceSnapshot {
    btc_value: String,
    fiat_value: String,
    timestamp: u32,
}

enum Precedent {
    SubportfolioLink_DoesNotExist,
    SubportfolioLink_IsNotActive,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_base_balance_snapshot_registry":[{"btc_value": "1.123", "fiat_value": "12.123", "timestamp": 1111111}]}'

Precedent:
'{"SubportfolioLink_DoesNotExist"}'
```
## /asset_snapshot/create POST
`Request`
 - data:
```
struct Incoming {
    server_access_token: AccessKey,
    asset_snapshot_registry: Vec<AssetSnapshot>,
}

struct AssetSnapshot {
    asset_id: String,
    price_usd: String,
    price_btc: Option<String>,
    created_at: u32,
}
```
 - validation:
```
`server_access_token`

`price_usd`:
    - string representation of float number greater than zero.

`price_btc`:
    - string representation of float number greater than zero.
```
 - json, example:
```
Incoming:
'{"server_access_token": "q", "asset_snapshot_registry":[{"asset_id": "btc",  "price_usd": "0.123", "price_btc": "0.00001", "created_at": 1111111}]}'
```
`Response` data: absent.
## /asset_snapshot/history POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    range: String,
    asset_id_registry: Vec<String>
}
```
 - validation:
```
`access_token`

`range`:
    - one of ["1d", "7d", "1m", "1y"]
```
 - json, example:
```
Incoming:
'{"access_token": "q", "range": "1d", "asset_id_registry": ["btc"]}'
```
`Response`
 - data:
```
struct Outcoming {
    asset_snapshot_history_registry: Vec<AssetSnapshotHistory>,
}

struct AssetSnapshotHistory {
    asset_id: String,
    asset_price_usd_registry: Vec<AssetData>
}

struct AssetData {
    price_usd: String,
    timestamp: u32,
}
```
 - json, example:
```
Outcoming:
'{"asset_snapshot_history_registry":[{"asset_id": "btc", "asset_price_usd_registry": [{"price_usd": "0.123", "timestamp": 1111111}]}]}'
```
## /asset_snapshot/history_for_subportfolio_link POST (GET)
`Request`
 - data:
```
struct Incoming {
    subportfolio_link_id: String,
    range: String,
    asset_id_registry: Vec<String>
}
```
 - validation:
```
`subportfolio_link_id`:
    - classic string representation of uuid.

`range`:
    - one of ["1d", "7d", "1m", "1y"]
```
 - json, example:
```
Incoming:
'{"subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000", "range": "1d", "asset_id_registry": ["btc"]}'
```
`Response`
 - data:
```
struct Outcoming {
    asset_snapshot_history_registry: Vec<AssetSnapshotHistory>,
}

struct AssetSnapshotHistory {
    asset_id: String,
    asset_price_usd_registry: Vec<AssetData>
}

struct AssetData {
    price_usd: String,
    timestamp: u32,
}

enum Precedent {
    SubportfolioLink_DoesNotExist,
    SubportfolioLink_IsNotActive,
}
```
 - json, example:
```
Outcoming:
'{"asset_snapshot_history_registry":[{"asset_id": "btc", "asset_price_usd_registry": [{"price_usd": "0.123", "timestamp": 1111111}]}]}'

Precedent:
'{"SubportfolioLink_DoesNotExist"}'
```
## /asset_snapshot/history_for_price_difference_percentage_calculating POST (GET)
`Request`
 - data:
```
struct Incoming {
    server_access_token: String,
    asset_id_registry: Vec<String>,
}
```
 - validation:
```
`server_access_token`
```
 - json, example:
```
Incoming:
'{"server_access_token": "q", "asset_id_registry": ["btc"]}'
```
`Response`
 - data:
```
struct Outcoming {
    asset_snapshot_for_price_difference_percentage_calculating_registry: Vec<AssetSnapshotForPriceDifferencePercentageCalculating>,
}

struct AssetSnapshotForPriceDifferencePercentageCalculating {
    asset_id: String,
    price_usd_24_hours: String,
    price_btc_24_hours: Option<String>,
    price_usd_7_days: Option<String>,
    price_usd_30_days: Option<String>,
    price_usd_1_year: Option<String>,
}
```
 - json, example:
```
Outcoming:
'{"asset_snapshot_history_registry":[{"asset_id": "btc", "asset_price_usd_registry": [{"price_usd": "0.123", "timestamp": 1111111}]}]}'
```
## /subportfolio/create POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_name: String,
    subportfolio_description: Option<String>
}
```
 - validation:
```
`access_token`

`subportfolio_name`:
    - maximum length is 128 bytes.
    - without extra spaces.
    - is not empty.

`subportfolio_description`:
    - maximum length is 10240 bytes.
    - without extra spaces.
    - is not empty.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_name": "qwerty", "subportfolio_description": null}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_id: String,
}

enum Precedent {
    Subportfolio_MaximumQuantityPerUser,
    Subportfolio_AlreadyExists,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_id": "550e8400-e29b-41d4-a716-446655440000"}'

Precedent:
'{"Subportfolio_MaximumQuantityPerUser"}'
```
## /subportfolio/delete POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
enum Precedent {
    Subportfolio_DoesNotExist,
}
```
 - json, example:
```
Precedent:
'{"Subportfolio_DoesNotExist"}'
```
## /subportfolio/update POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
    subportfolio_name: Option<String>,
    subportfolio_description: Option<SerializationLayer<Option<Subportfolio_Description>>>
}

struct SerializationLayer<T> {
    data: T
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.

`subportfolio_name`:
    - maximum length is 128 bytes.
    - without extra spaces.
    - is not empty.

`subportfolio_description`:
    - maximum length is 10240 bytes.
    - without extra spaces.
    - is not empty.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_name": "qwerty", "subportfolio_description": null}'
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_name": "qwertyqwerty", "subportfolio_description": {"data":null}}
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_name": null, "subportfolio_description": {"data": "long description"}}
```
`Response`
 - data:
```
enum Precedent {
    Subportfolio_DoesNotExist,
    Subportfolio_AlreadyExists,
}
```
 - json, example:
```
Precedent:
'{"Subportfolio_DoesNotExist"}'
```
## /subportfolio/all POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
}
```
 - validation:
```
`access_token`
```
 - json, example:
```
Incoming:
'{"access_token": "q"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_registry: Vec<Subportfolio>,
}

struct Subportfolio {
    id: String,
    name: String,
    description: Option<String>,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_registry": [{"id": "550e8400-e29b-41d4-a716-446655440000", "name": "qwerty", "description": null}]}'
```
## /subportfolio_asset/update POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
    asset_registry_for_creating: Vec<Asset>,
    asset_registry_for_deleting: Vec<Asset>,
}

struct Asset {
    exchange_id: Option<String>,
    exchange_name: Option<String>,
    wallet_id: Option<i32>,
    wallet_address: Option<String>,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "asset_registry_for_creating": ["exchange_id":null, "exchange_name": null, "wallet_id": 12345, "wallet_address": "q", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc"],"asset_registry_for_deleting": ["exchange_id":null, "exchange_name": null, "wallet_id": 123123, "wallet_address": "w", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc"]}'
```
`Response`
 - data:
```
enum Precedent {
    Subportfolio_DoesNotExist,
    SubportfolioAsset_MaximumQuantityPerSubportfolio,
    SubportfolioAsset_AlreadyExist {
        asset_registry: Vec<Asset>
    },
    SubportfolioAsset_DoesNotExist {
        asset_registry: Vec<Asset>
    }
}

struct Asset {
    exchange_id: Option<String>,
    exchange_name: Option<String>,
    wallet_id: Option<i32>,
    wallet_address: Option<String>,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
}
```
 - json, example:
```
Precedent:
'{"SubportfolioAsset_MaximumQuantityPerSubportfolio"}'
'{"SubportfolioAsset_SubportfolioAsset_AlreadyExist":{"asset_registry": ["exchange_id":null, "exchange_name": null, "wallet_id": 12345, "wallet_address": "q", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc"]}}'
'{"SubportfolioAsset_MaximumQuantityPerSubportfolio"}'
'{"SubportfolioAsset_DoesNotExist": {"exchange_id":null, "exchange_name": null, "wallet_id": 123123, "wallet_address": "w", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc"}}'
```
## /subportfolio_asset/all_for_subportfolio POST (GET)
`Request`
 - data:
```
Incoming {
    access_token: String,
    subportfolio_id: String,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_asset_registry: Vec<SubportfolioAsset>,
}

struct SubportfolioAsset {
    exchange_id: Option<String>,
    exchange_name: Option<String>,
    wallet_id: Option<i32>,
    wallet_address: Option<String>,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
    created_at: u32,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_asset_registry": [{"exchange_id":null, "exchange_name": null, "wallet_id": 12345, "wallet_address": "q", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc", "created_at": 1111111}]}'
```
## /subportfolio_asset/all_for_subportfolio_link POST (GET)
`Request`
 - data:
```
Incoming {
    subportfolio_link_id: String,
}
```
 - validation:
```
`subportfolio_link_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_asset_registry: Vec<SubportfolioAsset>,
}

struct SubportfolioAsset {
    exchange_id: Option<String>,
    exchange_name: Option<String>,
    wallet_id: Option<i32>,
    wallet_address: Option<String>,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
    created_at: u32,
}

enum Precedent {
    SubportfolioLink_DoesNotExist,
    SubportfolioLink_IsNotActive,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_asset_registry": [{"exchange_id":null, "exchange_name": null, "wallet_id": 12345, "wallet_address": "q", "wallet_label":null, "asset_network": "ethereum", "asset_chain_id":1, "asset_id": "btc", "created_at": 1111111}]}'

Precedent:
'{"SubportfolioLink_DoesNotExist"}'
```
## /subportfolio_link/create POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
    subportfolio_link_description: Option<String>,
}
```
 - validation:
```
`access_token`

`subportfolio_link_description`:
    - maximum length is 10240 bytes.
    - without extra spaces.
    - is not empty.

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_link_description": null}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_link_id: String,
}

enum Precedent {
    Subportfolio_DoesNotExist,
    SubportfolioLink_MaximumQuantityPerUserAndSubportfolio,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000"}'

Precedent:
'{"SubportfolioLink_MaximumQuantityPerUserAndSubportfolio"}'
```
## /subportfolio_link/delete POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_link_id: String,
}
```
 - validation:
```
`access_token`

`subportfolio_link_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
enum Precedent {
    SubportfolioLink_DoesNotExist,
}
```
 - json, example:
```
Precedent:
'{"SubportfolioLink_DoesNotExist"}'
```
## /subportfolio_link/update POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_link_id: String,
    subportfolio_link_is_active: Option<bool>,
    subportfolio_link_description: Option<SerializationLayer<Option<String>>>,
}

struct SerializationLayer<T> {
    data: T
}
```
 - validation:
```
`access_token`

`subportfolio_link_id`:
    - classic string representation of uuid.

`subportfolio_link_description`:
    - maximum length is 10240 bytes.
    - without extra spaces.
    - is not empty.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_link_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_link_is_active": true, "subportfolio_link_description": null}'
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_link_is_active": false, "subportfolio_link_description": {"data":null}}
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_link_is_active": null, "subportfolio_link_description": {"data": "long description"}}
```
`Response`
 - data:
```
enum Precedent {
    SubportfolioLink_DoesNotExist
}
```
 - json, example:
```
Precedent:
'{"SubportfolioLink_DoesNotExist"}'
```
## /subportfolio_link/all POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_link_registry: Vec<SubportfolioLink>
}

struct SubportfolioLink {
    id: String,
    is_active: bool,
    description: Option<String>,
    created_at: u32,
}

enum Precedent {
    Subportfolio_DoesNotExist,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_link_registry": [{"id": "550e8400-e29b-41d4-a716-446655440000", "is_active": true, "description": null, "created_at": 1111}]}'

Precedent:
'{"Subportfolio_DoesNotExist"}'
```
## /subportfolio_trackable_wallet/all POST (GET)
`Request`
 - data:
```
struct Incoming {
    server_access_token: String,
    user_id: Option<i32>,
    limit: i16,
}
```
 - validation:
```
`server_access_token`
```
 - json, example:
```
Incoming:
'{"server_access_token": "q", "user_id": null, "limit": 100}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_trackable_wallet_aggregated_registry: Vec<SubportfolioTrackableWalletAggregated>,
}

struct SubportfolioTrackableWalletAggregated {
    user_id: i32,
    subportfolio_id: String,
    wallet_id_registry: Vec<i32>,
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_trackable_wallet_aggregated_registry": [{"user_id": 1, "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "wallet_id_registry": [1]}]}'
```
## /subportfolio_trackable_wallet/all_for_subportfolio POST (GET)
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000"}'
```
`Response`
 - data:
```
struct Outcoming {
    subportfolio_trackable_wallet_wallet_id_registry: Vec<i32>,
}

enum Precedent {
    Subportfolio_DoesNotExist
}
```
 - json, example:
```
Outcoming:
'{"subportfolio_trackable_wallet_wallet_id_registry": [1]}'

Precedent:
'{"Subportfolio_DoesNotExist"}'
```
## /subportfolio_trackable_wallet/update POST
`Request`
 - data:
```
struct Incoming {
    access_token: String,
    subportfolio_id: String,
    subportfolio_trackable_wallet_wallet_id_registry_for_creating: Vec<i32>,
    subportfolio_trackable_wallet_wallet_id_registry_for_deleting: Vec<i32>,
}
```
 - validation:
```
`access_token`

`subportfolio_id`:
    - classic string representation of uuid.
```
 - json, example:
```
Incoming:
'{"access_token": "q", "subportfolio_id": "550e8400-e29b-41d4-a716-446655440000", "subportfolio_trackable_wallet_wallet_id_registry_for_creating": [1], "subportfolio_trackable_wallet_wallet_id_registry_for_deleting": []}'
```
`Response`
 - data:
```
enum Precedent {
        Subportfolio_DoesNotExist,
        SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio,
    }
```
 - json, example:
```
Precedent:
'{"SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio"}'
```