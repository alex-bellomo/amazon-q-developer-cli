// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_usage_limits_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_usage_limits::GetUsageLimitsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.profile_arn {
        object.key("profileArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_type {
        object.key("resourceType").string(var_2.as_str());
    }
    Ok(())
}
