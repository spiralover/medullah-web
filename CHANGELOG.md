# Medullah Changelog
medullah-web changelog file 

## 0.20.7 (2024-09-25)
* feat(rabbitmq): "close" method to close connection
* feat(rabbitmq): acquire connection pool in use by this instance

## 0.20.6 (2024-09-25)
* bump(crates): to their respective latest versions

## 0.20.5 (2024-09-18)
* feat(tokio): added 'blk()' as an alias of 'spawn_blocking()'

## 0.20.4 (2024-09-18)
* feat(app-message): improve database conflict error logging

## 0.20.3 (2024-09-16)
* feat(string): uc_first and uc_words helper functions

## 0.20.2 (2024-09-16)
* bump(deadpool-redis): to version 0.17.0

## 0.20.1 (2024-09-16)
* feat(app-message): handle blocking error hand return proper error message

## 0.20.0 (2024-09-14)
* feat(struct-responder): renamed method "send_response" to "into_response"
* feat(struct-responder): renamed method "send_struct_result" to "respond"
* feat(struct-responder): introduced "respond_msg"

## 0.19.4 (2024-09-08)
* feat(rabbitmq): return &mut Self for 'nack_on_failure' & 'requeue_on_failure'
* feat(rabbitmq): added 'execute_handler_asynchronously()' to alter state

## 0.19.3 (2024-09-08)
* fix(rabbitmq): separate exchange & routing key generic params to avoid first param influencing the second

## 0.19.2 (2024-09-08)
* feat(rabbitmq): routing key params now accepts vars that implements ToString

## 0.19.1 (2024-09-07)
* feat(reqwest): ability to directly deserialize error response

## 0.19.0 (2024-09-07)
* refactor(rabbitmq): recreate closed channel
* feat(rabbitmq): global state is now wrapped around Arc<Mutex<>>
* fix(rabbitmq): execute_handler_asynchronously not being set correctly
* refactor(rabbitmq): renamed "set_nack_on_failure" to "nack_on_failure"
* feat(rabbitmq): add "requeue_on_failure" method

## 0.18.2 (2024-09-05)
* feat(rabbitmq): use tag to provide more context in error log

## 0.18.1 (2024-09-05)
* feat(app-message): improve database error logging

## 0.18.0 (2024-09-05)
* feat(tokio): add name to tick & timeout for easier debugging
* feat(app-message): add error log to error kind error message
* refactor(app-message): renamed InvalidUUID variant to UuidError(_)
* feat(app-message): impl From<uuid::Error> for AppMessage

## 0.17.3 (2024-09-02)
* bump: to latest crates versions

## 0.17.2 (2024-09-02)
* feat(rabbitmq): expose option structs

## 0.17.1 (2024-09-02)
* feat(rabbitmq): added "ack_opt()" & "nack_opt()" to ack & nack with option

## 0.17.0 (2024-09-02)
* refactor(rabbitmq): use message wrapper instead of passing Delivery directly
* feat(app-message): impl From<std::str::Utf8Error>
* feat(app-message): improve readability
* feat(app-message): hide get_status_code()
* refactor(form-helper): removed 'get_nullable_time', 'get_nullable_uuid', 'get_uuid_from_string'

## 0.16.0 (2024-08-27)
* refactor(middleware): reworked logic & drop the usage of 'dyn_clone'

## 0.15.3 (2024-08-20)
* fix(database): doesn't require sha2 to work

## 0.15.2 (2024-08-20)
* fix(helpers): carefully decouple 
* fix(mailer): scope import to feature

## 0.15.1 (2024-08-20)
* fix(helpers): jwt helper should be scoped to feat-jwt

## 0.15.0 (2024-08-20)
* feat(jwt): introduced new feat-jwt feature to help decouple jwt from crypto dependencies
* fix(jwt): send customized error message for middleware-level error
* fix(json): renamed 'json()' to 'deserialize()'
* feat(json): can now collect serde_json::Value from extractor
* fix(json): deserialize() now accepts ref of itself instead of moving to itself
* test(json): added unit tests
* fix(hmac): now accepts value as borrowed values

