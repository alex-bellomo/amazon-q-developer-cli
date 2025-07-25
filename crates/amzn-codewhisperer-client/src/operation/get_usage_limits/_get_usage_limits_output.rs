// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUsageLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub limits: ::std::vec::Vec<crate::types::UsageLimitList>,
    /// Number of days remaining until the usage metrics reset
    pub days_until_reset: i32,
    /// Usage breakdown by SKU type
    pub usage_breakdown: ::std::option::Option<crate::types::UsageBreakdown>,
    /// Subscription Info
    pub subscription_info: ::std::option::Option<crate::types::SubscriptionInfo>,
    /// Overage Configuration
    pub overage_configuration: ::std::option::Option<crate::types::OverageConfiguration>,
    _request_id: Option<String>,
}
impl GetUsageLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn limits(&self) -> &[crate::types::UsageLimitList] {
        use std::ops::Deref;
        self.limits.deref()
    }

    /// Number of days remaining until the usage metrics reset
    pub fn days_until_reset(&self) -> i32 {
        self.days_until_reset
    }

    /// Usage breakdown by SKU type
    pub fn usage_breakdown(&self) -> ::std::option::Option<&crate::types::UsageBreakdown> {
        self.usage_breakdown.as_ref()
    }

    /// Subscription Info
    pub fn subscription_info(&self) -> ::std::option::Option<&crate::types::SubscriptionInfo> {
        self.subscription_info.as_ref()
    }

    /// Overage Configuration
    pub fn overage_configuration(&self) -> ::std::option::Option<&crate::types::OverageConfiguration> {
        self.overage_configuration.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetUsageLimitsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetUsageLimitsOutput {
    /// Creates a new builder-style object to manufacture
    /// [`GetUsageLimitsOutput`](crate::operation::get_usage_limits::GetUsageLimitsOutput).
    pub fn builder() -> crate::operation::get_usage_limits::builders::GetUsageLimitsOutputBuilder {
        crate::operation::get_usage_limits::builders::GetUsageLimitsOutputBuilder::default()
    }
}

/// A builder for
/// [`GetUsageLimitsOutput`](crate::operation::get_usage_limits::GetUsageLimitsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetUsageLimitsOutputBuilder {
    pub(crate) limits: ::std::option::Option<::std::vec::Vec<crate::types::UsageLimitList>>,
    pub(crate) days_until_reset: ::std::option::Option<i32>,
    pub(crate) usage_breakdown: ::std::option::Option<crate::types::UsageBreakdown>,
    pub(crate) subscription_info: ::std::option::Option<crate::types::SubscriptionInfo>,
    pub(crate) overage_configuration: ::std::option::Option<crate::types::OverageConfiguration>,
    _request_id: Option<String>,
}
impl GetUsageLimitsOutputBuilder {
    /// Appends an item to `limits`.
    ///
    /// To override the contents of this collection use [`set_limits`](Self::set_limits).
    pub fn limits(mut self, input: crate::types::UsageLimitList) -> Self {
        let mut v = self.limits.unwrap_or_default();
        v.push(input);
        self.limits = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_limits(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UsageLimitList>>) -> Self {
        self.limits = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_limits(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UsageLimitList>> {
        &self.limits
    }

    /// Number of days remaining until the usage metrics reset
    /// This field is required.
    pub fn days_until_reset(mut self, input: i32) -> Self {
        self.days_until_reset = ::std::option::Option::Some(input);
        self
    }

    /// Number of days remaining until the usage metrics reset
    pub fn set_days_until_reset(mut self, input: ::std::option::Option<i32>) -> Self {
        self.days_until_reset = input;
        self
    }

    /// Number of days remaining until the usage metrics reset
    pub fn get_days_until_reset(&self) -> &::std::option::Option<i32> {
        &self.days_until_reset
    }

    /// Usage breakdown by SKU type
    pub fn usage_breakdown(mut self, input: crate::types::UsageBreakdown) -> Self {
        self.usage_breakdown = ::std::option::Option::Some(input);
        self
    }

    /// Usage breakdown by SKU type
    pub fn set_usage_breakdown(mut self, input: ::std::option::Option<crate::types::UsageBreakdown>) -> Self {
        self.usage_breakdown = input;
        self
    }

    /// Usage breakdown by SKU type
    pub fn get_usage_breakdown(&self) -> &::std::option::Option<crate::types::UsageBreakdown> {
        &self.usage_breakdown
    }

    /// Subscription Info
    pub fn subscription_info(mut self, input: crate::types::SubscriptionInfo) -> Self {
        self.subscription_info = ::std::option::Option::Some(input);
        self
    }

    /// Subscription Info
    pub fn set_subscription_info(mut self, input: ::std::option::Option<crate::types::SubscriptionInfo>) -> Self {
        self.subscription_info = input;
        self
    }

    /// Subscription Info
    pub fn get_subscription_info(&self) -> &::std::option::Option<crate::types::SubscriptionInfo> {
        &self.subscription_info
    }

    /// Overage Configuration
    pub fn overage_configuration(mut self, input: crate::types::OverageConfiguration) -> Self {
        self.overage_configuration = ::std::option::Option::Some(input);
        self
    }

    /// Overage Configuration
    pub fn set_overage_configuration(
        mut self,
        input: ::std::option::Option<crate::types::OverageConfiguration>,
    ) -> Self {
        self.overage_configuration = input;
        self
    }

    /// Overage Configuration
    pub fn get_overage_configuration(&self) -> &::std::option::Option<crate::types::OverageConfiguration> {
        &self.overage_configuration
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`GetUsageLimitsOutput`](crate::operation::get_usage_limits::GetUsageLimitsOutput). This
    /// method will fail if any of the following fields are not set:
    /// - [`limits`](crate::operation::get_usage_limits::builders::GetUsageLimitsOutputBuilder::limits)
    /// - [`days_until_reset`](crate::operation::get_usage_limits::builders::GetUsageLimitsOutputBuilder::days_until_reset)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_usage_limits::GetUsageLimitsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_usage_limits::GetUsageLimitsOutput {
            limits: self.limits.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "limits",
                    "limits was not specified but it is required when building GetUsageLimitsOutput",
                )
            })?,
            days_until_reset: self.days_until_reset.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "days_until_reset",
                    "days_until_reset was not specified but it is required when building GetUsageLimitsOutput",
                )
            })?,
            usage_breakdown: self.usage_breakdown,
            subscription_info: self.subscription_info,
            overage_configuration: self.overage_configuration,
            _request_id: self._request_id,
        })
    }
}
