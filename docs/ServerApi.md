# \ServerApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_runtime_server**](ServerApi.md#add_runtime_server) | **POST** /services/haproxy/runtime/servers | Adds a new server to a backend
[**create_server**](ServerApi.md#create_server) | **POST** /services/haproxy/configuration/servers | Add a new server
[**delete_runtime_server**](ServerApi.md#delete_runtime_server) | **DELETE** /services/haproxy/runtime/servers/{name} | Deletes a server from a backend
[**delete_server**](ServerApi.md#delete_server) | **DELETE** /services/haproxy/configuration/servers/{name} | Delete a server
[**get_runtime_server**](ServerApi.md#get_runtime_server) | **GET** /services/haproxy/runtime/servers/{name} | Return one server runtime settings
[**get_runtime_servers**](ServerApi.md#get_runtime_servers) | **GET** /services/haproxy/runtime/servers | Return an array of runtime servers' settings
[**get_server**](ServerApi.md#get_server) | **GET** /services/haproxy/configuration/servers/{name} | Return one server
[**get_servers**](ServerApi.md#get_servers) | **GET** /services/haproxy/configuration/servers | Return an array of servers
[**replace_runtime_server**](ServerApi.md#replace_runtime_server) | **PUT** /services/haproxy/runtime/servers/{name} | Replace server transient settings
[**replace_server**](ServerApi.md#replace_server) | **PUT** /services/haproxy/configuration/servers/{name} | Replace a server



## add_runtime_server

> crate::models::RuntimeAddServer add_runtime_server(backend, runtime_add_server)
Adds a new server to a backend

Adds a new server to the specified backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |
**runtime_add_server** | [**RuntimeAddServer**](RuntimeAddServer.md) |  | [required] |

### Return type

[**crate::models::RuntimeAddServer**](runtime_add_server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server

> crate::models::Server create_server(server, backend, parent_name, parent_type, transaction_id, version, force_reload)
Add a new server

Adds a new server in the specified backend in the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server** | [**Server**](Server.md) |  | [required] |
**backend** | Option<**String**> | Parent backend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Server**](server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_runtime_server

> delete_runtime_server(name, backend)
Deletes a server from a backend

Deletes a server from the specified backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**backend** | **String** | Parent backend name | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> delete_server(name, backend, parent_name, parent_type, transaction_id, version, force_reload)
Delete a server

Deletes a server configuration by it's name in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**backend** | Option<**String**> | Parent backend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_server

> crate::models::RuntimeServer get_runtime_server(name, backend)
Return one server runtime settings

Returns one server runtime settings by it's name in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**backend** | **String** | Parent backend name | [required] |

### Return type

[**crate::models::RuntimeServer**](runtime_server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_servers

> Vec<crate::models::RuntimeServer> get_runtime_servers(backend)
Return an array of runtime servers' settings

Returns an array of all servers' runtime settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | **String** | Parent backend name | [required] |

### Return type

[**Vec<crate::models::RuntimeServer>**](runtime_server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server

> crate::models::GetServer200Response get_server(name, backend, parent_name, parent_type, transaction_id)
Return one server

Returns one server configuration by it's name in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**backend** | Option<**String**> | Parent backend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServer200Response**](getServer_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_servers

> crate::models::GetServers200Response get_servers(backend, parent_name, parent_type, transaction_id)
Return an array of servers

Returns an array of all servers that are configured in specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backend** | Option<**String**> | Parent backend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetServers200Response**](getServers_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_runtime_server

> crate::models::RuntimeServer replace_runtime_server(name, backend, runtime_server)
Replace server transient settings

Replaces a server transient settings by it's name in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**backend** | **String** | Parent backend name | [required] |
**runtime_server** | [**RuntimeServer**](RuntimeServer.md) |  | [required] |

### Return type

[**crate::models::RuntimeServer**](runtime_server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_server

> crate::models::Server replace_server(name, server, backend, parent_name, parent_type, transaction_id, version, force_reload)
Replace a server

Replaces a server configuration by it's name in the specified backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Server name | [required] |
**server** | [**Server**](Server.md) |  | [required] |
**backend** | Option<**String**> | Parent backend name |  |
**parent_name** | Option<**String**> | Parent name |  |
**parent_type** | Option<**String**> | Parent type |  |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |
**force_reload** | Option<**bool**> | If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration. |  |[default to false]

### Return type

[**crate::models::Server**](server.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

