// Copyright (c) 2013-2015 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! List of enums.

use traits::{FromPointerReader, FromPointerBuilder, ToU16, FromU16};
use private::layout::{ListReader, ListBuilder, PointerReader, PointerBuilder,
                      TwoBytes, PrimitiveElement};
use {NotInSchema, Result};

#[derive(Clone, Copy)]
pub struct Reader<'a, T> {
    marker : ::std::marker::PhantomData<T>,
    reader : ListReader<'a>
}

impl <'a, T : FromU16> Reader<'a, T> {
    pub fn new<'b>(reader : ListReader<'b>) -> Reader<'b, T> {
        Reader::<'b, T> { reader : reader, marker : ::std::marker::PhantomData }
    }

    pub fn len(&self) -> u32 { self.reader.len() }

}

impl <'a, T : FromU16> FromPointerReader<'a> for Reader<'a, T> {
    fn get_from_pointer(reader : &PointerReader<'a>) -> Result<Reader<'a, T>> {
        Ok(Reader { reader : try!(reader.get_list(TwoBytes, ::std::ptr::null())),
                    marker : ::std::marker::PhantomData })
    }
}

impl <'a, T : FromU16> Reader<'a, T> {
    pub fn get(&self, index : u32) -> ::std::result::Result<T, NotInSchema> {
        assert!(index < self.len());
        let result : u16 = PrimitiveElement::get(&self.reader, index);
        FromU16::from_u16(result)
    }
}

pub struct Builder<'a, T> {
    marker : ::std::marker::PhantomData<T>,
    builder : ListBuilder<'a>
}

impl <'a, T : ToU16 + FromU16> Builder<'a, T> {
    pub fn new(builder : ListBuilder<'a>) -> Builder<'a, T> {
        Builder { builder : builder, marker : ::std::marker::PhantomData }
    }

    pub fn len(&self) -> u32 { self.builder.len() }

    pub fn set(&mut self, index : u32, value : T) {
        assert!(index < self.len());
        PrimitiveElement::set(&self.builder, index, value.to_u16());
    }
}

impl <'a, T : FromU16> FromPointerBuilder<'a> for Builder<'a, T> {
    fn init_pointer(builder : PointerBuilder<'a>, size : u32) -> Builder<'a, T> {
        Builder { builder : builder.init_list(TwoBytes, size),
                  marker : ::std::marker::PhantomData }
    }
    fn get_from_pointer(builder : PointerBuilder<'a>) -> Result<Builder<'a, T>> {
        Ok(Builder { builder : try!(builder.get_list(TwoBytes, ::std::ptr::null())),
                     marker : ::std::marker::PhantomData })
    }
}

impl <'a, T : ToU16 + FromU16>  Builder<'a, T> {
    pub fn get(&self, index : u32) -> ::std::result::Result<T, NotInSchema> {
        assert!(index < self.len());
        let result : u16 = PrimitiveElement::get_from_builder(&self.builder, index);
        FromU16::from_u16(result)
    }
}

impl <'a, T> ::traits::SetPointerBuilder<Builder<'a, T>> for Reader<'a, T> {
    fn set_pointer_builder<'b>(pointer : ::private::layout::PointerBuilder<'b>,
                               value : Reader<'a, T>) -> Result<()> {
        pointer.set_list(&value.reader)
    }
}

impl <'a, 'b : 'a, T> ::traits::CastableTo<Builder<'a, T> > for Builder<'b, T> {
    fn cast(self) -> Builder<'a, T> {
        Builder { builder : self.builder, marker : ::std::marker::PhantomData }
    }
}

