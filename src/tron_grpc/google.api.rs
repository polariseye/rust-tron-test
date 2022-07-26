/// Defines the HTTP configuration for a service. It contains a list of
/// \[HttpRule][google.api.HttpRule\], each specifying the mapping of an RPC method
/// to one or more HTTP REST API methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    /// A list of HTTP rules for configuring the HTTP REST API methods.
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<HttpRule>,
}
/// `HttpRule` defines the mapping of an RPC method to one or more HTTP
/// REST APIs.  The mapping determines what portions of the request
/// message are populated from the path, query parameters, or body of
/// the HTTP request.  The mapping is typically specified as an
/// `google.api.http` annotation, see "google/api/annotations.proto"
/// for details.
///
/// The mapping consists of a field specifying the path template and
/// method kind.  The path template can refer to fields in the request
/// message, as in the example below which describes a REST GET
/// operation on a resource collection of messages:
///
/// ```proto
/// service Messaging {
///   rpc GetMessage(GetMessageRequest) returns (Message) {
///     option (google.api.http).get = "/v1/messages/{message_id}/{sub.subfield}";
///   }
/// }
/// message GetMessageRequest {
///   message SubMessage {
///     string subfield = 1;
///   }
///   string message_id = 1; // mapped to the URL
///   SubMessage sub = 2;    // `sub.subfield` is url-mapped
/// }
/// message Message {
///   string text = 1; // content of the resource
/// }
/// ```
///
/// This definition enables an automatic, bidrectional mapping of HTTP
/// JSON to RPC. Example:
///
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456/foo`  | `GetMessage(message_id: "123456" sub: SubMessage(subfield: "foo"))`
///
/// In general, not only fields but also field paths can be referenced
/// from a path pattern. Fields mapped to the path pattern cannot be
/// repeated and must have a primitive (non-message) type.
///
/// Any fields in the request message which are not bound by the path
/// pattern automatically become (optional) HTTP query
/// parameters. Assume the following definition of the request message:
///
/// ```proto
/// message GetMessageRequest {
///   message SubMessage {
///     string subfield = 1;
///   }
///   string message_id = 1; // mapped to the URL
///   int64 revision = 2;    // becomes a parameter
///   SubMessage sub = 3;    // `sub.subfield` becomes a parameter
/// }
/// ```
///
/// This enables a HTTP JSON to RPC mapping as below:
///
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456?revision=2&sub.subfield=foo` | `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield: "foo"))`
///
/// Note that fields which are mapped to HTTP parameters must have a
/// primitive type or a repeated primitive type. Message types are not
/// allowed. In the case of a repeated type, the parameter can be
/// repeated in the URL, as in `...?param=A&param=B`.
///
/// For HTTP method kinds which allow a request body, the `body` field
/// specifies the mapping. Consider a REST update method on the
/// message resource collection:
///
/// ```proto
/// service Messaging {
///   rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
///     option (google.api.http) = {
///       put: "/v1/messages/{message_id}"
///       body: "message"
///     };
///   }
/// }
/// message UpdateMessageRequest {
///   string message_id = 1; // mapped to the URL
///   Message message = 2;   // mapped to the body
/// }
/// ```
///
/// The following HTTP JSON to RPC mapping is enabled, where the
/// representation of the JSON in the request body is determined by
/// protos JSON encoding:
///
/// HTTP | RPC
/// -----|-----
/// `PUT /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" message { text: "Hi!" })`
///
/// The special name `*` can be used in the body mapping to define that
/// every field not bound by the path template should be mapped to the
/// request body.  This enables the following alternative definition of
/// the update method:
///
/// ```proto
/// service Messaging {
///   rpc UpdateMessage(Message) returns (Message) {
///     option (google.api.http) = {
///       put: "/v1/messages/{message_id}"
///       body: "*"
///     };
///   }
/// }
/// message Message {
///   string message_id = 1;
///   string text = 2;
/// }
/// ```
///
/// The following HTTP JSON to RPC mapping is enabled:
///
/// HTTP | RPC
/// -----|-----
/// `PUT /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" text: "Hi!")`
///
/// Note that when using `*` in the body mapping, it is not possible to
/// have HTTP parameters, as all fields not bound by the path end in
/// the body. This makes this option more rarely used in practice of
/// defining REST APIs. The common usage of `*` is in custom methods
/// which don't use the URL at all for transferring data.
///
/// It is possible to define multiple HTTP methods for one RPC by using
/// the `additional_bindings` option. Example:
///
/// ```proto
/// service Messaging {
///   rpc GetMessage(GetMessageRequest) returns (Message) {
///     option (google.api.http) = {
///       get: "/v1/messages/{message_id}"
///       additional_bindings {
///         get: "/v1/users/{user_id}/messages/{message_id}"
///       }
///     };
///   }
/// }
/// message GetMessageRequest {
///   string message_id = 1;
///   string user_id = 2;
/// }
/// ```
///
/// This enables the following two alternative HTTP JSON to RPC
/// mappings:
///
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
/// `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id: "123456")`
///
/// # Rules for HTTP mapping
///
/// The rules for mapping HTTP path, query parameters, and body fields
/// to the request message are as follows:
///
/// 1. The `body` field specifies either `*` or a field path, or is
///    omitted. If omitted, it assumes there is no HTTP body.
/// 2. Leaf fields (recursive expansion of nested messages in the
///    request) can be classified into three types:
///     (a) Matched in the URL template.
///     (b) Covered by body (if body is `*`, everything except (a) fields;
///         else everything under the body field)
///     (c) All other fields.
/// 3. URL query parameters found in the HTTP request are mapped to (c) fields.
/// 4. Any body sent with an HTTP request can contain only (b) fields.
///
/// The syntax of the path template is as follows:
///
///     Template = "/" Segments [ Verb ] ;
///     Segments = Segment { "/" Segment } ;
///     Segment  = "*" | "**" | LITERAL | Variable ;
///     Variable = "{" FieldPath [ "=" Segments ] "}" ;
///     FieldPath = IDENT { "." IDENT } ;
///     Verb     = ":" LITERAL ;
///
/// The syntax `*` matches a single path segment. It follows the semantics of
/// [RFC 6570](<https://tools.ietf.org/html/rfc6570>) Section 3.2.2 Simple String
/// Expansion.
///
/// The syntax `**` matches zero or more path segments. It follows the semantics
/// of [RFC 6570](<https://tools.ietf.org/html/rfc6570>) Section 3.2.3 Reserved
/// Expansion.
///
/// The syntax `LITERAL` matches literal text in the URL path.
///
/// The syntax `Variable` matches the entire path as specified by its template;
/// this nested template must not contain further variables. If a variable
/// matches a single path segment, its template may be omitted, e.g. `{var}`
/// is equivalent to `{var=*}`.
///
/// NOTE: the field paths in variables and in the `body` must not refer to
/// repeated fields or map fields.
///
/// Use CustomHttpPattern to specify any HTTP method that is not included in the
/// `pattern` field, such as HEAD, or "*" to leave the HTTP method unspecified for
/// a given URL path rule. The wild-card rule is useful for services that provide
/// content to Web (HTML) clients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRule {
    /// Selects methods to which this rule applies.
    ///
    /// Refer to \[selector][google.api.DocumentationRule.selector\] for syntax details.
    #[prost(string, tag="1")]
    pub selector: ::prost::alloc::string::String,
    /// The name of the request field whose value is mapped to the HTTP body, or
    /// `*` for mapping all fields not captured by the path pattern to the HTTP
    /// body. NOTE: the referred field must not be a repeated field.
    #[prost(string, tag="7")]
    pub body: ::prost::alloc::string::String,
    /// Additional HTTP bindings for the selector. Nested bindings must
    /// not contain an `additional_bindings` field themselves (that is,
    /// the nesting may only be one level deep).
    #[prost(message, repeated, tag="11")]
    pub additional_bindings: ::prost::alloc::vec::Vec<HttpRule>,
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[prost(oneof="http_rule::Pattern", tags="2, 3, 4, 5, 6, 8")]
    pub pattern: ::core::option::Option<http_rule::Pattern>,
}
/// Nested message and enum types in `HttpRule`.
pub mod http_rule {
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pattern {
        /// Used for listing and getting information about resources.
        #[prost(string, tag="2")]
        Get(::prost::alloc::string::String),
        /// Used for updating a resource.
        #[prost(string, tag="3")]
        Put(::prost::alloc::string::String),
        /// Used for creating a resource.
        #[prost(string, tag="4")]
        Post(::prost::alloc::string::String),
        /// Used for deleting a resource.
        #[prost(string, tag="5")]
        Delete(::prost::alloc::string::String),
        /// Used for updating a resource.
        #[prost(string, tag="6")]
        Patch(::prost::alloc::string::String),
        /// Custom pattern is used for defining custom verbs.
        #[prost(message, tag="8")]
        Custom(super::CustomHttpPattern),
    }
}
/// A custom pattern is used for defining custom HTTP verb.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHttpPattern {
    /// The name of this custom HTTP verb.
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    /// The path matched by this custom verb.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
