// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_subscriptions_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_available_subscriptions::ListAvailableSubscriptionsOutput,
    crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder =
        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
            .map_err(crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled(generic),
            );
        },
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::AccessDeniedError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                        )?
                };
                tmp
            })
        },
        "InternalServerException" => {
            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    output =
                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(
                            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                        )?
                };
                tmp
            })
        },
        "ThrottlingException" => {
            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::ThrottlingError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::throttling_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                        )?
                };
                tmp
            })
        },
        "ValidationException" => {
            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::ValidationError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled,
                        )?
                };
                tmp
            })
        },
        _ => crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_subscriptions_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_available_subscriptions::ListAvailableSubscriptionsOutput,
    crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsOutputBuilder::default(
            );
        output = crate::protocol_serde::shape_list_available_subscriptions::de_list_available_subscriptions(
            _response_body,
            output,
        )
        .map_err(crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_available_subscriptions_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError::unhandled)?
    })
}

pub fn ser_list_available_subscriptions_input(
    _input: &crate::operation::list_available_subscriptions::ListAvailableSubscriptionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError>
{
    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
}

pub(crate) fn de_list_available_subscriptions(
    value: &[u8],
    mut builder: crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "subscriptionPlans" => {
                    builder = builder.set_subscription_plans(
                        crate::protocol_serde::shape_subscription_plan_list::de_subscription_plan_list(tokens)?,
                    );
                },
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    format!("expected object key or end object, found: {:?}", other),
                ));
            },
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
