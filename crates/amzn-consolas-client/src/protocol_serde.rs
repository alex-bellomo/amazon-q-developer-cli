// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<
        ::aws_smithy_runtime_api::client::interceptors::context::Error,
    >,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<
    ::aws_smithy_types::error::metadata::Builder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_allow_vended_log_delivery_for_resource;

pub(crate) mod shape_associate_customization_permission;

pub(crate) mod shape_create_customization;

pub(crate) mod shape_create_profile;

pub(crate) mod shape_delete_customization;

pub(crate) mod shape_delete_customization_permissions;

pub(crate) mod shape_delete_profile;

pub(crate) mod shape_disassociate_customization_permission;

pub(crate) mod shape_generate_recommendations;

pub(crate) mod shape_get_customization;

pub(crate) mod shape_list_customization_permissions;

pub(crate) mod shape_list_customization_versions;

pub(crate) mod shape_list_customizations;

pub(crate) mod shape_list_profiles;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_lock_service_linked_role;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_unlock_service_linked_role;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_customization;

pub(crate) mod shape_update_profile;

pub(crate) mod shape_vend_key_grant;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() { b"{}" } else { data }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_allow_vended_log_delivery_for_resource_input;

pub(crate) mod shape_associate_customization_permission_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_customization_input;

pub(crate) mod shape_create_profile_input;

pub(crate) mod shape_delete_customization_input;

pub(crate) mod shape_delete_customization_permissions_input;

pub(crate) mod shape_delete_profile_input;

pub(crate) mod shape_disassociate_customization_permission_input;

pub(crate) mod shape_generate_recommendations_input;

pub(crate) mod shape_get_customization_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_customization_permissions_input;

pub(crate) mod shape_list_customization_versions_input;

pub(crate) mod shape_list_customizations_input;

pub(crate) mod shape_list_profiles_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_lock_service_linked_role_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_linked_role_lock_client_exception;

pub(crate) mod shape_service_linked_role_lock_service_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_unlock_service_linked_role_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_customization_input;

pub(crate) mod shape_update_profile_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_vend_key_grant_input;

pub(crate) mod shape_customization_permission;

pub(crate) mod shape_customization_summary_list;

pub(crate) mod shape_customization_version_summary_list;

pub(crate) mod shape_data_reference;

pub(crate) mod shape_evaluation_metrics;

pub(crate) mod shape_file_context;

pub(crate) mod shape_identity_center_permissions;

pub(crate) mod shape_identity_source;

pub(crate) mod shape_opt_in_features;

pub(crate) mod shape_profile_list;

pub(crate) mod shape_recommendations_list;

pub(crate) mod shape_reference_tracker_configuration;

pub(crate) mod shape_repository_list;

pub(crate) mod shape_resource_policy;

pub(crate) mod shape_string_list_type;

pub(crate) mod shape_supplemental_context;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_by_user_analytics;

pub(crate) mod shape_code_star_reference;

pub(crate) mod shape_customization_summary;

pub(crate) mod shape_customization_version_summary;

pub(crate) mod shape_dashboard_analytics;

pub(crate) mod shape_external_identity_source;

pub(crate) mod shape_mcp_configuration;

pub(crate) mod shape_notifications_feature;

pub(crate) mod shape_overage_configuration;

pub(crate) mod shape_profile;

pub(crate) mod shape_programming_language;

pub(crate) mod shape_prompt_logging;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_s3_reference;

pub(crate) mod shape_sso_identity_source;

pub(crate) mod shape_supplemental_context_metadata;

pub(crate) mod shape_workspace_context;

pub(crate) mod shape_active_functionality_list;

pub(crate) mod shape_application_properties_list;

pub(crate) mod shape_identity_details;

pub(crate) mod shape_imports;

pub(crate) mod shape_previous_editor_state_metadata;

pub(crate) mod shape_references;

pub(crate) mod shape_application_properties;

pub(crate) mod shape_external_identity_details;

pub(crate) mod shape_import;

pub(crate) mod shape_notifications;

pub(crate) mod shape_reference;

pub(crate) mod shape_sso_identity_details;

pub(crate) mod shape_span;
