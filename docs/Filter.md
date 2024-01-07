# Filter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_name** | Option<**String**> | Name of the fcgi-app section this filter will use. | [optional]
**bandwidth_limit_name** | Option<**String**> | Filter name that will be used by 'set-bandwidth-limit' actions to reference a specific bandwidth limitation filter | [optional]
**cache_name** | Option<**String**> |  | [optional]
**default_limit** | Option<**i32**> | The max number of bytes that can be forwarded over the period. The value must be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes. | [optional]
**default_period** | Option<**i32**> | The default time period used to evaluate the bandwidth limitation rate. It can be specified for per-stream bandwidth limitation filters only. It follows the HAProxy time format and is expressed in milliseconds. | [optional]
**index** | Option<**i32**> |  | 
**key** | Option<**String**> | A sample expression rule. It describes what elements will be analyzed, extracted, combined, and used to select which table entry to update the counters. It must be specified for shared bandwidth limitation filters only. | [optional]
**limit** | Option<**i32**> | The max number of bytes that can be forwarded over the period. The value must be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes. | [optional]
**min_size** | Option<**i32**> | The optional minimum number of bytes forwarded at a time by a stream excluding the last packet that may be smaller. This value can be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes. | [optional]
**spoe_config** | Option<**String**> |  | [optional]
**spoe_engine** | Option<**String**> |  | [optional]
**table** | Option<**String**> | An optional table to be used instead of the default one, which is the stick-table declared in the current proxy. It can be specified for shared bandwidth limitation filters only. | [optional]
**trace_hexdump** | Option<**bool**> |  | [optional]
**trace_name** | Option<**String**> |  | [optional]
**trace_rnd_forwarding** | Option<**bool**> |  | [optional]
**trace_rnd_parsing** | Option<**bool**> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


