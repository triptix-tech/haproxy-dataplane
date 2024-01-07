use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    acl_api: Box<dyn crate::apis::AclApi>,
    acl_runtime_api: Box<dyn crate::apis::AclRuntimeApi>,
    backend_api: Box<dyn crate::apis::BackendApi>,
    backend_switching_rule_api: Box<dyn crate::apis::BackendSwitchingRuleApi>,
    bind_api: Box<dyn crate::apis::BindApi>,
    cache_api: Box<dyn crate::apis::CacheApi>,
    cluster_api: Box<dyn crate::apis::ClusterApi>,
    configuration_api: Box<dyn crate::apis::ConfigurationApi>,
    declare_capture_api: Box<dyn crate::apis::DeclareCaptureApi>,
    defaults_api: Box<dyn crate::apis::DefaultsApi>,
    dgram_bind_api: Box<dyn crate::apis::DgramBindApi>,
    discovery_api: Box<dyn crate::apis::DiscoveryApi>,
    fcgi_app_api: Box<dyn crate::apis::FcgiAppApi>,
    filter_api: Box<dyn crate::apis::FilterApi>,
    frontend_api: Box<dyn crate::apis::FrontendApi>,
    global_api: Box<dyn crate::apis::GlobalApi>,
    group_api: Box<dyn crate::apis::GroupApi>,
    http_after_response_rule_api: Box<dyn crate::apis::HttpAfterResponseRuleApi>,
    http_check_api: Box<dyn crate::apis::HttpCheckApi>,
    http_error_rule_api: Box<dyn crate::apis::HttpErrorRuleApi>,
    http_errors_api: Box<dyn crate::apis::HttpErrorsApi>,
    http_request_rule_api: Box<dyn crate::apis::HttpRequestRuleApi>,
    http_response_rule_api: Box<dyn crate::apis::HttpResponseRuleApi>,
    health_api: Box<dyn crate::apis::HealthApi>,
    information_api: Box<dyn crate::apis::InformationApi>,
    log_forward_api: Box<dyn crate::apis::LogForwardApi>,
    log_target_api: Box<dyn crate::apis::LogTargetApi>,
    mailer_entry_api: Box<dyn crate::apis::MailerEntryApi>,
    mailers_api: Box<dyn crate::apis::MailersApi>,
    maps_api: Box<dyn crate::apis::MapsApi>,
    nameserver_api: Box<dyn crate::apis::NameserverApi>,
    peer_api: Box<dyn crate::apis::PeerApi>,
    peer_entry_api: Box<dyn crate::apis::PeerEntryApi>,
    process_manager_api: Box<dyn crate::apis::ProcessManagerApi>,
    reloads_api: Box<dyn crate::apis::ReloadsApi>,
    resolver_api: Box<dyn crate::apis::ResolverApi>,
    ring_api: Box<dyn crate::apis::RingApi>,
    server_api: Box<dyn crate::apis::ServerApi>,
    server_switching_rule_api: Box<dyn crate::apis::ServerSwitchingRuleApi>,
    server_template_api: Box<dyn crate::apis::ServerTemplateApi>,
    service_discovery_api: Box<dyn crate::apis::ServiceDiscoveryApi>,
    sites_api: Box<dyn crate::apis::SitesApi>,
    specification_api: Box<dyn crate::apis::SpecificationApi>,
    specification_openapiv3_api: Box<dyn crate::apis::SpecificationOpenapiv3Api>,
    spoe_api: Box<dyn crate::apis::SpoeApi>,
    spoe_transactions_api: Box<dyn crate::apis::SpoeTransactionsApi>,
    stats_api: Box<dyn crate::apis::StatsApi>,
    stick_rule_api: Box<dyn crate::apis::StickRuleApi>,
    stick_table_api: Box<dyn crate::apis::StickTableApi>,
    storage_api: Box<dyn crate::apis::StorageApi>,
    tcp_check_api: Box<dyn crate::apis::TcpCheckApi>,
    tcp_request_rule_api: Box<dyn crate::apis::TcpRequestRuleApi>,
    tcp_response_rule_api: Box<dyn crate::apis::TcpResponseRuleApi>,
    table_api: Box<dyn crate::apis::TableApi>,
    transactions_api: Box<dyn crate::apis::TransactionsApi>,
    user_api: Box<dyn crate::apis::UserApi>,
    userlist_api: Box<dyn crate::apis::UserlistApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            acl_api: Box::new(crate::apis::AclApiClient::new(rc.clone())),
            acl_runtime_api: Box::new(crate::apis::AclRuntimeApiClient::new(rc.clone())),
            backend_api: Box::new(crate::apis::BackendApiClient::new(rc.clone())),
            backend_switching_rule_api: Box::new(crate::apis::BackendSwitchingRuleApiClient::new(rc.clone())),
            bind_api: Box::new(crate::apis::BindApiClient::new(rc.clone())),
            cache_api: Box::new(crate::apis::CacheApiClient::new(rc.clone())),
            cluster_api: Box::new(crate::apis::ClusterApiClient::new(rc.clone())),
            configuration_api: Box::new(crate::apis::ConfigurationApiClient::new(rc.clone())),
            declare_capture_api: Box::new(crate::apis::DeclareCaptureApiClient::new(rc.clone())),
            defaults_api: Box::new(crate::apis::DefaultsApiClient::new(rc.clone())),
            dgram_bind_api: Box::new(crate::apis::DgramBindApiClient::new(rc.clone())),
            discovery_api: Box::new(crate::apis::DiscoveryApiClient::new(rc.clone())),
            fcgi_app_api: Box::new(crate::apis::FcgiAppApiClient::new(rc.clone())),
            filter_api: Box::new(crate::apis::FilterApiClient::new(rc.clone())),
            frontend_api: Box::new(crate::apis::FrontendApiClient::new(rc.clone())),
            global_api: Box::new(crate::apis::GlobalApiClient::new(rc.clone())),
            group_api: Box::new(crate::apis::GroupApiClient::new(rc.clone())),
            http_after_response_rule_api: Box::new(crate::apis::HttpAfterResponseRuleApiClient::new(rc.clone())),
            http_check_api: Box::new(crate::apis::HttpCheckApiClient::new(rc.clone())),
            http_error_rule_api: Box::new(crate::apis::HttpErrorRuleApiClient::new(rc.clone())),
            http_errors_api: Box::new(crate::apis::HttpErrorsApiClient::new(rc.clone())),
            http_request_rule_api: Box::new(crate::apis::HttpRequestRuleApiClient::new(rc.clone())),
            http_response_rule_api: Box::new(crate::apis::HttpResponseRuleApiClient::new(rc.clone())),
            health_api: Box::new(crate::apis::HealthApiClient::new(rc.clone())),
            information_api: Box::new(crate::apis::InformationApiClient::new(rc.clone())),
            log_forward_api: Box::new(crate::apis::LogForwardApiClient::new(rc.clone())),
            log_target_api: Box::new(crate::apis::LogTargetApiClient::new(rc.clone())),
            mailer_entry_api: Box::new(crate::apis::MailerEntryApiClient::new(rc.clone())),
            mailers_api: Box::new(crate::apis::MailersApiClient::new(rc.clone())),
            maps_api: Box::new(crate::apis::MapsApiClient::new(rc.clone())),
            nameserver_api: Box::new(crate::apis::NameserverApiClient::new(rc.clone())),
            peer_api: Box::new(crate::apis::PeerApiClient::new(rc.clone())),
            peer_entry_api: Box::new(crate::apis::PeerEntryApiClient::new(rc.clone())),
            process_manager_api: Box::new(crate::apis::ProcessManagerApiClient::new(rc.clone())),
            reloads_api: Box::new(crate::apis::ReloadsApiClient::new(rc.clone())),
            resolver_api: Box::new(crate::apis::ResolverApiClient::new(rc.clone())),
            ring_api: Box::new(crate::apis::RingApiClient::new(rc.clone())),
            server_api: Box::new(crate::apis::ServerApiClient::new(rc.clone())),
            server_switching_rule_api: Box::new(crate::apis::ServerSwitchingRuleApiClient::new(rc.clone())),
            server_template_api: Box::new(crate::apis::ServerTemplateApiClient::new(rc.clone())),
            service_discovery_api: Box::new(crate::apis::ServiceDiscoveryApiClient::new(rc.clone())),
            sites_api: Box::new(crate::apis::SitesApiClient::new(rc.clone())),
            specification_api: Box::new(crate::apis::SpecificationApiClient::new(rc.clone())),
            specification_openapiv3_api: Box::new(crate::apis::SpecificationOpenapiv3ApiClient::new(rc.clone())),
            spoe_api: Box::new(crate::apis::SpoeApiClient::new(rc.clone())),
            spoe_transactions_api: Box::new(crate::apis::SpoeTransactionsApiClient::new(rc.clone())),
            stats_api: Box::new(crate::apis::StatsApiClient::new(rc.clone())),
            stick_rule_api: Box::new(crate::apis::StickRuleApiClient::new(rc.clone())),
            stick_table_api: Box::new(crate::apis::StickTableApiClient::new(rc.clone())),
            storage_api: Box::new(crate::apis::StorageApiClient::new(rc.clone())),
            tcp_check_api: Box::new(crate::apis::TcpCheckApiClient::new(rc.clone())),
            tcp_request_rule_api: Box::new(crate::apis::TcpRequestRuleApiClient::new(rc.clone())),
            tcp_response_rule_api: Box::new(crate::apis::TcpResponseRuleApiClient::new(rc.clone())),
            table_api: Box::new(crate::apis::TableApiClient::new(rc.clone())),
            transactions_api: Box::new(crate::apis::TransactionsApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
            userlist_api: Box::new(crate::apis::UserlistApiClient::new(rc.clone())),
        }
    }

    pub fn acl_api(&self) -> &dyn crate::apis::AclApi{
        self.acl_api.as_ref()
    }

    pub fn acl_runtime_api(&self) -> &dyn crate::apis::AclRuntimeApi{
        self.acl_runtime_api.as_ref()
    }

    pub fn backend_api(&self) -> &dyn crate::apis::BackendApi{
        self.backend_api.as_ref()
    }

    pub fn backend_switching_rule_api(&self) -> &dyn crate::apis::BackendSwitchingRuleApi{
        self.backend_switching_rule_api.as_ref()
    }

    pub fn bind_api(&self) -> &dyn crate::apis::BindApi{
        self.bind_api.as_ref()
    }

    pub fn cache_api(&self) -> &dyn crate::apis::CacheApi{
        self.cache_api.as_ref()
    }

    pub fn cluster_api(&self) -> &dyn crate::apis::ClusterApi{
        self.cluster_api.as_ref()
    }

    pub fn configuration_api(&self) -> &dyn crate::apis::ConfigurationApi{
        self.configuration_api.as_ref()
    }

    pub fn declare_capture_api(&self) -> &dyn crate::apis::DeclareCaptureApi{
        self.declare_capture_api.as_ref()
    }

    pub fn defaults_api(&self) -> &dyn crate::apis::DefaultsApi{
        self.defaults_api.as_ref()
    }

    pub fn dgram_bind_api(&self) -> &dyn crate::apis::DgramBindApi{
        self.dgram_bind_api.as_ref()
    }

    pub fn discovery_api(&self) -> &dyn crate::apis::DiscoveryApi{
        self.discovery_api.as_ref()
    }

    pub fn fcgi_app_api(&self) -> &dyn crate::apis::FcgiAppApi{
        self.fcgi_app_api.as_ref()
    }

    pub fn filter_api(&self) -> &dyn crate::apis::FilterApi{
        self.filter_api.as_ref()
    }

    pub fn frontend_api(&self) -> &dyn crate::apis::FrontendApi{
        self.frontend_api.as_ref()
    }

    pub fn global_api(&self) -> &dyn crate::apis::GlobalApi{
        self.global_api.as_ref()
    }

    pub fn group_api(&self) -> &dyn crate::apis::GroupApi{
        self.group_api.as_ref()
    }

    pub fn http_after_response_rule_api(&self) -> &dyn crate::apis::HttpAfterResponseRuleApi{
        self.http_after_response_rule_api.as_ref()
    }

    pub fn http_check_api(&self) -> &dyn crate::apis::HttpCheckApi{
        self.http_check_api.as_ref()
    }

    pub fn http_error_rule_api(&self) -> &dyn crate::apis::HttpErrorRuleApi{
        self.http_error_rule_api.as_ref()
    }

    pub fn http_errors_api(&self) -> &dyn crate::apis::HttpErrorsApi{
        self.http_errors_api.as_ref()
    }

    pub fn http_request_rule_api(&self) -> &dyn crate::apis::HttpRequestRuleApi{
        self.http_request_rule_api.as_ref()
    }

    pub fn http_response_rule_api(&self) -> &dyn crate::apis::HttpResponseRuleApi{
        self.http_response_rule_api.as_ref()
    }

    pub fn health_api(&self) -> &dyn crate::apis::HealthApi{
        self.health_api.as_ref()
    }

    pub fn information_api(&self) -> &dyn crate::apis::InformationApi{
        self.information_api.as_ref()
    }

    pub fn log_forward_api(&self) -> &dyn crate::apis::LogForwardApi{
        self.log_forward_api.as_ref()
    }

    pub fn log_target_api(&self) -> &dyn crate::apis::LogTargetApi{
        self.log_target_api.as_ref()
    }

    pub fn mailer_entry_api(&self) -> &dyn crate::apis::MailerEntryApi{
        self.mailer_entry_api.as_ref()
    }

    pub fn mailers_api(&self) -> &dyn crate::apis::MailersApi{
        self.mailers_api.as_ref()
    }

    pub fn maps_api(&self) -> &dyn crate::apis::MapsApi{
        self.maps_api.as_ref()
    }

    pub fn nameserver_api(&self) -> &dyn crate::apis::NameserverApi{
        self.nameserver_api.as_ref()
    }

    pub fn peer_api(&self) -> &dyn crate::apis::PeerApi{
        self.peer_api.as_ref()
    }

    pub fn peer_entry_api(&self) -> &dyn crate::apis::PeerEntryApi{
        self.peer_entry_api.as_ref()
    }

    pub fn process_manager_api(&self) -> &dyn crate::apis::ProcessManagerApi{
        self.process_manager_api.as_ref()
    }

    pub fn reloads_api(&self) -> &dyn crate::apis::ReloadsApi{
        self.reloads_api.as_ref()
    }

    pub fn resolver_api(&self) -> &dyn crate::apis::ResolverApi{
        self.resolver_api.as_ref()
    }

    pub fn ring_api(&self) -> &dyn crate::apis::RingApi{
        self.ring_api.as_ref()
    }

    pub fn server_api(&self) -> &dyn crate::apis::ServerApi{
        self.server_api.as_ref()
    }

    pub fn server_switching_rule_api(&self) -> &dyn crate::apis::ServerSwitchingRuleApi{
        self.server_switching_rule_api.as_ref()
    }

    pub fn server_template_api(&self) -> &dyn crate::apis::ServerTemplateApi{
        self.server_template_api.as_ref()
    }

    pub fn service_discovery_api(&self) -> &dyn crate::apis::ServiceDiscoveryApi{
        self.service_discovery_api.as_ref()
    }

    pub fn sites_api(&self) -> &dyn crate::apis::SitesApi{
        self.sites_api.as_ref()
    }

    pub fn specification_api(&self) -> &dyn crate::apis::SpecificationApi{
        self.specification_api.as_ref()
    }

    pub fn specification_openapiv3_api(&self) -> &dyn crate::apis::SpecificationOpenapiv3Api{
        self.specification_openapiv3_api.as_ref()
    }

    pub fn spoe_api(&self) -> &dyn crate::apis::SpoeApi{
        self.spoe_api.as_ref()
    }

    pub fn spoe_transactions_api(&self) -> &dyn crate::apis::SpoeTransactionsApi{
        self.spoe_transactions_api.as_ref()
    }

    pub fn stats_api(&self) -> &dyn crate::apis::StatsApi{
        self.stats_api.as_ref()
    }

    pub fn stick_rule_api(&self) -> &dyn crate::apis::StickRuleApi{
        self.stick_rule_api.as_ref()
    }

    pub fn stick_table_api(&self) -> &dyn crate::apis::StickTableApi{
        self.stick_table_api.as_ref()
    }

    pub fn storage_api(&self) -> &dyn crate::apis::StorageApi{
        self.storage_api.as_ref()
    }

    pub fn tcp_check_api(&self) -> &dyn crate::apis::TcpCheckApi{
        self.tcp_check_api.as_ref()
    }

    pub fn tcp_request_rule_api(&self) -> &dyn crate::apis::TcpRequestRuleApi{
        self.tcp_request_rule_api.as_ref()
    }

    pub fn tcp_response_rule_api(&self) -> &dyn crate::apis::TcpResponseRuleApi{
        self.tcp_response_rule_api.as_ref()
    }

    pub fn table_api(&self) -> &dyn crate::apis::TableApi{
        self.table_api.as_ref()
    }

    pub fn transactions_api(&self) -> &dyn crate::apis::TransactionsApi{
        self.transactions_api.as_ref()
    }

    pub fn user_api(&self) -> &dyn crate::apis::UserApi{
        self.user_api.as_ref()
    }

    pub fn userlist_api(&self) -> &dyn crate::apis::UserlistApi{
        self.userlist_api.as_ref()
    }

}
