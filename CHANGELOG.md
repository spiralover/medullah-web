# Medullah Changelog
medullah-web changelog file 

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