## 0.14.8 (2024-08-20)
* fix(json): JsonBody.raw() now return &String

## 0.14.7 (2024-08-20)
* fix(responder): 'message' should be flexible enough to decide weather message is success/failure
* fix(app-message): return 401 on jwt decoding failure

## 0.14.6 (2024-08-20)
* feat(http): added separate serializer & deserializer
* feat(app-state): added 'helpers()' helper func to easily acquire struct of helpers

## 0.14.5 (2024-08-18)
* bump: upgraded packages to their latest versions

## 0.14.4 (2024-08-17)
* fix(jwt): use 'auth_iss_public_key' to verify jwt signatrue
* fix(workflow): merge qa & release workflows

## 0.14.3 (2024-08-17)
* feat(app-message): handle database conflict

## 0.14.2 (2024-08-17)
* fix(cache): add more debug info
* fix(redis): avoid unwrapping serde result
* fix(mailer): config setup should be scoped to "feat-mailer"

## 0.14.1 (2024-08-16)
* fix(nerve): removed partially implemented feature

## 0.14.0 (2024-08-14)
* refactor(http): ntex should not be a feature as we build around it

## 0.13.1 (2024-08-14)
* refactor(http): ntex should not be a feature as we build around it

## 0.13.0 (2024-08-14)
* refactor(database): feature is now optional
* fix(readme): broken changelog link
* refactor(id-generator): removed feature
* refactor(redis + cache): scope feature to "feat-redis"

## 0.12.0 (2024-08-14)
* fix(reqwest): scope feature to "feat-reqwest"
* refactor(responder): 'respond_map' now has http status-like names
* test(responder): unit test mappable
* test(responder): unit test .respond() & .respond_msg()

## 0.11.0 (2024-08-13)
* feat(helpers): added "jwt" to global accessor
* refactor(base64): converted to assoc members and added tests
* refactor(password): now collect salt in constructor
* refactor(password): avoid unwrapping and return error
* feat(app-message): support argon error

## 0.10.0 (2024-08-13)
* refactor(helpers): renamed "security" to "jwt"
* refactor(hmac): convert funcs to associate funcs
* refactor(hmac): return error instead of unwrapping result
* test(test): unit test for hmac & currency
* refactor(helpers): separate password & string helper
* feat(string): str-based uuid generation helper
* refactor(mailer): scope feature to "feat-mailer"
* test(jwt): added unit test

## 0.9.0 (2024-08-11)
* refactor(services): convert rabbitmq & redis to normal classes
* refactor(helpers): separate password & string helper
* fix(rabbitmq): 'consume_detached' params no longer have 'static lifetime
* feat(rabbitmq): ability to specify whether to auto-requeue on failure
* fix(rabbitmq): avoid expensive clone in iteration

## 0.8.4 (2024-08-11)
* feat(rabbitmq): auto-execute handler in the background (asynchronously).
* fix(rabbitmq): lifetime issues related to "creates a temporary value which is freed while still in use"

## 0.8.2 (2024-08-11)
* refactor(app-message): remove unused "make_response" func which is clearly a duplicate of "send_response"
* chore(deps): bump openssl from 0.10.64 to 0.10.66
* feat(rabbitmq): ability to auto-nack on failure

## 0.8.0 (2024-08-05)
* feat(pre-exec-middleware): handle and return proper response
* refactor(middleware): pass request & response as borrowed instances

## 0.7.0 (2024-08-04)
* refactor(responder): replaced funcs with associated funcs

## 0.6.0 (2024-08-04)
* refactor(bootstrap): remove unnecessary usage of "Rc" and also usages of clones
* feat(thread-boot): closure to be called during thread boot-up, this closure will return vector of app routes

## 0.5.0 (2024-08-02)
* refactor(database): removed unnecessary database variants from AppMessage
* feat(http): support Forbidden variant
* bump(crates): to latest versions
* chore(changelog): added this file

