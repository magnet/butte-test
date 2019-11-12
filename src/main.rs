/*
 * Copyright 2018 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// import the generated code
mod greeter_exps;
mod greeter_generated;
use bytes::{Bytes, BytesMut};
use greeter_exps::*;
use greeter_generated::foo::bar::{HelloReply as FbsHelloReply, HelloReplyArgs};
// Example how to use FlatBuffers to create and read binary buffers.

fn main() {
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

    let message = builder.create_string("Hello World!");

    let hello_reply = FbsHelloReply::create(&mut builder, &HelloReplyArgs { message });

    builder.finish(hello_reply, None);

    // We now have a FlatBuffer we can store on disk or send over a network.

    // ** file/network code goes here :) **

    // Instead, we're going to access it right away (as if we just received it).
    // This must be called after `finish()`.
    let buf = builder.collapse(); // (Vec<u8>, usize)
    let mut bm = BytesMut::from(buf.0);
    let bm = bm.split_off(buf.1);

    let bm = HelloReply::from(bm);

    // Get access to the root:

    // Get and test some scalar types from the FlatBuffer.
    let message = bm.message();

    assert_eq!(message, "Hello World!");

    println!("It worked");
}
