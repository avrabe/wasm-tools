/* Copyright 2019 Mozilla Foundation
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

use crate::types::CoreTypeId;
use crate::{
    BinaryReaderError, FuncType, GlobalType, HeapType, MemoryType, RefType, TableType, ValType,
    WasmFeatures,
};
use std::ops::Range;

/// Types that qualify as Wasm function types for validation purposes.
pub trait WasmFuncType: Clone {
    /// Returns the number of input types.
    fn len_inputs(&self) -> usize;

    /// Returns the number of output types.
    fn len_outputs(&self) -> usize;

    /// Returns the type at given index if any.
    ///
    /// The type must be canonicalized.
    ///
    /// # Note
    ///
    /// The returned type may be wrapped by the user crate and thus
    /// the actually returned type only has to be comparable to a Wasm type.
    fn input_at(&self, at: u32) -> Option<ValType>;

    /// Returns the type at given index if any.
    ///
    /// The type must be canonicalized.
    ///
    /// # Note
    ///
    /// The returned type may be wrapped by the user crate and thus
    /// the actually returned type only has to be comparable to a Wasm type.
    fn output_at(&self, at: u32) -> Option<ValType>;

    /// Returns the list of inputs as an iterator.
    fn inputs(&self) -> WasmFuncTypeInputs<'_, Self>
    where
        Self: Sized,
    {
        let range = 0..self.len_inputs() as u32;
        WasmFuncTypeInputs {
            func_type: self,
            range,
        }
    }

    /// Returns the list of outputs as an iterator.
    fn outputs(&self) -> WasmFuncTypeOutputs<'_, Self>
    where
        Self: Sized,
    {
        let range = 0..self.len_outputs() as u32;
        WasmFuncTypeOutputs {
            func_type: self,
            range,
        }
    }
}

impl<T> WasmFuncType for &'_ T
where
    T: ?Sized + WasmFuncType,
{
    fn len_inputs(&self) -> usize {
        T::len_inputs(self)
    }
    fn len_outputs(&self) -> usize {
        T::len_outputs(self)
    }
    fn input_at(&self, at: u32) -> Option<ValType> {
        T::input_at(self, at)
    }
    fn output_at(&self, at: u32) -> Option<ValType> {
        T::output_at(self, at)
    }
}

/// Iterator over the inputs of a Wasm function type.
#[derive(Clone)]
pub struct WasmFuncTypeInputs<'a, T> {
    /// The iterated-over function type.
    func_type: &'a T,
    /// The range we're iterating over.
    range: Range<u32>,
}

impl<T> Iterator for WasmFuncTypeInputs<'_, T>
where
    T: WasmFuncType,
{
    type Item = crate::ValType;

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|i| self.func_type.input_at(i).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T> DoubleEndedIterator for WasmFuncTypeInputs<'_, T>
where
    T: WasmFuncType,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|i| self.func_type.input_at(i).unwrap())
    }
}

impl<T> ExactSizeIterator for WasmFuncTypeInputs<'_, T>
where
    T: WasmFuncType,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}

/// Iterator over the outputs of a Wasm function type.
#[derive(Clone)]
pub struct WasmFuncTypeOutputs<'a, T> {
    /// The iterated-over function type.
    func_type: &'a T,
    /// The range we're iterating over.
    range: Range<u32>,
}

impl<T> Iterator for WasmFuncTypeOutputs<'_, T>
where
    T: WasmFuncType,
{
    type Item = crate::ValType;

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|i| self.func_type.output_at(i).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T> DoubleEndedIterator for WasmFuncTypeOutputs<'_, T>
where
    T: WasmFuncType,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|i| self.func_type.output_at(i).unwrap())
    }
}

impl<T> ExactSizeIterator for WasmFuncTypeOutputs<'_, T>
where
    T: WasmFuncType,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}

/// Types that qualify as Wasm validation database.
///
/// # Note
///
/// The `wasmparser` crate provides a builtin validation framework but allows
/// users of this crate to also feed the parsed Wasm into their own data
/// structure while parsing and also validate at the same time without
/// the need of an additional parsing or validation step or copying data around.
pub trait WasmModuleResources {
    /// The function type used for validation.
    type FuncType: WasmFuncType;

    /// Returns the table at given index if any.
    ///
    /// The table element type must be canonicalized.
    fn table_at(&self, at: u32) -> Option<TableType>;

    /// Returns the linear memory at given index.
    fn memory_at(&self, at: u32) -> Option<MemoryType>;

    /// Returns the tag at given index.
    ///
    /// The tag's function type must be canonicalized.
    fn tag_at(&self, at: u32) -> Option<&Self::FuncType>;

    /// Returns the global variable at given index.
    ///
    /// The global's value type must be canonicalized.
    fn global_at(&self, at: u32) -> Option<GlobalType>;

    /// Returns the `FuncType` associated with the given type index.
    ///
    /// The function type must be canonicalized.
    fn func_type_at(&self, type_idx: u32) -> Option<&Self::FuncType>;

    /// Returns the type id associated with the given function
    /// index.
    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId>;

    /// Returns the `FuncType` associated with the given function index.
    ///
    /// The function type must be canonicalized.
    fn type_of_function(&self, func_idx: u32) -> Option<&Self::FuncType>;

    /// Returns the element type at the given index.
    ///
    /// The `RefType` must be canonicalized.
    fn element_type_at(&self, at: u32) -> Option<RefType>;

    /// Is `a` a subtype of `b`?
    fn is_subtype(&self, a: ValType, b: ValType) -> bool;

    /// Check and canonicalize a value type.
    ///
    /// This will validate that `t` is valid under the `features` provided and
    /// then additionally validate the structure of `t`. For example any type
    /// references that `t` makes are validated and canonicalized.
    fn check_value_type(
        &self,
        t: &mut ValType,
        features: &WasmFeatures,
        offset: usize,
    ) -> Result<(), BinaryReaderError> {
        features
            .check_value_type(*t)
            .map_err(|s| BinaryReaderError::new(s, offset))?;
        match t {
            ValType::Ref(r) => {
                let nullable = r.is_nullable();
                let mut hty = r.heap_type();
                self.check_heap_type(&mut hty, offset)?;
                *r = RefType::new(nullable, hty).unwrap();
                Ok(())
            }
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => Ok(()),
        }
    }

    /// Checks that a `HeapType` is valid and then additionally place it in its
    /// canonical form.
    ///
    /// Similar to `check_value_type` but for heap types.
    fn check_heap_type(
        &self,
        heap_type: &mut HeapType,
        offset: usize,
    ) -> Result<(), BinaryReaderError>;

    /// Returns the number of elements.
    fn element_count(&self) -> u32;

    /// Returns the number of bytes in the Wasm data section.
    fn data_count(&self) -> Option<u32>;

    /// Returns whether the function index is referenced in the module anywhere
    /// outside of the start/function sections.
    fn is_function_referenced(&self, idx: u32) -> bool;
}

impl<T> WasmModuleResources for &'_ T
where
    T: ?Sized + WasmModuleResources,
{
    type FuncType = T::FuncType;

    fn table_at(&self, at: u32) -> Option<TableType> {
        T::table_at(self, at)
    }
    fn memory_at(&self, at: u32) -> Option<MemoryType> {
        T::memory_at(self, at)
    }
    fn tag_at(&self, at: u32) -> Option<&Self::FuncType> {
        T::tag_at(self, at)
    }
    fn global_at(&self, at: u32) -> Option<GlobalType> {
        T::global_at(self, at)
    }
    fn func_type_at(&self, at: u32) -> Option<&Self::FuncType> {
        T::func_type_at(self, at)
    }
    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId> {
        T::type_id_of_function(self, func_idx)
    }
    fn type_of_function(&self, func_idx: u32) -> Option<&Self::FuncType> {
        T::type_of_function(self, func_idx)
    }
    fn check_heap_type(&self, t: &mut HeapType, offset: usize) -> Result<(), BinaryReaderError> {
        T::check_heap_type(self, t, offset)
    }
    fn element_type_at(&self, at: u32) -> Option<RefType> {
        T::element_type_at(self, at)
    }
    fn is_subtype(&self, a: ValType, b: ValType) -> bool {
        T::is_subtype(self, a, b)
    }

    fn element_count(&self) -> u32 {
        T::element_count(self)
    }
    fn data_count(&self) -> Option<u32> {
        T::data_count(self)
    }
    fn is_function_referenced(&self, idx: u32) -> bool {
        T::is_function_referenced(self, idx)
    }
}

impl<T> WasmModuleResources for std::sync::Arc<T>
where
    T: WasmModuleResources,
{
    type FuncType = T::FuncType;

    fn table_at(&self, at: u32) -> Option<TableType> {
        T::table_at(self, at)
    }

    fn memory_at(&self, at: u32) -> Option<MemoryType> {
        T::memory_at(self, at)
    }

    fn tag_at(&self, at: u32) -> Option<&Self::FuncType> {
        T::tag_at(self, at)
    }

    fn global_at(&self, at: u32) -> Option<GlobalType> {
        T::global_at(self, at)
    }

    fn func_type_at(&self, type_idx: u32) -> Option<&Self::FuncType> {
        T::func_type_at(self, type_idx)
    }

    fn type_id_of_function(&self, func_idx: u32) -> Option<CoreTypeId> {
        T::type_id_of_function(self, func_idx)
    }

    fn type_of_function(&self, func_idx: u32) -> Option<&Self::FuncType> {
        T::type_of_function(self, func_idx)
    }

    fn check_heap_type(&self, t: &mut HeapType, offset: usize) -> Result<(), BinaryReaderError> {
        T::check_heap_type(self, t, offset)
    }

    fn element_type_at(&self, at: u32) -> Option<RefType> {
        T::element_type_at(self, at)
    }

    fn is_subtype(&self, a: ValType, b: ValType) -> bool {
        T::is_subtype(self, a, b)
    }

    fn element_count(&self) -> u32 {
        T::element_count(self)
    }

    fn data_count(&self) -> Option<u32> {
        T::data_count(self)
    }

    fn is_function_referenced(&self, idx: u32) -> bool {
        T::is_function_referenced(self, idx)
    }
}

impl WasmFuncType for FuncType {
    fn len_inputs(&self) -> usize {
        self.params().len()
    }

    fn len_outputs(&self) -> usize {
        self.results().len()
    }

    fn input_at(&self, at: u32) -> Option<ValType> {
        self.params().get(at as usize).copied()
    }

    fn output_at(&self, at: u32) -> Option<ValType> {
        self.results().get(at as usize).copied()
    }
}
