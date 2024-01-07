# Backend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**abortonclose** | Option<**String**> |  | [optional]
**accept_invalid_http_response** | Option<**String**> |  | [optional]
**adv_check** | Option<**String**> |  | [optional]
**allbackups** | Option<**String**> |  | [optional]
**balance** | Option<[**crate::models::Balance**](balance.md)> |  | [optional]
**bind_process** | Option<**String**> |  | [optional]
**check_timeout** | Option<**i32**> |  | [optional]
**checkcache** | Option<**String**> |  | [optional]
**compression** | Option<[**crate::models::Compression**](compression.md)> |  | [optional]
**connect_timeout** | Option<**i32**> |  | [optional]
**cookie** | Option<[**crate::models::Cookie**](cookie.md)> |  | [optional]
**default_server** | Option<[**crate::models::DefaultServer**](default_server.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disabled** | Option<**bool**> |  | [optional]
**dynamic_cookie_key** | Option<**String**> |  | [optional]
**email_alert** | Option<[**crate::models::EmailAlert**](email_alert.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**error_files** | Option<[**Vec<crate::models::Errorfile>**](errorfile.md)> |  | [optional]
**errorfiles_from_http_errors** | Option<[**Vec<crate::models::Errorfiles>**](errorfiles.md)> |  | [optional]
**errorloc302** | Option<[**crate::models::Errorloc**](errorloc.md)> |  | [optional]
**errorloc303** | Option<[**crate::models::Errorloc**](errorloc.md)> |  | [optional]
**external_check** | Option<**String**> |  | [optional]
**external_check_command** | Option<**String**> |  | [optional]
**external_check_path** | Option<**String**> |  | [optional]
**force_persist** | Option<[**crate::models::BackendForcePersist**](backend_force_persist.md)> |  | [optional]
**forwardfor** | Option<[**crate::models::Forwardfor**](forwardfor.md)> |  | [optional]
**from** | Option<**String**> |  | [optional]
**fullconn** | Option<**i32**> |  | [optional]
**h1_case_adjust_bogus_server** | Option<**String**> |  | [optional]
**hash_type** | Option<[**crate::models::HashType**](hash_type.md)> |  | [optional]
**http_buffer_request** | Option<**String**> |  | [optional]
**http_check** | Option<[**crate::models::HttpCheck**](http_check.md)> |  | [optional]
**http_keep_alive** | Option<**String**> |  | [optional]
**http_no_delay** | Option<**String**> |  | [optional]
**http_server_close** | Option<**String**> |  | [optional]
**http_use_htx** | Option<**String**> |  | [optional]
**http_connection_mode** | Option<**String**> |  | [optional]
**http_keep_alive_timeout** | Option<**i32**> |  | [optional]
**http_pretend_keepalive** | Option<**String**> |  | [optional]
**http_proxy** | Option<**String**> |  | [optional]
**http_request_timeout** | Option<**i32**> |  | [optional]
**http_restrict_req_hdr_names** | Option<**String**> |  | [optional]
**http_reuse** | Option<**String**> |  | [optional]
**http_send_name_header** | Option<**String**> |  | [optional]
**httpchk_params** | Option<[**crate::models::HttpchkParams**](httpchk_params.md)> |  | [optional]
**httpclose** | Option<**String**> |  | [optional]
**id** | Option<**i32**> |  | [optional]
**ignore_persist** | Option<[**crate::models::BackendForcePersist**](backend_force_persist.md)> |  | [optional]
**independent_streams** | Option<**String**> |  | [optional]
**load_server_state_from_file** | Option<**String**> |  | [optional]
**log_health_checks** | Option<**String**> |  | [optional]
**log_tag** | Option<**String**> |  | [optional]
**max_keep_alive_queue** | Option<**i32**> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**mysql_check_params** | Option<[**crate::models::MysqlCheckParams**](mysql_check_params.md)> |  | [optional]
**name** | **String** |  | 
**nolinger** | Option<**String**> |  | [optional]
**originalto** | Option<[**crate::models::Originalto**](originalto.md)> |  | [optional]
**persist** | Option<**String**> |  | [optional]
**persist_rule** | Option<[**crate::models::PersistRule**](persist_rule.md)> |  | [optional]
**pgsql_check_params** | Option<[**crate::models::PgsqlCheckParams**](pgsql_check_params.md)> |  | [optional]
**prefer_last_server** | Option<**String**> |  | [optional]
**queue_timeout** | Option<**i32**> |  | [optional]
**redispatch** | Option<[**crate::models::Redispatch**](redispatch.md)> |  | [optional]
**retries** | Option<**i32**> |  | [optional]
**retry_on** | Option<**String**> |  | [optional]
**server_fin_timeout** | Option<**i32**> |  | [optional]
**server_state_file_name** | Option<**String**> |  | [optional]
**server_timeout** | Option<**i32**> |  | [optional]
**smtpchk_params** | Option<[**crate::models::SmtpchkParams**](smtpchk_params.md)> |  | [optional]
**source** | Option<[**crate::models::Source**](source.md)> |  | [optional]
**splice_auto** | Option<**String**> |  | [optional]
**splice_request** | Option<**String**> |  | [optional]
**splice_response** | Option<**String**> |  | [optional]
**spop_check** | Option<**String**> |  | [optional]
**srvtcpka** | Option<**String**> |  | [optional]
**srvtcpka_cnt** | Option<**i32**> |  | [optional]
**srvtcpka_idle** | Option<**i32**> |  | [optional]
**srvtcpka_intvl** | Option<**i32**> |  | [optional]
**stats_options** | Option<[**crate::models::StatsOptions**](stats_options.md)> |  | [optional]
**stick_table** | Option<[**crate::models::ConfigStickTable**](config_stick_table.md)> |  | [optional]
**tarpit_timeout** | Option<**i32**> |  | [optional]
**tcp_smart_connect** | Option<**String**> |  | [optional]
**tcpka** | Option<**String**> |  | [optional]
**transparent** | Option<**String**> |  | [optional]
**tunnel_timeout** | Option<**i32**> |  | [optional]
**use_fcgi_app** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

