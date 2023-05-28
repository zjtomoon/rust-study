# quick start

## step 1

``` bash

$ mkdir volo-example
$ cd volo-example
$ mkdir idl
$ vim idl/volo_example.proto


```

```bash


syntax = "proto3";
package volo.example;

message Item {
    int64 id = 1;
    string title = 2;
    string content = 3;

    map<string, string> extra = 10;
}

message GetItemRequest {
    int64 id = 1;
}

message GetItemResponse {
    Item item = 1;
}

service ItemService {
    rpc GetItem(GetItemRequest) returns (GetItemResponse);
}


```

## step 2

``` bash

     volo init --includes=idl volo-example idl/volo_example.proto

     volo init --includes=proto hello proto/hello.proto


# 如果只需要增加一个 IDL（如 client 的 IDL）而不需要生成模板的话，如：

     volo idl add idl/volo_example.proto

# volo 工具还支持从 git 下载 IDL 并生成代码

     volo idl add -g git@github.com:org/repo.git -r main /path/to/your/idl.proto


```

## step 3

编辑src/lib.rs

``` rust

#![feature(type_alias_impl_trait)]

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_grpc::Request<volo_gen::volo::example::GetItemRequest>,
    ) -> core::result::Result<volo_grpc::Response<volo_gen::volo::example::GetItemResponse>, volo_grpc::Status>
    {
        Ok(volo_grpc::Response::new(Default::default()))
    }
}


```

## step 4

``` bash

    cargo update

    cargo build

    cargo run --bin server

```