# Medullah Changelog
medullah-web changelog file 

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

