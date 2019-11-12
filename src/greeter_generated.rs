pub mod baz {
    pub mod buzz {
        #[repr(i32)]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum Foo {
            a = 0_i32,
            b = 1_i32,
            c = 2_i32,
        }
        impl<'a> flatbuffers::Follow<'a> for Foo {
            type Inner = Self;
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::read_scalar_at::<Self>(buf, loc)
            }
        }
        impl flatbuffers::EndianScalar for Foo {
            #[inline]
            fn to_little_endian(self) -> Self {
                let n = i32::to_le(self as i32);
                let p = &n as *const i32 as *const Self;
                unsafe { *p }
            }
            #[inline]
            fn from_little_endian(self) -> Self {
                let n = i32::from_le(self as i32);
                let p = &n as *const i32 as *const Self;
                unsafe { *p }
            }
        }
        impl flatbuffers::Push for Foo {
            type Output = Self;
            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                flatbuffers::emplace_scalar::<Self>(dst, *self);
            }
        }
        pub fn enum_name_foo(e: Foo) -> &'static str {
            match e {
                Foo::a => "a",
                Foo::b => "b",
                Foo::c => "c",
            }
        }
        /// A greeter service!
        /// ... with a multiline doc comment?!
        pub trait GreeterService {
            fn say_hello(
                request: super::super::foo::bar::HelloRequest,
            ) -> super::super::foo::bar::HelloReply;
            fn say_many_hellos(
                request: super::super::foo::bar::ManyHellosRequest,
            ) -> super::super::foo::bar::HelloReply;
        }
    }
}
/// A response from the server
/// Useful for getting a greeting back!
pub mod foo {
    pub mod bar {
        pub enum HelloReplyOffset {}
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct HelloReply<'a> {
            table: flatbuffers::Table<'a>,
        }
        impl<'a> From<flatbuffers::Table<'a>> for HelloReply<'a> {
            fn from(table: flatbuffers::Table<'a>) -> Self {
                Self { table }
            }
        }
        impl<'a> HelloReply<'a> {
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args HelloReplyArgs<'args>,
            ) -> flatbuffers::WIPOffset<HelloReply<'bldr>> {
                let mut builder = HelloReplyBuilder::new(fbb);
                builder.add_message(args.message);
                builder.finish()
            }
            pub const VT_MESSAGE: flatbuffers::VOffsetT = 4i16;

            #[inline]
            pub fn message(&self) -> Option<&'a str> {
                self.table
                    .get::<flatbuffers::ForwardsUOffset<&str>>(HelloReply::VT_MESSAGE, None)
            }

    
        }

        impl<'a> flatbuffers::Follow<'a> for HelloReply<'a> {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                let table = flatbuffers::Table { buf, loc };
                Self { table }
            }
        }
        pub struct HelloReplyArgs<'a> {
            pub message: flatbuffers::WIPOffset<&'a str>,
        }
        pub struct HelloReplyBuilder<'a, 'b> {
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> HelloReplyBuilder<'a, 'b> {
            #[inline]
            fn add_message(&mut self, message: flatbuffers::WIPOffset<&'b str>) {
                self.fbb
                    .push_slot_always::<flatbuffers::WIPOffset<&'_ str>>(
                        HelloReply::VT_MESSAGE,
                        message,
                    );
            }
            #[inline]
            pub fn new(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Self {
                let start = fbb.start_table();
                HelloReplyBuilder { fbb, start }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<HelloReply<'a>> {
                let o = self.fbb.end_table(self.start);
                self.fbb.required(o, HelloReply::VT_MESSAGE, "message");
                flatbuffers::WIPOffset::new(o.value())
            }
        }
        pub enum HelloRequestOffset {}
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct HelloRequest<'a> {
            table: flatbuffers::Table<'a>,
        }
        impl<'a> From<flatbuffers::Table<'a>> for HelloRequest<'a> {
            fn from(table: flatbuffers::Table<'a>) -> Self {
                Self { table }
            }
        }
        impl<'a> HelloRequest<'a> {
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args HelloRequestArgs<'args>,
            ) -> flatbuffers::WIPOffset<HelloRequest<'bldr>> {
                let mut builder = HelloRequestBuilder::new(fbb);
                builder.add_name(args.name);
                builder.finish()
            }
            pub const VT_NAME: flatbuffers::VOffsetT = 4i16;
        }
        impl<'a> flatbuffers::Follow<'a> for HelloRequest<'a> {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                let table = flatbuffers::Table { buf, loc };
                Self { table }
            }
        }
        pub struct HelloRequestArgs<'a> {
            pub name: flatbuffers::WIPOffset<&'a str>,
        }
        pub struct HelloRequestBuilder<'a, 'b> {
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> HelloRequestBuilder<'a, 'b> {
            #[inline]
            fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb
                    .push_slot_always::<flatbuffers::WIPOffset<&'_ str>>(
                        HelloRequest::VT_NAME,
                        name,
                    );
            }
            #[inline]
            pub fn new(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Self {
                let start = fbb.start_table();
                HelloRequestBuilder { fbb, start }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<HelloRequest<'a>> {
                let o = self.fbb.end_table(self.start);
                self.fbb.required(o, HelloRequest::VT_NAME, "name");
                flatbuffers::WIPOffset::new(o.value())
            }
        }
        pub enum ManyHellosRequestOffset {}
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct ManyHellosRequest<'a> {
            table: flatbuffers::Table<'a>,
        }
        impl<'a> From<flatbuffers::Table<'a>> for ManyHellosRequest<'a> {
            fn from(table: flatbuffers::Table<'a>) -> Self {
                Self { table }
            }
        }
        impl<'a> ManyHellosRequest<'a> {
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args ManyHellosRequestArgs<'args>,
            ) -> flatbuffers::WIPOffset<ManyHellosRequest<'bldr>> {
                let mut builder = ManyHellosRequestBuilder::new(fbb);
                builder.add_name(args.name);
                builder.add_num_greetings(args.num_greetings);
                builder.finish()
            }
            pub const VT_NAME: flatbuffers::VOffsetT = 4i16;
            pub const VT_NUM_GREETINGS: flatbuffers::VOffsetT = 6i16;
        }
        impl<'a> flatbuffers::Follow<'a> for ManyHellosRequest<'a> {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                let table = flatbuffers::Table { buf, loc };
                Self { table }
            }
        }
        pub struct ManyHellosRequestArgs<'a> {
            pub name: flatbuffers::WIPOffset<&'a str>,
            pub num_greetings: i32,
        }
        pub struct ManyHellosRequestBuilder<'a, 'b> {
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> ManyHellosRequestBuilder<'a, 'b> {
            #[inline]
            fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb
                    .push_slot_always::<flatbuffers::WIPOffset<&'_ str>>(
                        ManyHellosRequest::VT_NAME,
                        name,
                    );
            }
            #[inline]
            fn add_num_greetings(&mut self, num_greetings: i32) {
                self.fbb
                    .push_slot_always::<i32>(ManyHellosRequest::VT_NUM_GREETINGS, num_greetings);
            }
            #[inline]
            pub fn new(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Self {
                let start = fbb.start_table();
                ManyHellosRequestBuilder { fbb, start }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<ManyHellosRequest<'a>> {
                let o = self.fbb.end_table(self.start);
                self.fbb.required(o, ManyHellosRequest::VT_NAME, "name");
                self.fbb
                    .required(o, ManyHellosRequest::VT_NUM_GREETINGS, "num_greetings");
                flatbuffers::WIPOffset::new(o.value())
            }
        }
    }
}
