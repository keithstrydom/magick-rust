/*
 * Copyright 2016 Mattis Marjak
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
use super::bindings;

impl From<bindings::MagickBooleanType> for bool {
    fn from(value: bindings::MagickBooleanType) -> Self {
        match value {
            bindings::MagickBooleanType::MagickFalse => false,
            bindings::MagickBooleanType::MagickTrue => true,
        }
    }
}

impl From<bool> for bindings::MagickBooleanType {
    fn from(value: bool) -> Self {
        match value {
            true => bindings::MagickBooleanType::MagickTrue,
            false => bindings::MagickBooleanType::MagickFalse,
        }
    }
}
