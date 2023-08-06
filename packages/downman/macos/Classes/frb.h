#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire___record__String_String {
  struct wire_uint_8_list *field0;
  struct wire_uint_8_list *field1;
} wire___record__String_String;

typedef struct wire_list___record__String_String {
  struct wire___record__String_String *ptr;
  int32_t len;
} wire_list___record__String_String;

typedef struct wire_BaseConfig {
  struct wire_uint_8_list *base_url;
  struct wire_list___record__String_String *headers;
  uintptr_t *timeout_sec;
} wire_BaseConfig;

typedef struct wire_HttpClient {
  const void *ptr;
} wire_HttpClient;

typedef struct wire_Config {
  struct wire_list___record__String_String *headers;
  uintptr_t *timeout_sec;
} wire_Config;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

WireSyncReturn wire_http_client_new(struct wire_BaseConfig *config);

void wire____why_http_client(int64_t port_);

void wire_http_client_get(int64_t port_,
                          struct wire_HttpClient client,
                          struct wire_uint_8_list *url,
                          struct wire_Config *config);

void wire_http_client_post(int64_t port_,
                           struct wire_HttpClient client,
                           struct wire_uint_8_list *url,
                           struct wire_uint_8_list *body,
                           struct wire_Config *config);

void wire_http_client_put(int64_t port_,
                          struct wire_HttpClient client,
                          struct wire_uint_8_list *url,
                          struct wire_uint_8_list *body,
                          struct wire_Config *config);

void wire_http_client_patch(int64_t port_,
                            struct wire_HttpClient client,
                            struct wire_uint_8_list *url,
                            struct wire_uint_8_list *body,
                            struct wire_Config *config);

void wire_http_client_delete(int64_t port_,
                             struct wire_HttpClient client,
                             struct wire_uint_8_list *url,
                             struct wire_Config *config);

void wire_http_client_options(int64_t port_,
                              struct wire_HttpClient client,
                              struct wire_uint_8_list *url,
                              struct wire_Config *config);

void wire_http_client_head(int64_t port_,
                           struct wire_HttpClient client,
                           struct wire_uint_8_list *url,
                           struct wire_Config *config);

struct wire_HttpClient new_HttpClient(void);

struct wire_BaseConfig *new_box_autoadd_base_config_0(void);

struct wire_Config *new_box_autoadd_config_0(void);

uintptr_t *new_box_autoadd_usize_0(uintptr_t value);

struct wire_list___record__String_String *new_list___record__String_String_0(int32_t len);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_HttpClient(const void *ptr);

const void *share_opaque_HttpClient(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_http_client_new);
    dummy_var ^= ((int64_t) (void*) wire____why_http_client);
    dummy_var ^= ((int64_t) (void*) wire_http_client_get);
    dummy_var ^= ((int64_t) (void*) wire_http_client_post);
    dummy_var ^= ((int64_t) (void*) wire_http_client_put);
    dummy_var ^= ((int64_t) (void*) wire_http_client_patch);
    dummy_var ^= ((int64_t) (void*) wire_http_client_delete);
    dummy_var ^= ((int64_t) (void*) wire_http_client_options);
    dummy_var ^= ((int64_t) (void*) wire_http_client_head);
    dummy_var ^= ((int64_t) (void*) new_HttpClient);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_base_config_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_config_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_usize_0);
    dummy_var ^= ((int64_t) (void*) new_list___record__String_String_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_HttpClient);
    dummy_var ^= ((int64_t) (void*) share_opaque_HttpClient);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
