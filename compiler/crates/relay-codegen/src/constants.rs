/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use interner::{Intern, StringKey};
use lazy_static::lazy_static;

pub struct CodegenConstants {
    pub alias: StringKey,
    pub args: StringKey,
    pub argument_definitions: StringKey,
    pub client_extension: StringKey,
    pub concrete_type: StringKey,
    pub condition_value: StringKey,
    pub condition: StringKey,
    pub connection: StringKey,
    pub count: StringKey,
    pub cursor: StringKey,
    pub default_value: StringKey,
    pub defer: StringKey,
    pub derived_from: StringKey,
    pub direction: StringKey,
    pub dynamic_key_argument: StringKey,
    pub dynamic_key: StringKey,
    pub fields: StringKey,
    pub filters: StringKey,
    pub fragment_spread: StringKey,
    pub fragment_value: StringKey,
    pub fragment: StringKey,
    pub handle: StringKey,
    pub id: StringKey,
    pub if_: StringKey,
    pub inline_fragment: StringKey,
    pub items: StringKey,
    pub key: StringKey,
    pub kind: StringKey,
    pub label: StringKey,
    pub linked_field: StringKey,
    pub linked_handle: StringKey,
    pub list_value: StringKey,
    pub literal: StringKey,
    pub local_argument: StringKey,
    pub mask: StringKey,
    pub metadata: StringKey,
    pub mutation: StringKey,
    pub name: StringKey,
    pub object_value: StringKey,
    pub operation_kind: StringKey,
    pub operation_value: StringKey,
    pub operation: StringKey,
    pub params: StringKey,
    pub passing_value: StringKey,
    pub path: StringKey,
    pub plural: StringKey,
    pub query: StringKey,
    pub request: StringKey,
    pub root_argument: StringKey,
    pub scalar_field: StringKey,
    pub scalar_handle: StringKey,
    pub selections: StringKey,
    pub split_operation: StringKey,
    pub storage_key: StringKey,
    pub stream: StringKey,
    pub subscription: StringKey,
    pub text: StringKey,
    pub type_: StringKey,
    pub use_customized_batch: StringKey,
    pub value: StringKey,
    pub variable_name: StringKey,
    pub variable: StringKey,
}

impl Default for CodegenConstants {
    fn default() -> Self {
        Self {
            alias: "alias".intern(),
            args: "args".intern(),
            argument_definitions: "argumentDefinitions".intern(),
            client_extension: "ClientExtension".intern(),
            concrete_type: "concreteType".intern(),
            condition_value: "Condition".intern(),
            condition: "condition".intern(),
            connection: "connection".intern(),
            count: "count".intern(),
            cursor: "cursor".intern(),
            default_value: "defaultValue".intern(),
            defer: "Defer".intern(),
            derived_from: "derivedFrom".intern(),
            direction: "direction".intern(),
            dynamic_key_argument: "__dynamicKey".intern(),
            dynamic_key: "dynamicKey".intern(),
            fields: "fields".intern(),
            filters: "filters".intern(),
            fragment_spread: "FragmentSpread".intern(),
            fragment_value: "Fragment".intern(),
            fragment: "fragment".intern(),
            handle: "handle".intern(),
            id: "id".intern(),
            if_: "if".intern(),
            inline_fragment: "InlineFragment".intern(),
            items: "items".intern(),
            key: "key".intern(),
            kind: "kind".intern(),
            label: "label".intern(),
            linked_field: "LinkedField".intern(),
            linked_handle: "LinkedHandle".intern(),
            list_value: "ListValue".intern(),
            literal: "Literal".intern(),
            local_argument: "LocalArgument".intern(),
            mask: "mask".intern(),
            metadata: "metadata".intern(),
            mutation: "mutation".intern(),
            name: "name".intern(),
            object_value: "ObjectValue".intern(),
            operation_kind: "operationKind".intern(),
            operation_value: "Operation".intern(),
            operation: "operation".intern(),
            params: "params".intern(),
            passing_value: "passingValue".intern(),
            path: "path".intern(),
            plural: "plural".intern(),
            query: "query".intern(),
            request: "Request".intern(),
            root_argument: "RootArgument".intern(),
            scalar_field: "ScalarField".intern(),
            scalar_handle: "ScalarHandle".intern(),
            selections: "selections".intern(),
            split_operation: "SplitOperation".intern(),
            storage_key: "storageKey".intern(),
            stream: "Stream".intern(),
            subscription: "subscription".intern(),
            text: "text".intern(),
            type_: "type".intern(),
            use_customized_batch: "useCustomizedBatch".intern(),
            value: "value".intern(),
            variable_name: "variableName".intern(),
            variable: "Variable".intern(),
        }
    }
}

lazy_static! {
    pub static ref CODEGEN_CONSTANTS: CodegenConstants = Default::default();
}
