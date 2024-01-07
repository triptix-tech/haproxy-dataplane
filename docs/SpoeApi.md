# \SpoeApi

All URIs are relative to *http://127.0.0.1/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_spoe**](SpoeApi.md#create_spoe) | **POST** /services/haproxy/spoe/spoe_files | Creates SPOE file with its entries
[**create_spoe_agent**](SpoeApi.md#create_spoe_agent) | **POST** /services/haproxy/spoe/spoe_agents | Add a new spoe agent to scope
[**create_spoe_group**](SpoeApi.md#create_spoe_group) | **POST** /services/haproxy/spoe/spoe_groups | Add a new SPOE groups
[**create_spoe_message**](SpoeApi.md#create_spoe_message) | **POST** /services/haproxy/spoe/spoe_messages | Add a new spoe message to scope
[**create_spoe_scope**](SpoeApi.md#create_spoe_scope) | **POST** /services/haproxy/spoe/spoe_scopes | Add a new spoe scope
[**delete_spoe_agent**](SpoeApi.md#delete_spoe_agent) | **DELETE** /services/haproxy/spoe/spoe_agents/{name} | Delete a SPOE agent
[**delete_spoe_file**](SpoeApi.md#delete_spoe_file) | **DELETE** /services/haproxy/spoe/spoe_files/{name} | Delete SPOE file
[**delete_spoe_group**](SpoeApi.md#delete_spoe_group) | **DELETE** /services/haproxy/spoe/spoe_groups/{name} | Delete a SPOE groups
[**delete_spoe_message**](SpoeApi.md#delete_spoe_message) | **DELETE** /services/haproxy/spoe/spoe_messages/{name} | Delete a spoe message
[**delete_spoe_scope**](SpoeApi.md#delete_spoe_scope) | **DELETE** /services/haproxy/spoe/spoe_scopes/{name} | Delete a SPOE scope
[**get_all_spoe_files**](SpoeApi.md#get_all_spoe_files) | **GET** /services/haproxy/spoe/spoe_files | Return all available SPOE files
[**get_one_spoe_file**](SpoeApi.md#get_one_spoe_file) | **GET** /services/haproxy/spoe/spoe_files/{name} | Return one SPOE file
[**get_spoe_agent**](SpoeApi.md#get_spoe_agent) | **GET** /services/haproxy/spoe/spoe_agents/{name} | Return a spoe agent
[**get_spoe_agents**](SpoeApi.md#get_spoe_agents) | **GET** /services/haproxy/spoe/spoe_agents | Return an array of spoe agents in one scope
[**get_spoe_configuration_version**](SpoeApi.md#get_spoe_configuration_version) | **GET** /services/haproxy/spoe/version | Return a SPOE configuration version
[**get_spoe_group**](SpoeApi.md#get_spoe_group) | **GET** /services/haproxy/spoe/spoe_groups/{name} | Return a SPOE groups
[**get_spoe_groups**](SpoeApi.md#get_spoe_groups) | **GET** /services/haproxy/spoe/spoe_groups | Return an array of SPOE groups
[**get_spoe_message**](SpoeApi.md#get_spoe_message) | **GET** /services/haproxy/spoe/spoe_messages/{name} | Return a spoe message
[**get_spoe_messages**](SpoeApi.md#get_spoe_messages) | **GET** /services/haproxy/spoe/spoe_messages | Return an array of spoe messages in one scope
[**get_spoe_scope**](SpoeApi.md#get_spoe_scope) | **GET** /services/haproxy/spoe/spoe_scopes/{name} | Return one SPOE scope
[**get_spoe_scopes**](SpoeApi.md#get_spoe_scopes) | **GET** /services/haproxy/spoe/spoe_scopes | Return an array of spoe scopes
[**replace_spoe_agent**](SpoeApi.md#replace_spoe_agent) | **PUT** /services/haproxy/spoe/spoe_agents/{name} | Replace a SPOE agent
[**replace_spoe_group**](SpoeApi.md#replace_spoe_group) | **PUT** /services/haproxy/spoe/spoe_groups/{name} | Replace a SPOE groups
[**replace_spoe_message**](SpoeApi.md#replace_spoe_message) | **PUT** /services/haproxy/spoe/spoe_messages/{name} | Replace a spoe message



## create_spoe

> String create_spoe(file_upload)
Creates SPOE file with its entries

Creates SPOE file with its entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_upload** | Option<**std::path::PathBuf**> | The spoe file to upload |  |

### Return type

**String**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_spoe_agent

> crate::models::SpoeAgent create_spoe_agent(spoe, scope, spoe_agent, transaction_id, version)
Add a new spoe agent to scope

Adds a new spoe agent to the spoe scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**spoe_agent** | [**SpoeAgent**](SpoeAgent.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeAgent**](spoe_agent.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_spoe_group

> crate::models::SpoeGroup create_spoe_group(spoe, scope, spoe_group, transaction_id, version)
Add a new SPOE groups

Adds a new SPOE groups to the SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**spoe_group** | [**SpoeGroup**](SpoeGroup.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeGroup**](spoe_group.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_spoe_message

> crate::models::SpoeMessage create_spoe_message(spoe, scope, spoe_message, transaction_id, version)
Add a new spoe message to scope

Adds a new spoe message to the spoe scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**spoe_message** | [**SpoeMessage**](SpoeMessage.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeMessage**](spoe_message.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_spoe_scope

> String create_spoe_scope(spoe, body, transaction_id, version)
Add a new spoe scope

Adds a new spoe scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**body** | **String** |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

**String**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_agent

> delete_spoe_agent(spoe, scope, name, transaction_id, version)
Delete a SPOE agent

Deletes a SPOE agent from the configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe agent name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_file

> delete_spoe_file(name)
Delete SPOE file

Deletes SPOE file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | SPOE file name | [required] |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_group

> delete_spoe_group(spoe, scope, name, transaction_id, version)
Delete a SPOE groups

Deletes a SPOE groups from the one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe group name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_message

> delete_spoe_message(spoe, scope, name, transaction_id, version)
Delete a spoe message

Deletes a spoe message from the SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe message name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spoe_scope

> delete_spoe_scope(spoe, name, transaction_id, version)
Delete a SPOE scope

Deletes a SPOE scope from the configuration file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**name** | **String** | Spoe scope name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

 (empty response body)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_spoe_files

> Vec<String> get_all_spoe_files()
Return all available SPOE files

Returns all available SPOE files.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_spoe_file

> crate::models::GetOneSpoeFile200Response get_one_spoe_file(name)
Return one SPOE file

Returns one SPOE file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | SPOE file name | [required] |

### Return type

[**crate::models::GetOneSpoeFile200Response**](getOneSpoeFile_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_agent

> crate::models::GetSpoeAgent200Response get_spoe_agent(spoe, scope, name, transaction_id)
Return a spoe agent

Returns one spoe agent configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe agent name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeAgent200Response**](getSpoeAgent_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_agents

> crate::models::GetSpoeAgents200Response get_spoe_agents(spoe, scope, transaction_id)
Return an array of spoe agents in one scope

Returns an array of all configured spoe agents in one scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeAgents200Response**](getSpoeAgents_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_configuration_version

> i32 get_spoe_configuration_version(spoe, transaction_id)
Return a SPOE configuration version

Returns SPOE configuration version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

**i32**

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_group

> crate::models::GetSpoeGroup200Response get_spoe_group(spoe, scope, name, transaction_id)
Return a SPOE groups

Returns one SPOE groups configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe group name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeGroup200Response**](getSpoeGroup_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_groups

> crate::models::GetSpoeGroups200Response get_spoe_groups(spoe, scope, transaction_id)
Return an array of SPOE groups

Returns an array of all configured SPOE groups in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeGroups200Response**](getSpoeGroups_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_message

> crate::models::GetSpoeMessage200Response get_spoe_message(spoe, scope, name, transaction_id)
Return a spoe message

Returns one spoe message configuration in SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe message name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeMessage200Response**](getSpoeMessage_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_messages

> crate::models::GetSpoeMessages200Response get_spoe_messages(spoe, scope, transaction_id)
Return an array of spoe messages in one scope

Returns an array of all configured spoe messages in one scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeMessages200Response**](getSpoeMessages_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_scope

> crate::models::GetSpoeScope200Response get_spoe_scope(spoe, name, transaction_id)
Return one SPOE scope

Returns one SPOE scope in one SPOE file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**name** | **String** | Spoe scope | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeScope200Response**](getSpoeScope_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spoe_scopes

> crate::models::GetSpoeScopes200Response get_spoe_scopes(spoe, transaction_id)
Return an array of spoe scopes

Returns an array of all configured spoe scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |

### Return type

[**crate::models::GetSpoeScopes200Response**](getSpoeScopes_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_spoe_agent

> crate::models::SpoeAgent replace_spoe_agent(spoe, scope, name, spoe_agent, transaction_id, version)
Replace a SPOE agent

Replaces a SPOE agent configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe agent name | [required] |
**spoe_agent** | [**SpoeAgent**](SpoeAgent.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeAgent**](spoe_agent.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_spoe_group

> crate::models::SpoeGroup replace_spoe_group(spoe, scope, name, spoe_group, transaction_id, version)
Replace a SPOE groups

Replaces a SPOE groups configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe group name | [required] |
**spoe_group** | [**SpoeGroup**](SpoeGroup.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeGroup**](spoe_group.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_spoe_message

> crate::models::SpoeMessage replace_spoe_message(spoe, scope, name, spoe_message, transaction_id, version)
Replace a spoe message

Replaces a spoe message configuration in one SPOE scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spoe** | **String** | Spoe file name | [required] |
**scope** | **String** | Spoe scope | [required] |
**name** | **String** | Spoe message name | [required] |
**spoe_message** | [**SpoeMessage**](SpoeMessage.md) |  | [required] |
**transaction_id** | Option<**String**> | ID of the transaction where we want to add the operation. Cannot be used when version is specified. |  |
**version** | Option<**i32**> | Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version. |  |

### Return type

[**crate::models::SpoeMessage**](spoe_message.md)

### Authorization

[basic_auth](../README.md#basic_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

