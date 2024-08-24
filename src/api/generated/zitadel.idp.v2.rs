// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Idp {
    /// Unique identifier for the identity provider.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::object::v2::Details>,
    /// Current state of the identity provider.
    #[prost(enumeration="IdpState", tag="3")]
    pub state: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Type of the identity provider, for example OIDC, JWT, LDAP and SAML.
    #[prost(enumeration="IdpType", tag="5")]
    pub r#type: i32,
    /// Configuration for the type of the identity provider.
    #[prost(message, optional, tag="6")]
    pub config: ::core::option::Option<IdpConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdpConfig {
    #[prost(message, optional, tag="1")]
    pub options: ::core::option::Option<Options>,
    #[prost(oneof="idp_config::Config", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub config: ::core::option::Option<idp_config::Config>,
}
/// Nested message and enum types in `IDPConfig`.
pub mod idp_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="2")]
        Ldap(super::LdapConfig),
        #[prost(message, tag="3")]
        Google(super::GoogleConfig),
        #[prost(message, tag="4")]
        Oauth(super::OAuthConfig),
        #[prost(message, tag="5")]
        Oidc(super::GenericOidcConfig),
        #[prost(message, tag="6")]
        Jwt(super::JwtConfig),
        #[prost(message, tag="7")]
        Github(super::GitHubConfig),
        #[prost(message, tag="8")]
        GithubEs(super::GitHubEnterpriseServerConfig),
        #[prost(message, tag="9")]
        Gitlab(super::GitLabConfig),
        #[prost(message, tag="10")]
        GitlabSelfHosted(super::GitLabSelfHostedConfig),
        #[prost(message, tag="11")]
        AzureAd(super::AzureAdConfig),
        #[prost(message, tag="12")]
        Apple(super::AppleConfig),
        #[prost(message, tag="13")]
        Saml(super::SamlConfig),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtConfig {
    /// The endpoint where the JWT can be extracted.
    #[prost(string, tag="1")]
    pub jwt_endpoint: ::prost::alloc::string::String,
    /// The issuer of the JWT (for validation).
    #[prost(string, tag="2")]
    pub issuer: ::prost::alloc::string::String,
    /// The endpoint to the key (JWK) which is used to sign the JWT with.
    #[prost(string, tag="3")]
    pub keys_endpoint: ::prost::alloc::string::String,
    /// The name of the header where the JWT is sent in, default is authorization.
    #[prost(string, tag="4")]
    pub header_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthConfig {
    /// Client id generated by the identity provider.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// The endpoint where ZITADEL send the user to authenticate.
    #[prost(string, tag="2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    /// The endpoint where ZITADEL can get the token.
    #[prost(string, tag="3")]
    pub token_endpoint: ::prost::alloc::string::String,
    /// The endpoint where ZITADEL can get the user information.
    #[prost(string, tag="4")]
    pub user_endpoint: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request on the identity provider.
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Defines how the attribute is called where ZITADEL can get the id of the user.
    #[prost(string, tag="6")]
    pub id_attribute: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericOidcConfig {
    /// The OIDC issuer of the identity provider.
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// Client id generated by the identity provider.
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request on the identity provider.
    #[prost(string, repeated, tag="3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, provider information get mapped from the id token, not from the userinfo endpoint.
    #[prost(bool, tag="4")]
    pub is_id_token_mapping: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubConfig {
    /// The client ID of the GitHub App.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to GitHub.
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubEnterpriseServerConfig {
    /// The client ID of the GitHub App.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_endpoint: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to GitHub.
    #[prost(string, repeated, tag="5")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleConfig {
    /// Client id of the Google application.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to Google.
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitLabConfig {
    /// Client id of the GitLab application.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to GitLab.
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitLabSelfHostedConfig {
    #[prost(string, tag="1")]
    pub issuer: ::prost::alloc::string::String,
    /// Client id of the GitLab application.
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to GitLab.
    #[prost(string, repeated, tag="3")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapConfig {
    #[prost(string, repeated, tag="1")]
    pub servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="2")]
    pub start_tls: bool,
    #[prost(string, tag="3")]
    pub base_dn: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bind_dn: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user_base: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub user_object_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub user_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    #[prost(message, optional, tag="9")]
    pub attributes: ::core::option::Option<LdapAttributes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlConfig {
    /// Metadata of the SAML identity provider.
    #[prost(bytes="vec", tag="1")]
    pub metadata_xml: ::prost::alloc::vec::Vec<u8>,
    /// Binding which defines the type of communication with the identity provider.
    #[prost(enumeration="SamlBinding", tag="2")]
    pub binding: i32,
    /// Boolean which defines if the authentication requests are signed.
    #[prost(bool, tag="3")]
    pub with_signed_request: bool,
    /// `nameid-format` for the SAML Request.
    #[prost(enumeration="SamlNameIdFormat", tag="4")]
    pub name_id_format: i32,
    /// Optional name of the attribute, which will be used to map the user
    /// in case the nameid-format returned is `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`.
    #[prost(string, optional, tag="5")]
    pub transient_mapping_attribute_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureAdConfig {
    /// Client id of the Azure AD application
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// Defines what user accounts should be able to login (Personal, Organizational, All).
    #[prost(message, optional, tag="2")]
    pub tenant: ::core::option::Option<AzureAdTenant>,
    /// Azure AD doesn't send if the email has been verified. Enable this if the user email should always be added verified in ZITADEL (no verification emails will be sent).
    #[prost(bool, tag="3")]
    pub email_verified: bool,
    /// The scopes requested by ZITADEL during the request to Azure AD.
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    /// Enable if users should be able to link an existing ZITADEL user with an external account.
    #[prost(bool, tag="1")]
    pub is_linking_allowed: bool,
    /// Enable if users should be able to create a new account in ZITADEL when using an external account.
    #[prost(bool, tag="2")]
    pub is_creation_allowed: bool,
    /// Enable if a new account in ZITADEL should be created automatically when login with an external account.
    #[prost(bool, tag="3")]
    pub is_auto_creation: bool,
    /// Enable if a the ZITADEL account fields should be updated automatically on each login.
    #[prost(bool, tag="4")]
    pub is_auto_update: bool,
    /// Enable if users should get prompted to link an existing ZITADEL user to an external account if the selected attribute matches.
    #[prost(enumeration="AutoLinkingOption", tag="5")]
    pub auto_linking: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LdapAttributes {
    #[prost(string, tag="1")]
    pub id_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub first_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub display_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub nick_name_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub preferred_username_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub email_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub email_verified_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub phone_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub phone_verified_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub preferred_language_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub avatar_url_attribute: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub profile_attribute: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureAdTenant {
    #[prost(oneof="azure_ad_tenant::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<azure_ad_tenant::Type>,
}
/// Nested message and enum types in `AzureADTenant`.
pub mod azure_ad_tenant {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration="super::AzureAdTenantType", tag="1")]
        TenantType(i32),
        #[prost(string, tag="2")]
        TenantId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppleConfig {
    /// Client id (App ID or Service ID) provided by Apple.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// Team ID provided by Apple.
    #[prost(string, tag="2")]
    pub team_id: ::prost::alloc::string::String,
    /// ID of the private key generated by Apple.
    #[prost(string, tag="3")]
    pub key_id: ::prost::alloc::string::String,
    /// The scopes requested by ZITADEL during the request to Apple.
    #[prost(string, repeated, tag="4")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpState {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Removed = 3,
    Migrated = 4,
}
impl IdpState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpState::Unspecified => "IDP_STATE_UNSPECIFIED",
            IdpState::Active => "IDP_STATE_ACTIVE",
            IdpState::Inactive => "IDP_STATE_INACTIVE",
            IdpState::Removed => "IDP_STATE_REMOVED",
            IdpState::Migrated => "IDP_STATE_MIGRATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_STATE_ACTIVE" => Some(Self::Active),
            "IDP_STATE_INACTIVE" => Some(Self::Inactive),
            "IDP_STATE_REMOVED" => Some(Self::Removed),
            "IDP_STATE_MIGRATED" => Some(Self::Migrated),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdpType {
    Unspecified = 0,
    Oidc = 1,
    Jwt = 2,
    Ldap = 3,
    Oauth = 4,
    AzureAd = 5,
    Github = 6,
    GithubEs = 7,
    Gitlab = 8,
    GitlabSelfHosted = 9,
    Google = 10,
    Apple = 11,
    Saml = 12,
}
impl IdpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdpType::Unspecified => "IDP_TYPE_UNSPECIFIED",
            IdpType::Oidc => "IDP_TYPE_OIDC",
            IdpType::Jwt => "IDP_TYPE_JWT",
            IdpType::Ldap => "IDP_TYPE_LDAP",
            IdpType::Oauth => "IDP_TYPE_OAUTH",
            IdpType::AzureAd => "IDP_TYPE_AZURE_AD",
            IdpType::Github => "IDP_TYPE_GITHUB",
            IdpType::GithubEs => "IDP_TYPE_GITHUB_ES",
            IdpType::Gitlab => "IDP_TYPE_GITLAB",
            IdpType::GitlabSelfHosted => "IDP_TYPE_GITLAB_SELF_HOSTED",
            IdpType::Google => "IDP_TYPE_GOOGLE",
            IdpType::Apple => "IDP_TYPE_APPLE",
            IdpType::Saml => "IDP_TYPE_SAML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "IDP_TYPE_OIDC" => Some(Self::Oidc),
            "IDP_TYPE_JWT" => Some(Self::Jwt),
            "IDP_TYPE_LDAP" => Some(Self::Ldap),
            "IDP_TYPE_OAUTH" => Some(Self::Oauth),
            "IDP_TYPE_AZURE_AD" => Some(Self::AzureAd),
            "IDP_TYPE_GITHUB" => Some(Self::Github),
            "IDP_TYPE_GITHUB_ES" => Some(Self::GithubEs),
            "IDP_TYPE_GITLAB" => Some(Self::Gitlab),
            "IDP_TYPE_GITLAB_SELF_HOSTED" => Some(Self::GitlabSelfHosted),
            "IDP_TYPE_GOOGLE" => Some(Self::Google),
            "IDP_TYPE_APPLE" => Some(Self::Apple),
            "IDP_TYPE_SAML" => Some(Self::Saml),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SamlBinding {
    Unspecified = 0,
    Post = 1,
    Redirect = 2,
    Artifact = 3,
}
impl SamlBinding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SamlBinding::Unspecified => "SAML_BINDING_UNSPECIFIED",
            SamlBinding::Post => "SAML_BINDING_POST",
            SamlBinding::Redirect => "SAML_BINDING_REDIRECT",
            SamlBinding::Artifact => "SAML_BINDING_ARTIFACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAML_BINDING_UNSPECIFIED" => Some(Self::Unspecified),
            "SAML_BINDING_POST" => Some(Self::Post),
            "SAML_BINDING_REDIRECT" => Some(Self::Redirect),
            "SAML_BINDING_ARTIFACT" => Some(Self::Artifact),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SamlNameIdFormat {
    Unspecified = 0,
    EmailAddress = 1,
    Persistent = 2,
    Transient = 3,
}
impl SamlNameIdFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SamlNameIdFormat::Unspecified => "SAML_NAME_ID_FORMAT_UNSPECIFIED",
            SamlNameIdFormat::EmailAddress => "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS",
            SamlNameIdFormat::Persistent => "SAML_NAME_ID_FORMAT_PERSISTENT",
            SamlNameIdFormat::Transient => "SAML_NAME_ID_FORMAT_TRANSIENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAML_NAME_ID_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS" => Some(Self::EmailAddress),
            "SAML_NAME_ID_FORMAT_PERSISTENT" => Some(Self::Persistent),
            "SAML_NAME_ID_FORMAT_TRANSIENT" => Some(Self::Transient),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoLinkingOption {
    /// AUTO_LINKING_OPTION_UNSPECIFIED disables the auto linking prompt.
    Unspecified = 0,
    /// AUTO_LINKING_OPTION_USERNAME will use the username of the external user to check for a corresponding ZITADEL user.
    Username = 1,
    /// AUTO_LINKING_OPTION_EMAIL  will use the email of the external user to check for a corresponding ZITADEL user with the same verified email
    /// Note that in case multiple users match, no prompt will be shown.
    Email = 2,
}
impl AutoLinkingOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutoLinkingOption::Unspecified => "AUTO_LINKING_OPTION_UNSPECIFIED",
            AutoLinkingOption::Username => "AUTO_LINKING_OPTION_USERNAME",
            AutoLinkingOption::Email => "AUTO_LINKING_OPTION_EMAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_LINKING_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTO_LINKING_OPTION_USERNAME" => Some(Self::Username),
            "AUTO_LINKING_OPTION_EMAIL" => Some(Self::Email),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AzureAdTenantType {
    Common = 0,
    Organisations = 1,
    Consumers = 2,
}
impl AzureAdTenantType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AzureAdTenantType::Common => "AZURE_AD_TENANT_TYPE_COMMON",
            AzureAdTenantType::Organisations => "AZURE_AD_TENANT_TYPE_ORGANISATIONS",
            AzureAdTenantType::Consumers => "AZURE_AD_TENANT_TYPE_CONSUMERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AZURE_AD_TENANT_TYPE_COMMON" => Some(Self::Common),
            "AZURE_AD_TENANT_TYPE_ORGANISATIONS" => Some(Self::Organisations),
            "AZURE_AD_TENANT_TYPE_CONSUMERS" => Some(Self::Consumers),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdpByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdpByIdResponse {
    #[prost(message, optional, tag="1")]
    pub idp: ::core::option::Option<Idp>,
}
include!("zitadel.idp.v2.tonic.rs");
// @@protoc_insertion_point(module)